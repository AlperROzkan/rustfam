use serde::{Deserialize, Serialize};
use std::fmt;

static mut PERSON_ID: u16 = 0;

/**
Defines a normal person, nothing weird here.
* id : The id of the person
* firstname: The firstname of the person
* lastname: The lastname of the person
*/
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Hash, Eq, PartialOrd)]
pub struct Person<'a> {
    id: u16,
    firstname: &'a str,
    lastname: &'a str,
}

impl<'a> Person<'a> {
    /**
    Instanciates a new person
    * firstname: The firstname of the person
    * lastname: The lastname of the person
    */
    pub unsafe fn new(firstname: &'a str, lastname: &'a str) -> Person<'a> {
        PERSON_ID += 1;
        Person {
            id: PERSON_ID,
            firstname: firstname,
            lastname: lastname,
        }
    }

    /// Get a reference to the person's id.
    pub fn id(&self) -> &u16 {
        &self.id
    }

    /// Get a reference to the person's firstname.
    pub fn firstname(&self) -> &'a str {
        &self.firstname
    }

    /// Get a reference to the person's lastname.
    pub fn lastname(&self) -> &'a str {
        &self.lastname
    }
}

impl<'a> fmt::Display for Person<'a> {
    /**
    To string for a person
    * self
    * f : The formatter chosen for the to string
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
mod tests {}
