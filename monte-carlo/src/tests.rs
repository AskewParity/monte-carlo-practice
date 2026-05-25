use crate::pi;
use std::f64::consts::PI;

#[test]
fn test_pi_accuracy() {
    let points = 2_u32.pow(20);
    let estimated_pi = pi::calculate_pi(points);
    let expected = PI;
    
    let error = (estimated_pi - expected).abs();
    
    println!("Estimated Pi: {}", estimated_pi);
    println!("Actual Pi: {}", expected);
    println!("Error: {}", error);
    
    assert!(error < 0.01, "Error is too large: {error}");
}
