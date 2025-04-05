fn main() {
    const RADIUS: f32 = 5.0;

    let area = std::f32::consts::PI * RADIUS.powi(2);

    println!("The area of a circle with RADIUS {RADIUS:.2} is {area:.5}");
}