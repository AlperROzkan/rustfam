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

    let rosa: Person = Person::new("Rosa", "Trent");
    let gregor: Person = Person::new("Gregor", "Pierce");

    

    /*
     * Family manipulation
     */
    let mut fam1: CloseFamily = CloseFamily::new("Glenn-Pierce");
    fam1.add_parent(helen);
    fam1.add_parent(george);
    fam1.add_child(rose);
    fam1.add_child(tom);

    let mut fam2: CloseFamily = CloseFamily::new("Trent-Pierce");
    fam2.add_parent(rosa);
    fam2.add_parent(gregor);
    fam2.add_child(george);

    println!("{}", fam1);
}
