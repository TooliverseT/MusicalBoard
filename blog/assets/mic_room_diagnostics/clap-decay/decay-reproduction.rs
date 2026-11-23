fn compute_rt60(samples: &[(f64, f64)]) -> f64 {
    if samples.len() < 5 { return 0.0; }

    // Find peak (highest RMS energy)
    let peak_idx = samples.iter().enumerate()
        .max_by(|(_, a), (_, b)| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0);

    let peak_db = samples[peak_idx].1;
    let peak_t  = samples[peak_idx].0;
    let after   = &samples[peak_idx..];

    // T20: find -5 dB and -25 dB crossing points after peak; RT60 = T20 × 3
    let t_5  = after.iter().find(|(_, db)| *db <= peak_db - 5.0).map(|(t, _)| t - peak_t);
    let t_25 = after.iter().find(|(_, db)| *db <= peak_db - 25.0).map(|(t, _)| t - peak_t);

    if let (Some(t5), Some(t25)) = (t_5, t_25) {
        let t20 = (t25 - t5).max(0.0);
        (t20 * 3.0).clamp(0.0, 8000.0)
    } else if let Some(last) = after.last() {
        // Fallback: linear extrapolation
        let drop    = (peak_db - last.1).max(1.0);
        let elapsed = (last.0  - peak_t).max(1.0);
        (elapsed * 60.0 / drop).clamp(0.0, 8000.0)
    } else {
        0.0
    }
}

fn main() {
    // CSV input: case_id,time_ms,rms_db, grouped by case_id.
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut id = String::new();
    let mut samples = Vec::new();
    for line in input.lines().skip(1) {
        let cells: Vec<&str> = line.split(',').collect();
        assert_eq!(cells.len(), 3);
        if !id.is_empty() && cells[0] != id {
            println!("{},{}", id, compute_rt60(&samples));
            samples.clear();
        }
        id = cells[0].to_string();
        let time: f64 = cells[1].parse().unwrap();
        let db: f64 = cells[2].parse().unwrap();
        assert!(time.is_finite() && db.is_finite());
        samples.push((time, db));
    }
    if !id.is_empty() { println!("{},{}", id, compute_rt60(&samples)); }
}
