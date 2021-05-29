use crate::Person;
use serde::{Deserialize, Serialize};
use std::fmt;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

static mut FAMILY_ID: u16 = 0;

/**
* Get a stringified family from a file
* - path : path to the family to stringify
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
 * Struct defining a close family.
 * Describes a very close family with two parents and their children.
 * It does not describe an extended family like cousins, aunts, uncles, grandparents, ...
 * - name:      Family name
 * - parents:   Parents parents of the family (0 <= 2)
 * - children:  The children of the family
 */
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CloseFamily<'a> {
    id: u16,
    name: &'a str,
    parents: Vec<Person<'a>>,
    children: Vec<Person<'a>>,
}

impl<'a> CloseFamily<'a> {
    /**
     * Instaniates a CloseFamily
     */
    pub fn new(name: &'a str) -> CloseFamily<'a> {
        unsafe {
            FAMILY_ID += 1;
            CloseFamily {
                id: FAMILY_ID,
                name: name,
                parents: Vec::new(),
                children: Vec::new(),
            }
        }
    }

    /**
     * Instanciates a CloseFamily with empty parents and children
     * - name:      Family name
     * - parents:   The parents of the family. This is an already existing vector.
     * - children:  The children of the family. This is an already existing vector.  
     */
    pub fn new_existing(
        name: &'a str,
        parents: Vec<Person<'a>>,
        children: Vec<Person<'a>>,
    ) -> CloseFamily<'a> {
        unsafe {
            CloseFamily {
                id: FAMILY_ID,
                name: name,
                parents: parents,
                children: children,
            }
        }
    }

    /*
     * Instanciates a CloseFamily from an existing file
     * - path = Path : Path to read the CloseFamily from
    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Result<CloseFamily<'a>, Box<Error>> {
        // Open the file with a buffer
        let file: File = File::open(path)?;
        let reader = BufReader::new(file);

        // Instanciates the user from the file
        let close_family_from_file = serde_json::from_reader(reader)?;

        // Return the user
        Ok(close_family_from_file)
    }
    */

    /**  
     * Adds a parent to the family
     * TODO : Error if more than two
     * - self
     * - parent: A parent
     */
    pub fn add_parent(&mut self, parent: Person<'a>) {
        self.parents.push(parent);
    }

    /**
     * Replaces parents
     * TODO : Error if more than two
     * - self
     * - parents : a collection of parents
     */
    pub fn replace_parents(&mut self, parents: Vec<Person<'a>>) {
        self.parents = parents;
    }

    /**
     * Adds a child to the family
     * - self
     * - child: A child born from the parents
     */
    pub fn add_child(&mut self, child: Person<'a>) {
        self.children.push(child);
    }

    /**
     * Replaces children
     * - self
     * - children : a collection of children
     */
    pub fn replace_children(&mut self, children: Vec<Person<'a>>) {
        self.children = children;
    }

    /**
     * Adds multiple children
     * - self
     * - children
     */
    pub fn add_children(&mut self, children: Vec<Person<'a>>) {
        for elem in children {
            self.add_child(elem);
        }
    }

    /**
     * Returns the parents of the family
     * - self
     */
    pub fn get_parents(self) -> Vec<Person<'a>> {
        self.parents
    }

    /**
     * Returns the children of the family
     *
     */
    pub fn get_children(self) -> Vec<Person<'a>> {
        self.children
    }

    /**
     * Write this family to a file
     * TODO : Test
     * - self
     * - path: Path to file to write to
     */
    pub fn write_to_file<P: AsRef<Path>>(self, path: P) {
        // Create a file and write this family to it
        match File::create(&path) {
            Ok(file) => {
                match serde_json::to_writer(file, &self) {
                    Ok(_) => (),
                    Err(e) => panic!("Error when serializing family to file: {}", e),
                }
            },
            Err(e) => panic!(
                "Error when creating file while writing family to a file. Error {}",
                e
            ),
        }
    }
}

impl<'a> fmt::Display for CloseFamily<'a> {
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
    fn setup() -> CloseFamily<'static> {
        let mut fam_test: CloseFamily = CloseFamily::new("Glenn-Pierce");
        let helen: Person;
        let george: Person;
        let rose: Person;
        unsafe {
            helen = Person::new("Helen", "Glenn");
            george = Person::new("George", "Pierce");
            rose = Person::new("Tom", "Pierce");
        }

        fam_test
    }

    /*
     * Test if write wrote correctly

    #[test]
    fn write_to_file_1() {

        let mut file: File; // File to read & write from
        let path_file = Path::new("./data/fam_test.json");

        let fam_test: CloseFamily = setup(); // initialize the family
        let fam_read: CloseFamily = fam_test.clone();

        file = File::open(path_file).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        let fam_test: CloseFamily = setup(); // initialize the family
        let fam_read: CloseFamily = fam_test.clone();
        let path_test: &Path = Path::new("./data/fam_test.json"); // path for test file

        match fam_test.write_to_file(&path_test) {
            Ok(()) => {
                match File::create(&path_test) {
                    Ok(file) => {
                        let mut buf_reader: BufReader<File> = BufReader::new(file);
                        let mut contents: String = String::new();
                        match buf_reader.read_to_string(&mut contents) {
                            Ok(_) => assert_eq!(contents, serde_json::to_string(&fam_read).unwrap()),
                            Err(e) => panic!("Error when reading file: {}", e),
                        }
                    },
                    Err(e) => panic!("Error when creating file for reading family: {}", e),
                }
            }
            Err(e) => panic!("Error when writing family to file:  {}", e),
        };
    }*/
}
