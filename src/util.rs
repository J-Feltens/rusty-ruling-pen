pub fn interpolate1d(mut start: f64, mut end: f64, factor: f64) -> f64 {
    assert!(factor >= 0.0 && factor <= 1.0, "Cannot clamp yet");

    if start > end {
        (end, start) = (start, end);
    }

    let delta = end - start;
    return start + factor * delta;
}
