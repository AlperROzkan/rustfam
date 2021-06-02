use crate::model::person::Person;
use serde::{Deserialize, Serialize};

/**
 * A Pairing between two people. This struct is used as a Key in a Collection in Extended family.
 * - linked_person_1
 * - linked_person_2
 */
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pairing<'a> {
    // Each attribute must be borrowed explicitely
    #[serde(borrow)]
    linked_person_1: Person<'a>,
    #[serde(borrow)]
    linked_person_2: Person<'a>,
}

impl<'a> Pairing<'a> {
    pub fn new(linked_person_1: Person<'a>, linked_person_2: Person<'a>) -> Self {
        Self {
            linked_person_1,
            linked_person_2,
        }
    }

    /// Get a reference to the pairing's linked person 1.
    pub fn linked_person_1(&self) -> &Person<'a> {
        &self.linked_person_1
    }

    /// Get a mutable reference to the pairing's linked person 1.
    pub fn linked_person_1_mut(&mut self) -> &mut Person<'a> {
        &mut self.linked_person_1
    }

    /// Get a reference to the pairing's linked person 2.
    pub fn linked_person_2(&self) -> &Person<'a> {
        &self.linked_person_2
    }

    /// Get a mutable reference to the pairing's linked person 2.
    pub fn linked_person_2_mut(&mut self) -> &mut Person<'a> {
        &mut self.linked_person_2
    }
}
