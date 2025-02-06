fn main() {
    // Annotate the vector type as Vec<i32> to hold both u8 and i8 values
    let mut numbers: Vec<i32> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());  // `into()` converts `u8` to `i32`
    let n2: i8 = -1;
    numbers.push(n2.into());  // `into()` converts `i8` to `i32`

    println!("{numbers:?}");
}
