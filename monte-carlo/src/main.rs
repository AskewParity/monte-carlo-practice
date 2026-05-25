mod pi;

#[cfg(test)]
mod tests;

const POINTS: u32 = 2_u32.pow(20);

fn main() {
    let pi_approx = pi::calculate_pi(POINTS);
    println!("Pi is approximately: {pi_approx}");
}

