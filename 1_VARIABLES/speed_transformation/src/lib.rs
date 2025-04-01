// converts speed from km/h (kilometers per hour) and returns m/s (meters per second).
pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    km_h / 3.6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_km_per_hour_to_meters_per_second() {
        let km_h = 36.0;
        let expected = 10.0; // 36 km/h equals 10 m/s
        let result = km_per_hour_to_meters_per_second(km_h);
        // Using an epsilon for floating point comparison
        let epsilon = 1e-10;
        assert!((result - expected).abs() < epsilon, "Expected {}, got {}", expected, result);
    }
}
