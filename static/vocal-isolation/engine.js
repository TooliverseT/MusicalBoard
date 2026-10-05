/* Vocal Isolation engine (main thread side).
 *
 * window.MBVocalIsolation.separate({ left, right, onProgress }) -> Promise<{ left, right }>
 *   left/right: Float32Array at 44.1 kHz. Returns the separated vocal, same length.
 *   onProgress(ev): { stage: 'download', loaded, total }
 *                   { stage: 'prepare', done, total }
 *                   { stage: 'separate', done, total, elapsedMs }
 * window.MBVocalIsolation.cancel()       stops the running job (rejects with 'cancelled').
 * window.MBVocalIsolation.workerCount(chunks) how many workers a job of `chunks` chunks uses.
 * window.MBVocalIsolation.chunkCount(samples) chunks needed for `samples` samples.
 * window.MBVocalIsolation.modelCached()  Promise<boolean>.
 *
 * GitHub Pages cannot send COOP/COEP (and AdSense does not run cross-origin isolated),
 * so there is no SharedArrayBuffer. Instead each worker owns a single-threaded session
 * and the chunks are spread over the pool.
 */
(function () {
  'use strict';

  const BASE = '/static/vocal-isolation/';
  const MODEL_URL = BASE + 'models/mdx_b_vocals.onnx';
  const MODEL_SHA256 = '9b7dcb9d878acb0f3e64ff3fd27750faae96577013f6d50f5996875bf4250713';
  const MODEL_BYTES = 29703204;
  const CACHE_NAME = 'mb-vocal-isolation-v1';

  const HOP = 1024;
  const CHUNK = HOP * 255;
  const TRIM = 3072;
  const GEN = CHUNK - 2 * TRIM;
  const MAX_WORKERS = 4;
  const IDLE_TERMINATE_MS = 60000;

  let pool = [];
  let idleTimer = null;
  let current = null;
  let modelPromise = null;

  function hex(buf) {
    return Array.from(new Uint8Array(buf), (b) => b.toString(16).padStart(2, '0')).join('');
  }

  async function verify(bytes) {
    if (!(self.crypto && crypto.subtle)) return true;
    return hex(await crypto.subtle.digest('SHA-256', bytes)) === MODEL_SHA256;
  }

  async function openCache() {
    try {
      return self.caches ? await caches.open(CACHE_NAME) : null;
    } catch (_) {
      return null;
    }
  }

  async function modelCached() {
    const cache = await openCache();
    return !!(cache && (await cache.match(MODEL_URL)));
  }

  async function downloadModel(onProgress) {
    const cache = await openCache();
    if (cache) {
      const hit = await cache.match(MODEL_URL);
      if (hit) {
        const bytes = await hit.arrayBuffer();
        if (await verify(bytes)) return bytes;
        await cache.delete(MODEL_URL);
      }
    }
    const resp = await fetch(MODEL_URL);
    if (!resp.ok) throw new Error('model-download-failed');
    const total = Number(resp.headers.get('Content-Length')) || MODEL_BYTES;
    const reader = resp.body.getReader();
    const parts = [];
    let loaded = 0;
    onProgress({ stage: 'download', loaded, total });
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      parts.push(value);
      loaded += value.length;
      onProgress({ stage: 'download', loaded: Math.min(loaded, total), total });
    }
    const bytes = new Uint8Array(loaded);
    let off = 0;
    for (const p of parts) {
      bytes.set(p, off);
      off += p.length;
    }
    if (!(await verify(bytes.buffer))) throw new Error('model-checksum-mismatch');
    if (cache) {
      try {
        await cache.put(MODEL_URL, new Response(bytes.slice().buffer, {
          headers: { 'Content-Type': 'application/octet-stream' },
        }));
      } catch (_) {
        // Quota: keep going without the cache.
      }
    }
    return bytes.buffer;
  }

  function loadModel(onProgress) {
    if (!modelPromise) {
      modelPromise = downloadModel(onProgress).catch((e) => {
        modelPromise = null;
        throw e;
      });
    }
    return modelPromise;
  }

  function chunkCount(samples) {
    return Math.max(1, Math.ceil((samples + 1) / GEN));
  }

  function workerCount(chunks) {
    const ua = navigator.userAgent || '';
    if (/Mobi|Android|iPhone|iPad|iPod/i.test(ua)) return 1;
    const cores = navigator.hardwareConcurrency || 2;
    const mem = navigator.deviceMemory || 8;
    const n = Math.min(MAX_WORKERS, cores - 1, Math.floor(mem / 2));
    return Math.max(1, Math.min(n, chunks || MAX_WORKERS));
  }

  function terminatePool() {
    for (const w of pool) w.worker.terminate();
    pool = [];
  }

  function scheduleIdleTerminate() {
    clearTimeout(idleTimer);
    idleTimer = setTimeout(() => {
      if (!current) terminatePool();
    }, IDLE_TERMINATE_MS);
  }

  function spawnWorker(model) {
    return new Promise((resolve, reject) => {
      const worker = new Worker(BASE + 'worker.js');
      worker.onmessage = (e) => {
        if (e.data.type === 'ready') resolve({ worker, busy: false });
        else if (e.data.type === 'error') {
          worker.terminate();
          reject(new Error(e.data.message));
        }
      };
      worker.onerror = (e) => {
        worker.terminate();
        reject(new Error((e && e.message) || 'worker-failed'));
      };
      worker.postMessage({ type: 'init', model: model.slice(0) });
    });
  }

  async function ensurePool(model, wanted, onPrepare) {
    while (pool.length > wanted) pool.pop().worker.terminate();
    let done = pool.length;
    onPrepare(done, wanted);
    while (pool.length < wanted) {
      try {
        pool.push(await spawnWorker(model));
      } catch (e) {
        if (pool.length > 0) break;
        throw e;
      }
      done += 1;
      onPrepare(done, wanted);
    }
  }

  function paddedChunk(src, index, length) {
    // Same layout as kuielab demix_base: TRIM zeros, signal, zeros up to the last chunk.
    const out = new Float32Array(CHUNK);
    const start = index * GEN - TRIM;
    const from = Math.max(0, start);
    const to = Math.min(length, start + CHUNK);
    if (to > from) out.set(src.subarray(from, to), from - start);
    return out;
  }

  async function separate({ left, right, onProgress }) {
    if (current) throw new Error('busy');
    const report = onProgress || function () {};
    const length = left.length;
    const total = chunkCount(length);
    const job = { id: Math.random(), cancelled: false, reject: null, abort: null };
    const aborted = new Promise((_, reject) => { job.abort = reject; });
    aborted.catch(() => {});
    // Terminated workers never answer, so every wait must also end on cancel.
    const guard = (p) => Promise.race([p, aborted]);
    current = job;
    clearTimeout(idleTimer);
    try {
      const model = await guard(loadModel(report));
      await guard(ensurePool(model, workerCount(total), (done, all) => {
        if (!job.cancelled) report({ stage: 'prepare', done, total: all });
      }));

      const outL = new Float32Array(length);
      const outR = new Float32Array(length);
      const started = performance.now();
      let next = 0;
      let done = 0;
      report({ stage: 'separate', done, total, elapsedMs: 0 });

      await guard(new Promise((resolve, reject) => {
        job.reject = reject;
        const dispatch = (slot) => {
          if (job.cancelled) return;
          if (next >= total) {
            slot.busy = false;
            return;
          }
          const index = next++;
          slot.busy = true;
          const l = paddedChunk(left, index, length);
          const r = paddedChunk(right, index, length);
          slot.worker.postMessage({ type: 'run', job: job.id, index, left: l, right: r }, [l.buffer, r.buffer]);
        };
        for (const slot of pool) {
          slot.worker.onmessage = (e) => {
            const msg = e.data;
            if (msg.job !== job.id || job.cancelled) return;
            if (msg.type === 'error') {
              reject(new Error(msg.message));
              return;
            }
            const at = msg.index * GEN;
            const n = Math.min(GEN, length - at);
            if (n > 0) {
              outL.set(msg.left.subarray(0, n), at);
              outR.set(msg.right.subarray(0, n), at);
            }
            done += 1;
            report({ stage: 'separate', done, total, elapsedMs: performance.now() - started });
            if (done === total) resolve();
            else dispatch(slot);
          };
          slot.worker.onerror = (e) => reject(new Error((e && e.message) || 'worker-failed'));
          dispatch(slot);
        }
      }));
      return { left: outL, right: outR };
    } catch (e) {
      if (job.cancelled) throw new Error('cancelled');
      terminatePool();
      throw e;
    } finally {
      if (current === job) current = null;
      scheduleIdleTerminate();
    }
  }

  function cancel() {
    const job = current;
    if (!job) return;
    job.cancelled = true;
    // Workers mid-chunk cannot be interrupted; drop them so the CPU is free at once.
    terminatePool();
    job.abort(new Error('cancelled'));
  }

  self.MBVocalIsolation = { separate, cancel, workerCount, chunkCount, modelCached, MODEL_BYTES };
})();
