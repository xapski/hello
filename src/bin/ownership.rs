struct Book {
    pages: i32,
    rating: i32,
}

struct LineItem {
    name: String, // an owned string
    count: i32,
}

struct Person {
    name: String,
    fav_col: String,
    age: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?} ", book.pages);
}

fn display_rating(book: &Book) {
    println!("pages = {:?}", book.rating);
}

// this func asks for a borrowed str
fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn print_data(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let book = Book {
        pages: 34,
        rating: 5,
    };

    display_page_count(&book); // borrows "book" so that display rating can also use it
    display_rating(&book);

    let receipt = vec![
        LineItem {
            name: String::from("cereal"), // convert from borrowed to owned
            count: 5,
        },
        LineItem {
            name: "fajitas".to_owned(),
            count: 6,
        },
    ];

    for item in receipt {
        print_name(&item.name); // convert from owned to borrowed
        println!("count: {:?}", item.count);
    }

    let people = vec![
        Person {
            name: "George".to_owned(),
            fav_col: "Blue".to_owned(),
            age: 10,
        },
        Person {
            name: "Mike".to_owned(),
            fav_col: "Red".to_owned(),
            age: 23,
        },
        Person {
            name: "Richard".to_owned(),
            fav_col: "Orange".to_owned(),
            age: 18,
        },
    ];

    for person in people {
        if person.age > 10 {
            print_data(&person.name);
            print_data(&person.fav_col);
        }
    }
}
