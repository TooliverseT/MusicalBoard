const MAX_PITCH_PEAKS_PER_SLICE: usize = 3;

// multi-frequency 분석 함수 추가
fn analyze_multiple_frequencies(
    buffer: &[f32],
    sample_rate: f64,
    sensitivity: f32,
    peak_threshold: f32,
) -> Vec<(f64, f32)> {
    // A0~C8 범위로 확장 (A0: 27.5Hz, C8: 4186Hz)
    const MIN_FREQ: f64 = 27.5; // A0 주파수
    const MAX_FREQ: f64 = 4186.0; // C8 주파수
    let peak_threshold = peak_threshold.clamp(0.12, 0.98);
    const ABSOLUTE_MIN_FREQ: f64 = 20.0; // 검출 가능한 절대 최소 주파수 (A0보다 약간 낮게)
    const ABSOLUTE_MAX_FREQ: f64 = 5000.0; // 검출 가능한 절대 최대 주파수 (C8보다 약간 높게)

    let rms = (buffer.iter().map(|&x| x * x).sum::<f32>() / buffer.len() as f32).sqrt();
    if rms < sensitivity {
        return Vec::new();
    }

    // 검출 가능한 절대 범위로 lag 범위 계산
    let absolute_min_lag = (sample_rate / ABSOLUTE_MAX_FREQ).max(1.0) as usize;
    let absolute_max_lag = (sample_rate / ABSOLUTE_MIN_FREQ) as usize;

    // 버퍼 길이보다 큰 lag는 계산할 수 없으므로 제한
    let absolute_max_lag = absolute_max_lag.min(buffer.len() - 1);

    // min_lag가 max_lag보다 크면 값을 교체하여 오류 방지
    let (absolute_min_lag, absolute_max_lag) = if absolute_min_lag > absolute_max_lag {
        (1, absolute_min_lag.min(buffer.len() - 1))
    } else {
        (absolute_min_lag, absolute_max_lag)
    };

    // 상관관계 계산 범위는 넓게 잡되, 유효 주파수 판정은 MIN_FREQ~MAX_FREQ로 제한
    let target_min_lag = (sample_rate / MAX_FREQ) as usize;
    let target_max_lag = (sample_rate / MIN_FREQ) as usize;

    // 모든 lag에 대한 상관관계 계산 (넓은 범위)
    let mut correlations = Vec::with_capacity(absolute_max_lag + 1);
    correlations.push(0.0); // 0 lag 값

    for lag in 1..=absolute_max_lag {
        let mut sum = 0.0;
        for i in 0..(buffer.len() - lag) {
            sum += buffer[i] * buffer[i + lag];
        }
        correlations.push(sum);
    }

    // 모든 lag에 대한 상관관계 값 중 최댓값 찾기
    let max_corr = if absolute_min_lag < absolute_max_lag {
        *correlations
            .iter()
            .skip(absolute_min_lag)
            .take(absolute_max_lag - absolute_min_lag)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0)
    } else {
        // min_lag가 max_lag보다 크거나 같은 경우
        0.0
    };

    // 임계값 설정 (모바일 등 저신호에서는 peak_threshold를 낮춰 피크 검출 완화)
    let threshold = max_corr * peak_threshold;

    // 피크 찾기 (전체 범위에서)
    let mut peaks = Vec::new();
    for lag in absolute_min_lag..=absolute_max_lag {
        let corr = correlations[lag];

        // 주변 값보다 큰지 확인 (피크 찾기)
        if corr > threshold
            && (lag <= absolute_min_lag + 1 || corr > correlations[lag - 1])
            && (lag >= absolute_max_lag - 1 || corr > correlations[lag + 1])
        {
            let frequency = sample_rate / lag as f64;

            // 주파수가 범위를 벗어나면 명확히 표시
            let amplitude = (corr / max_corr) as f32;

            if frequency >= MIN_FREQ && frequency <= MAX_FREQ {
                // 정상 범위 주파수는 그대로 추가
                peaks.push((frequency, amplitude));
            } else {
                // 범위 밖 주파수는 특별히 표시 (진폭에 0.5 곱하기)
                // 이는 UI에서 범위 밖 주파수를 표시하되 약하게 표시하는데 사용할 수 있음
                peaks.push((frequency, amplitude * 0.5));
            }
        }
    }

    // 진폭 기준 내림차순 정렬 후 상위 N개만 history에 보관
    peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    peaks.truncate(MAX_PITCH_PEAKS_PER_SLICE);

    peaks
}


fn main() {
    let bytes = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();
    // Our generator writes canonical 44-byte-header mono PCM16 WAVs only.
    assert_eq!(&bytes[36..40], b"data");
    let mono: Vec<f32> = bytes[44..].chunks_exact(2)
        .map(|b| i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0).collect();
    for start in (0..=mono.len()-2048).step_by(1200) {
        let peaks = analyze_multiple_frequencies(&mono[start..start+2048], 48000.0, 0.00008, 0.12);
        print!("{}", start);
        for (freq, corr) in peaks { print!("\t{:.9}\t{:.9}", freq, corr); }
        println!();
    }
}
