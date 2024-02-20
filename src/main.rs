enum Publication {
    Book(Book),
    Magazine(Magazine),
}

// Book struct 
struct Book {
    title: String,
    author: String,
    pageCount: u32,
}

// Magazine struct 
struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

//Vec<Publication> array
fn create_publications() -> Vec<Publication> {
    let book1 = Book {
        title: String::from("Act of Valor"),
        author: String::from("Sun Tzu"),
        pageCount: 300,
    };

    let magazine1 = Magazine {
        title: String::from("level"),
        issue: 42,
        topic: String::from("gaming"),
    };

    let book2 = Book {
        title: String::from("C++ Primer"),
        author: String::from("Stanley B. Lippman"),
        pageCount: 250,
    };

    let magazine2 = Magazine {
        title: String::from("Science Magazine"),
        issue: 18,
        topic: String::from("Artificial Intelligence"),
    };

    vec![Publication::Book(book1), Publication::Magazine(magazine1), Publication::Book(book2), Publication::Magazine(magazine2)]
}

// func which prints according to types of publications
fn print_publications(publications: &Vec<Publication>) {
    for pub_item in publications {
        match pub_item {
            Publication::Book(book) => {
                println!("Book: {} Author: {}, {} Page", book.title, book.author, book.pageCount);
            }
            Publication::Magazine(magazine) => {
                println!("Magazine: {} - İssue: {}, Topic: {}", magazine.title, magazine.issue, magazine.topic);
            }
        }
    }
}
fn main() {
    
    let publications = create_publications();

    
    print_publications(&publications);
} 
