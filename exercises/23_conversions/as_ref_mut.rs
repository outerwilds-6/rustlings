// Obtain the number of bytes (not characters) in the given argument
// `.len()` returns the number of bytes in a string.
fn byte_counter<T>(arg: T) -> usize 
where
    T: AsRef<str>, // Add AsRef trait bound to convert to &str
{
    arg.as_ref().len()
}

// Obtain the number of characters (not bytes) in the given argument.
fn char_counter<T>(arg: T) -> usize 
where
    T: AsRef<str>, // Add AsRef trait bound to convert to &str
{
    arg.as_ref().chars().count()
}

// Squares a number using `as_mut()`.
// T needs to implement AsMut to allow mutable access to the value.
fn num_sq<T>(arg: &mut T)
where
    T: AsMut<u32>, // Allow mutable references to u32
{
    // Dereference the value, square it, and update the original value
    let val = arg.as_mut();
    *val = (*val) * (*val);
}

fn main() {
    // You can optionally experiment here.
    let s = "Café au lait";
    println!("Byte count: {}", byte_counter(s));
    println!("Character count: {}", char_counter(s));
    
    let mut num: Box<u32> = Box::new(3);
    num_sq(&mut num);
    println!("Squared number: {}", *num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
