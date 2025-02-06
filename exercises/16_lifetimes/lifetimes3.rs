// Adding lifetime 'a to indicate that both references in Book
// will live as long as the struct itself.
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let author = "George Orwell";
    let title = "1984";

    // Now we can create the Book with references that live long enough
    let book = Book {
        author,  // author reference
        title,   // title reference
    };

    println!("{} by {}", book.title, book.author);
}
