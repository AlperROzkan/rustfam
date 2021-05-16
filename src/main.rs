mod model;
use model::family::CloseFamily;
use model::person::Person;

fn main() {
    /*
     * People declaration
     */
    let helen: Person = Person::new("Helen", "Glenn");
    let george: Person = Person::new("George", "Pierce");
    let rose: Person = Person::new("Tom", "Pierce");
    let tom = Person::new("Tom", "Pierce");

    /*
     * Family manipulation
     */
    let mut fam1: CloseFamily = CloseFamily::new("Glenn-Pierce");
    fam1.add_parent(helen);
    fam1.add_parent(george);
    fam1.add_child(rose);
    fam1.add_child(tom);

    println!("{}", fam1);
}
