struct Temperature {
    degrees_f: f64,
}

// an implementation is like an object declaration in OOP
impl Temperature {
    // this method creates an instanc eof the temperature stuct
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }

    // this method prints out the degrees_f property
    fn show_temp(&self) {
        // see that the fn uses a borrwed ref of the Temperature struct
        println!("{:?} degrees F", self.degrees_f)
    }
}
fn main() {
    let hot = Temperature { degrees_f: 65.0 };
    hot.show_temp();

    // to create a Temperature struct with degrees_f of 32.0
    let cold = Temperature::freezing();
    cold.show_temp();
    cold.show_temp();
}
