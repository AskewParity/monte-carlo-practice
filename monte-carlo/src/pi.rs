use rand::Rng;

const RADIUS: f64 = 1.0;

fn is_in_circle(x: f64, y: f64) -> bool {
    x.powi(2) + y.powi(2) <= RADIUS * RADIUS
}

pub fn calculate_pi(points: u32) -> f64 {
    let mut rng = rand::thread_rng();
    let mut in_circle: u32 = 0; 
    
    for _ in 0..points {
        let x = rng.gen_range(0.0..RADIUS);
        let y = rng.gen_range(0.0..RADIUS);

        if is_in_circle(x, y) {
            in_circle += 1;
        }
    }

    4.0 * (in_circle as f64) / (points as f64)
}
