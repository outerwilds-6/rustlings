// Define the AppendBar trait with one method that appends "Bar" to a vector of strings.
trait AppendBar {
    fn append_bar(self) -> Self;
}

// Implement the AppendBar trait for Vec<String>.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());  // Append "Bar" to the vector
        self  // Return the modified vector
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
