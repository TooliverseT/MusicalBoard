/* Vocal Isolation worker: one single-threaded onnxruntime-web session per worker.
 * The page runs several of these in parallel (see engine.js); each one turns one
 * stereo chunk into the separated vocal for the same span.
 *
 * Model: KUIELab-MDX-Net (Leaderboard B) vocals, CC-BY 4.0.
 * Input/output: [1, 4, 2048, 256] = [L.re, L.im, R.re, R.im] x freq x frame,
 * STFT n_fft 6144, hop 1024, periodic Hann, center/reflect padding (torch.stft).
 */
'use strict';

const ORT_BASE = '/static/vocal-isolation/ort/';
importScripts(ORT_BASE + 'ort.wasm.min.js');
ort.env.wasm.numThreads = 1;
ort.env.wasm.wasmPaths = ORT_BASE;

const N_FFT = 6144;
const HALF = N_FFT / 2;
const HOP = 1024;
const DIM_F = 2048;
const DIM_T = 256;
const CHUNK = HOP * (DIM_T - 1);
const TRIM = N_FFT / 2;
const GEN = CHUNK - 2 * TRIM;
const SUB = N_FFT / 3;

const WINDOW = new Float32Array(N_FFT);
for (let i = 0; i < N_FFT; i++) WINDOW[i] = 0.5 - 0.5 * Math.cos((2 * Math.PI * i) / N_FFT);

// ── FFT of size 6144 = 3 x 2048 (one decimation-in-time radix-3 step over radix-2) ──

const LOG2_SUB = Math.log2(SUB);
const BITREV = new Uint32Array(SUB);
for (let i = 0; i < SUB; i++) {
  let r = 0;
  for (let b = 0; b < LOG2_SUB; b++) r |= ((i >> b) & 1) << (LOG2_SUB - 1 - b);
  BITREV[i] = r;
}
const SUB_COS = new Float64Array(SUB / 2);
const SUB_SIN = new Float64Array(SUB / 2);
for (let i = 0; i < SUB / 2; i++) {
  SUB_COS[i] = Math.cos((-2 * Math.PI * i) / SUB);
  SUB_SIN[i] = Math.sin((-2 * Math.PI * i) / SUB);
}
const TW_COS = new Float64Array(N_FFT);
const TW_SIN = new Float64Array(N_FFT);
for (let k = 0; k < N_FFT; k++) {
  TW_COS[k] = Math.cos((-2 * Math.PI * k) / N_FFT);
  TW_SIN[k] = Math.sin((-2 * Math.PI * k) / N_FFT);
}

function fftRadix2(re, im) {
  for (let i = 0; i < SUB; i++) {
    const j = BITREV[i];
    if (j > i) {
      let t = re[i]; re[i] = re[j]; re[j] = t;
      t = im[i]; im[i] = im[j]; im[j] = t;
    }
  }
  for (let size = 2; size <= SUB; size <<= 1) {
    const half = size >> 1;
    const step = SUB / size;
    for (let start = 0; start < SUB; start += size) {
      for (let k = 0; k < half; k++) {
        const wr = SUB_COS[k * step];
        const wi = SUB_SIN[k * step];
        const a = start + k;
        const b = a + half;
        const xr = re[b] * wr - im[b] * wi;
        const xi = re[b] * wi + im[b] * wr;
        re[b] = re[a] - xr; im[b] = im[a] - xi;
        re[a] += xr; im[a] += xi;
      }
    }
  }
}

const subRe = [new Float64Array(SUB), new Float64Array(SUB), new Float64Array(SUB)];
const subIm = [new Float64Array(SUB), new Float64Array(SUB), new Float64Array(SUB)];

/** In-place forward DFT of length N_FFT. */
function fft6144(re, im) {
  for (let r = 0; r < 3; r++) {
    const sr = subRe[r];
    const si = subIm[r];
    for (let n = 0; n < SUB; n++) {
      sr[n] = re[3 * n + r];
      si[n] = im[3 * n + r];
    }
    fftRadix2(sr, si);
  }
  for (let k = 0; k < N_FFT; k++) {
    const m = k % SUB;
    let accRe = subRe[0][m];
    let accIm = subIm[0][m];
    for (let r = 1; r < 3; r++) {
      const t = (r * k) % N_FFT;
      const wr = TW_COS[t];
      const wi = TW_SIN[t];
      const ar = subRe[r][m];
      const ai = subIm[r][m];
      accRe += ar * wr - ai * wi;
      accIm += ar * wi + ai * wr;
    }
    re[k] = accRe;
    im[k] = accIm;
  }
}

const bufRe = new Float64Array(N_FFT);
const bufIm = new Float64Array(N_FFT);

function reflectIndex(i, n) {
  if (i < 0) return -i;
  if (i >= n) return 2 * (n - 1) - i;
  return i;
}

/** Writes the STFT of one channel into `out` at channel offset `c0` (re) / `c0 + 1` (im). */
function stftChannel(x, out, c0) {
  const planeRe = c0 * DIM_F * DIM_T;
  const planeIm = (c0 + 1) * DIM_F * DIM_T;
  for (let t = 0; t < DIM_T; t++) {
    const start = t * HOP - HALF;
    for (let i = 0; i < N_FFT; i++) {
      bufRe[i] = x[reflectIndex(start + i, CHUNK)] * WINDOW[i];
      bufIm[i] = 0;
    }
    fft6144(bufRe, bufIm);
    for (let f = 0; f < DIM_F; f++) {
      out[planeRe + f * DIM_T + t] = bufRe[f];
      out[planeIm + f * DIM_T + t] = bufIm[f];
    }
  }
}

/** Inverse STFT of one channel; returns the GEN samples left after trimming both ends. */
function istftChannel(spec, c0) {
  const planeRe = c0 * DIM_F * DIM_T;
  const planeIm = (c0 + 1) * DIM_F * DIM_T;
  const total = (DIM_T - 1) * HOP + N_FFT;
  const acc = new Float64Array(total);
  const norm = new Float64Array(total);
  for (let t = 0; t < DIM_T; t++) {
    bufRe.fill(0);
    bufIm.fill(0);
    for (let f = 0; f < DIM_F; f++) {
      const r = spec[planeRe + f * DIM_T + t];
      const i = spec[planeIm + f * DIM_T + t];
      // Inverse via conj(FFT(conj(X))) / N, with Hermitian symmetry for a real signal.
      bufRe[f] = r;
      bufIm[f] = -i;
      if (f > 0) {
        bufRe[N_FFT - f] = r;
        bufIm[N_FFT - f] = i;
      }
    }
    fft6144(bufRe, bufIm);
    const off = t * HOP;
    for (let n = 0; n < N_FFT; n++) {
      const w = WINDOW[n];
      acc[off + n] += (bufRe[n] / N_FFT) * w;
      norm[off + n] += w * w;
    }
  }
  const out = new Float32Array(GEN);
  const base = HALF + TRIM;
  for (let n = 0; n < GEN; n++) {
    const d = norm[base + n];
    out[n] = d > 1e-11 ? acc[base + n] / d : 0;
  }
  return out;
}

let session = null;

async function init(modelBytes) {
  session = await ort.InferenceSession.create(new Uint8Array(modelBytes), {
    executionProviders: ['wasm'],
    enableCpuMemArena: false,
    enableMemPattern: false,
  });
}

async function runChunk(left, right) {
  const input = new Float32Array(4 * DIM_F * DIM_T);
  stftChannel(left, input, 0);
  stftChannel(right, input, 2);
  const result = await session.run({ input: new ort.Tensor('float32', input, [1, 4, DIM_F, DIM_T]) });
  const spec = result.output.data;
  return [istftChannel(spec, 0), istftChannel(spec, 2)];
}

onmessage = async (e) => {
  const msg = e.data;
  try {
    if (msg.type === 'init') {
      await init(msg.model);
      postMessage({ type: 'ready' });
    } else if (msg.type === 'run') {
      const t0 = performance.now();
      const [left, right] = await runChunk(msg.left, msg.right);
      postMessage(
        { type: 'result', job: msg.job, index: msg.index, left, right, ms: performance.now() - t0 },
        [left.buffer, right.buffer],
      );
    }
  } catch (err) {
    postMessage({ type: 'error', job: msg.job, message: String((err && err.message) || err) });
  }
};
