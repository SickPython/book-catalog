use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) -> io::Result<()> {
    
    let mut file = File::create(filename)?;

    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year)?;
    }

    Ok(())
}

fn load_books(filename: &str) -> io::Result<Vec<Book>> {

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year: u16 = parts[2].parse().unwrap_or(0);
            books.push(Book { title, author, year });
        }
    }

    Ok(books)
}

fn main() -> io::Result<()> {
    
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt")?;
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt")?;
    println!("Loaded books:");

    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }

    Ok(())
}