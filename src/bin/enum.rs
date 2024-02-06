enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        // using _ means to ignore whatever is being matched i.e. n, this is not okay if
        // we still plan to use n
        // so we name any other variable not captured by match "other"
        other => println!("number : {:?}", other),
    }

    let flat = Discount::Flat(8);
    match flat {
        Discount::Flat(2) => println!("flat 2"),
        // here we not only match the flat variant of the discount enum but also its integer property
        // we named it amount and for any value of amount we perform a print action
        // KIM this is the equivalent of using _ and is necessary to capture all
        // possble values of the integer property of flat
        Discount::Flat(amount) => println!("flat discount amount of {:?}", amount),
        // the _ helps us ignore the percent variant
        // the return value () is equivalent to null
        _ => (),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };
    match concert {
        // here we ignore the value of event but use it regardless
        Ticket { price: 50, event } => {
            println!("{:?} event for fifty dollars", event)
        }
        // unlike an enum, to ignore other properties of a struct we use ..
        Ticket { price: 40, .. } => println!("price = forty dollars"),
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}
