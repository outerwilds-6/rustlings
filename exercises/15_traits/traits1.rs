// Define the AppendBar trait with one method that appends "Bar" to a String.
trait AppendBar {
    fn append_bar(self) -> Self;
}

// Implement the AppendBar trait for String.
impl AppendBar for String {
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");  // Append "Bar" to the string
        self  // Return the modified string
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();  // Append "Bar" to "Foo"
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
