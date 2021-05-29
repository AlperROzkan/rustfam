mod model;
use model::family::CloseFamily;
use model::family::read_family_from_file;
use model::person::Person;
// File import-export
use std::path::Path;

fn main() {
    /*
     * Family declaration
     */
    let mut fam1: CloseFamily = CloseFamily::new("Glenn-Pierce");
    let mut fam2: CloseFamily = CloseFamily::new("Trent-Pierce");

    /*
     * Person declaration
     */
    let helen: Person;
    let george: Person;
    let rose: Person;
    let tom: Person;
    let rosa: Person;
    let gregor: Person;

    /*
     * Initialization of the people
     */
    unsafe {
        helen = Person::new("Helen", "Glenn");
        george = Person::new("George", "Pierce");
        rose = Person::new("Tom", "Pierce");
        tom = Person::new("Tom", "Pierce");
        rosa = Person::new("Rosa", "Trent");
        gregor = Person::new("Gregor", "Pierce");
    }

    /*
     * Add people to the family
     */
    fam1.add_parent(helen);
    fam1.add_parent(george);
    fam1.add_child(rose);
    fam1.add_child(tom);

    fam2.add_parent(rosa);
    fam2.add_parent(gregor);
    fam2.add_child(george);

    /*

    // ************
    // Write & Read
    // ************
    let mut file: File; // File to read & write from
    let path_file = Path::new("./data/fam1.txt");

    /*
    * Write family to file
    */
    fs::remove_file(path_file)?;
    file = File::create(path_file)?;
    file.write_all(b"Ducks");

    /*
     * Read family from file
     */
    file = File::open(path_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    print!("{}", contents);

    */

    fam1.write_to_file(Path::new("data/fam1.txt"));
    println!("{}", read_family_from_file(Path::new("data/fam1.txt")));
}
