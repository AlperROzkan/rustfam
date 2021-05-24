use serde::{
	ser::{SerializeStruct, Serializer},
	Deserialize, Serialize,
};
use serde_json::json;
use std::fmt;

static mut PERSON_ID: u16 = 0;

/**
 * Defines a normal person, nothing weird here.
 * - id : The id of the person
 * - firstname: The firstname of the person
 * - lastname: The lastname of the person 
 */
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Person<'a> {
    id: u16,
    pub firstname: &'a str,
    pub lastname: &'a str,
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
