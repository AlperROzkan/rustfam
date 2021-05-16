mod model;
use model::person::Person;
use model::family::CloseFamily;

fn main() {
    let helen = Person {
        lastname:   String::from("Helen"),
        surname:    String::from("Glenn"),
    };

    let george = Person {
        lastname:   String::from("George"),
        surname:    String::from("Pierce"),
    };

    let rose = Person {
        lastname:   String::from("Rose"),
        surname:    String::from("Pierce"),
    };

    let fam1 = CloseFamily {
        parents:    vec![george, helen],
        children:   vec![rose],
    };

    println!("{}", fam1);
    
}