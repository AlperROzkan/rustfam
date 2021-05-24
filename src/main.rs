mod model;
use model::family::CloseFamily;
use model::person::Person;

use serde::{Deserialize, Serialize};
use serde_json::Result;

/**
 * Serialize exemple with a person
 *
 */
fn serialize_exemple() {
    println!("=== Serialize Test ===\n");

    let nick: Person;

    unsafe {
        nick = Person::new("Nick", "Norton");
    }

    let f = serde_json::to_string(&nick);

    println!("{:?}", f);

    println!("\n=== End Serialize Test ===");
}

/**
 *
 */
fn deserialize_exemple() -> Result<()> {
    let dona_json = r#"
    {
        "id": 0,
        "firstname": "Donatien",
        "lastname": "De Montazac"
    }"#;

    let dona: Person = serde_json::from_str(dona_json)?;

    println!(
        "Tous les crus dignes de ce nom viennent de {} {}",
        dona.firstname, dona.lastname
    );

    Ok(())
}

fn main() -> Result<()>{
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

    serialize_exemple();

    deserialize_exemple()
}
