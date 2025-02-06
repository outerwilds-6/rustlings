#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // Define `shared_numbers` by using `Arc` to allow multiple threads to safely access it.
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // Define `child_numbers` using `shared_numbers`.
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            // Filter and sum every eighth number based on the offset.
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    // Join all threads to ensure they finish before the main thread exits.
    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
