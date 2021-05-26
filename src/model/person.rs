use serde::{Serialize, Deserialize};
use std::fmt;

static mut PERSON_ID: u16 = 0;

/**
 * Defines a normal person, nothing weird here.
 * - id : The id of the person
 * - firstname: The firstname of the person
 * - lastname: The lastname of the person
 */
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct Person<'a> {
    id: u16,
    firstname: &'a str,
    lastname: &'a str,
}

impl<'a> Person<'a> {
    /**
     * Instanciates a new person
     * - firstname: The firstname of the person
     * - lastname: The lastname of the person
     */
    pub unsafe fn new(firstname: &'a str, lastname: &'a str) -> Person<'a> {
        PERSON_ID += 1;
        Person {
            id: PERSON_ID,
            firstname: firstname,
            lastname: lastname,
        }
    }

    /**
     * Gets id
     * - self
     */
    pub fn get_id(self) -> u16 {
        self.id
    }

    /**
     * Gets firstname
     * - self
     */
    pub fn get_firstname(self) -> String {
        String::from(self.firstname)
    }

    /**
     * Gets lastname
     * - self
     */
    pub fn get_lastname(self) -> String {
        String::from(self.lastname)
    }
}

impl<'a> fmt::Display for Person<'a> {
    /**
     * To string for a person
     * - self
     * - f : The formatter chosen for the to string
     * returns Result
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "id: {} \nlastname: {} \nsurname: {}",
            self.id, self.firstname, self.lastname
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{Token, assert_tokens};

    /**
     * Test the instanciation
     */
    #[test]
    fn new_test_1() {
        let donna: Person;

        unsafe {
            donna = Person::new("Donna", "Lonne");
        }
        assert_eq!(&donna.get_firstname(), "Donna");
        assert_eq!(&donna.get_lastname(), "Lonne");
    }

    /**
     * Test the failure of the instanciation
     */
    #[test]
    fn new_test_2() {
        let donna: Person;

        unsafe {
            donna = Person::new("Donna", "Lonne");
        }
        assert_ne!(&donna.get_firstname(), "Donald");
        assert_eq!(&donna.get_lastname(), "Lonne");
    }
}
