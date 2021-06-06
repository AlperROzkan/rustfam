use serde::{Deserialize, Serialize};

/**
A Pairing between two things.
* linked_1
* linked_2
*/
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pairing<T> {
    // Each attribute must be borrowed explicitely
    linked_1: T,
    linked_2: T,
}

impl<T> Pairing<T> {
    pub fn new(linked_1: T, linked_2: T) -> Self {
        Self { linked_1, linked_2 }
    }

    /// Get a reference to the pairing's linked person 1.
    pub fn linked_1(&self) -> &T {
        &self.linked_1
    }

    /// Get a mutable reference to the pairing's linked person 1.
    pub fn linked_1_mut(&mut self) -> &mut T {
        &mut self.linked_1
    }

    /// Get a reference to the pairing's linked person 2.
    pub fn linked_2(&self) -> &T {
        &self.linked_2
    }

    /// Get a mutable reference to the pairing's linked person 2.
    pub fn linked_2_mut(&mut self) -> &mut T {
        &mut self.linked_2
    }
}
