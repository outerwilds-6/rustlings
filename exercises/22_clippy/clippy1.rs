fn main() {
    // Use the constant value for PI directly from the standard library
    let pi: f32 = std::f32::consts::PI;  // Use the built-in f32 constant

    let radius: f32 = 5.0;
    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
