mod macros {
    // Define the macro inside the `macros` module
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    // Use the macro from the `macros` module
    my_macro!();
}
