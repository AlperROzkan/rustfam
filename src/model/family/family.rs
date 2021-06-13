use serde::{Serialize};
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use crate::model::person::Person;

static mut FAMILY_ID: u16 = 0;

/**
Get a stringified family from a file
* path : path to the family to stringify
*/
pub fn read_family_from_file<P: AsRef<Path>>(path: P) -> String {
    let mut buf_reader;
    let mut contents;

    match File::open(path) {
        Ok(file) => {
            buf_reader = BufReader::new(file);
            contents = String::new();
            match buf_reader.read_to_string(&mut contents) {
                Ok(_) => (),
                Err(e) => panic!("Error when reading family from file: {}", e),
            };
            contents
        }
        Err(e) => panic!(
            "Error when opening file when reading a family from file: {}",
            e
        ),
    }
}

/**
Struct defining a close family.
Describes a very close family with two parents and their children.
It does not describe an extended family like cousins, aunts, uncles, grandparents, ...
* name:     Family name
* parents:  Parents parents of the family (0 <= 2)
* children: The children of the family
*/
#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct Family<'a> {
    id: u16,
    name: &'a str,
    parents: Vec<Person<'a>>,
    children: Vec<Person<'a>>,
}

impl<'a> Family<'a> {
    /**
     * Instaniates a Family
     */
    pub fn new(name: &'a str) -> Family<'a> {
        unsafe {
            FAMILY_ID += 1;
            Family {
                id: FAMILY_ID,
                name: name,
                parents: Vec::new(),
                children: Vec::new(),
            }
        }
    }

    /**
    Instanciates a Family with empty parents and children
     * name:      Family name
     * parents:   The parents of the family. This is an already existing vector.
     * children:  The children of the family. This is an already existing vector.
     */
    pub fn new_existing(
        name: &'a str,
        parents: Vec<Person<'a>>,
        children: Vec<Person<'a>>,
    ) -> Family<'a> {
        unsafe {
            Family {
                id: FAMILY_ID,
                name,
                parents,
                children,
            }
        }
    }

    /**
    Adds a parent to the family
    * TODO : Error if more than two
    * self
    * parent: A parent
    */
    pub fn add_parent(&mut self, parent: Person<'a>) {
        self.parents.push(parent);
    }

    /**
    Adds a child to the family
    * self
    * child: A child born from the parents
    */
    pub fn add_child(&mut self, child: Person<'a>) {
        self.children.push(child);
    }

    /**
    Adds multiple children
    * self
    * children
    */
    pub fn add_children(&mut self, children: Vec<Person<'a>>) {
        for elem in children {
            self.add_child(elem);
        }
    }

    /**
    Gets a family member from it's firstname. Checks the parents and children vecs.
    * self
    * firstname : the firstname of the person to search
    */
    pub fn get_family_member_firstname(&mut self, firstname: String) -> Option<Person> {
        for person in self.parents.iter() {
            if person.firstname().eq(&firstname) {
                return Some(*person);
            }
        }
        for person in self.children.iter() {
            if person.firstname().eq(&firstname) {
                return Some(*person);
            }
        }
        None
    }

    /**
    Get a parent with his firstname.
    * self
    * firstname : the firstname of the person
    */
    pub fn get_parent_firstname(&mut self, firstname: String) -> Option<Person> {
        for parent in self.parents.iter() {
            if parent.firstname().eq(&firstname) {
                return Some(*parent);
            }
        }
        None
    }

    /**
    Get a child with his firstname.
    * self
    * firstname : the firstname of the person
    */
    pub fn get_child_firstname(&mut self, firstname: String) -> Option<Person> {
        for child in self.children.iter() {
            if child.firstname().eq(&firstname) {
                return Some(*child);
            }
        }
        None
    }
    
    /**
    Write this family to a file
    * TODO : Test
    * self
    * path: Path to file to write to
    */
    pub fn write_to_file<P: AsRef<Path>>(self, path: P) {
        // Create a file and write this family to it
        match File::create(&path) {
            Ok(file) => match serde_json::to_writer(file, &self) {
                Ok(_) => (),
                Err(e) => panic!("Error when serializing family to file: {}", e),
            },
            Err(e) => panic!(
                "Error when creating file while writing family to a file. Error {}",
                e
            ),
        }
    }

    /**
    Initializes a family
    */
    pub fn init_1() -> Family<'a> {
        let gregor: Person;
        let gloria: Person;
        let artie: Person;

        unsafe {
            gregor = Person::new("Gregor", "Drouv");
            gloria = Person::new("Gloria", "Donia");
            artie = Person::new("Artie", "Drouv");
        }
        
        Family::new_existing("drouv-donia", vec![gregor, gloria], vec![artie])
    }

    /**
    Initializes a family
    */
    pub fn init_2() -> Family<'a> {
        let mut return_fam: Family<'a> = Family::new("drouv-donia");
        let jack: Person;
        let gislaine: Person;
        let perinne: Person;

        unsafe {
            jack = Person::new("Jack", "Tarot");
            gislaine = Person::new("Gislaine", "Montaig");
            perinne = Person::new("Perinne", "Tarot");
        }

        return_fam.add_parent(jack);
        return_fam.add_parent(gislaine);
        return_fam.add_child(perinne);
        return_fam        
    }
}

impl<'a> fmt::Display for Family<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = String::from("[Parents]\n");
        for elem in &self.parents {
            result.push_str(&elem.to_string());
        }
        result += "[Children]\n";
        let mut i: u8 = 0;
        for elem in &self.children {
            result += "Child n°";
            result.push_str(&i.to_string());
            result += "\n";
            result.push_str(&elem.to_string());
            i += 1;
        }
        writeln!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Read};

    use super::*;

    /*
     * Setup a family to do tests to
     */
    fn setup() -> Family<'static> {
        let mut fam_test: Family = Family::new("Glenn-Pierce");
        let helen: Person;
        let george: Person;
        let rose: Person;
        unsafe {
            helen = Person::new("Helen", "Glenn");
            george = Person::new("George", "Pierce");
            rose = Person::new("Rose", "Pierce");
        }

        fam_test.add_parent(helen);
        fam_test.add_parent(george);
        fam_test.add_child(rose);

        fam_test
    }

    /**
     * Test if write wrote correctly
     */
    #[test]
    fn write_to_file_1() {
        // Initialize family
        let fam_test: Family = setup();
        let path: &Path = Path::new("data/fam_test.json");

        // Write family to test_file
        fam_test.write_to_file(path);

        // Verify if file contents corespond to family
        match File::create(path) {
            Ok(file) => {
                let mut buf_reader: BufReader<File> = BufReader::new(file);
                let mut contents = String::new(); // Read family from file
                buf_reader.read_to_string(&mut contents);
                assert_eq!(contents, read_family_from_file(path));
            }
            Err(e) => panic!("Test, error write_to_file_1: {}", e),
        }
    }

    #[test]
    fn get_child_firstname_test_1() {
        let mut fam_test: Family = setup();
        
        // Test if we can find someone named "Rose" in family
        match fam_test.get_child_firstname(String::from("Rose")) {
            Some(child) => assert_eq!(String::from(child.firstname()), String::from("Rose")),
            None => panic!("get_child_firstname_test_1 : failed to find child from a family"),
        }
    }

    #[test]
    fn get_child_firstname_test_2() {
        let mut fam_test: Family = setup();
        
        // Test if we can find someone named "Rosen" in family
        match fam_test.get_child_firstname(String::from("Rose")) {
            Some(child) => assert_ne!(String::from(child.firstname()), String::from("Rosen")),
            None => panic!("get_child_firstname_test_2 : failed to find child from a family"),
        }
    }

    #[test]
    fn get_parent_firstname_test_1() {
        let mut fam_test: Family = setup();
        
        // Test if we can find someone named "Helen" in family
        match fam_test.get_parent_firstname(String::from("Helen")) {
            Some(parent) => assert_eq!(String::from(parent.firstname()), String::from("Helen")),
            None => panic!("get_parent_firstname_test_1 : failed to find parent from a family"),
        }
    }

    #[test]
    fn get_parent_firstname_test_2() {
        let mut fam_test: Family = setup();
        
        // Test if we can find someone named "Helen" in family
        match fam_test.get_parent_firstname(String::from("George")) {
            Some(parent) => assert_eq!(String::from(parent.firstname()), String::from("George")),
            None => panic!("get_parent_firstname_test_2 : failed to find parent from a family"),
        }
    }

    #[test]
    fn get_parent_firstname_test_3() {
        let mut fam_test: Family = setup();
        let firstname_test = "false parent";
        
        // Test if we can find someone named "Helen" in family
        match fam_test.get_parent_firstname(String::from("George")) {
            Some(parent) => assert_ne!(String::from(parent.firstname()), String::from(firstname_test)),
            None => panic!("get_parent_firstname_test_2 : failed to find parent from a family"),
        }
    }
}
