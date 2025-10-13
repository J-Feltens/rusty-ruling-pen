pub fn map_range(
    value: f64,
    range_from_min: f64,
    range_from_max: f64,
    range_to_min: f64,
    range_to_max: f64,
) -> f64 {
    let output = range_to_min
        + ((range_to_max - range_from_min) / (range_from_max - range_from_min))
            * (value - range_from_min);
    return output;
}
