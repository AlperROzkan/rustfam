mod model;
use model::family::CloseFamily;
use model::person::Person;

fn main() {
    /*
     * People declaration
     */
    let mut fam1: CloseFamily = CloseFamily::new("Glenn-Pierce");
    let mut fam2: CloseFamily = CloseFamily::new("Trent-Pierce");
    let helen: Person;
    let george: Person;
    let rose: Person;
    let tom: Person;
    let rosa: Person;
    let gregor: Person;

    unsafe {
        helen = Person::new("Helen", "Glenn");
        george = Person::new("George", "Pierce");
        rose = Person::new("Tom", "Pierce");
        tom = Person::new("Tom", "Pierce");
        rosa = Person::new("Rosa", "Trent");
        gregor = Person::new("Gregor", "Pierce");
    }
    fam1.add_parent(helen);
    fam1.add_parent(george);
    fam1.add_child(rose);
    fam1.add_child(tom);

    fam2.add_parent(rosa);
    fam2.add_parent(gregor);
    fam2.add_child(george);
    

    println!("{}", fam1);
}
