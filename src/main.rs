mod model;
use model::person::Person;

fn main() {
    let helen = Person {
        lastname:   String::from("Helen"),
        surname:    String::from("Glenn"),
    };

    println!("{}", helen);
}