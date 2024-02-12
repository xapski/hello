struct Temperature {
    degrees_f: f64,
}
// an implementation is like an object declaration in OOP
impl Temperature {
    // this method creates an instanc eof the temperature stuct
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }

    // fn boiling() -> Self {
    //     Self { degrees_f: 100.0 }
    // }

    // this method prints out the degrees_f property
    fn show_temp(&self) {
        // see that the fn uses a borrwed ref of the Temperature struct
        println!("{:?} degrees F", self.degrees_f)
    }
}

enum Color {
    Brown,
    Red,
}
impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth; {:?}", self.depth);
    }
}
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}
impl ShippingBox {
    // to make a new shipping box
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?} ", self.weight)
    }
}

fn main() {
    let hot = Temperature { degrees_f: 65.0 };
    hot.show_temp();

    // to create a Temperature struct with degrees_f of 32.0
    let cold = Temperature::freezing();
    cold.show_temp();
    cold.show_temp();

    let small_dimensions = Dimensions {
        width: 1.6,
        height: 2.0,
        depth: 2.0,
    };

    let small_box = ShippingBox::new(3.0, Color::Red, small_dimensions);
    small_box.print();
}
