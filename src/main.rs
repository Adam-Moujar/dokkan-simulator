#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
struct Unit {
    name: String,
}

impl Unit {
    fn load_goku() -> Unit {
        Unit {
            name: "UR PHY GOKU".to_string(),
        }
    }

    fn print_unit(&self) {
        println!("-------------------------");
        println!("Unit name is: {:?}", self.name);
    }
}

fn main() {
    let goku = Unit::load_goku();
    goku.print_unit();
}
