use super::{close_family::CloseFamily, pairing::Pairing};
use crate::Person;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/**
Struct defining an extended family.
Describes an extended family with a network of connected families.
Will link mulitple families.
It is a liaison between two children of different families.
* name: Name of the extended family
* families: A collection of families with a pairing of married persons as keys
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedFamily<'a> {
    name: &'a str,
    families: HashMap<Pairing<Person<'a>>, Pairing<CloseFamily<'a>>>,
}

impl<'a> ExtendedFamily<'a> {
    
    /**
    Add a family to the extended family, must specify a pairing.
     * self
     * pairing : two linked people that are the link between two families
     * family
     */
    pub fn new(name: &'a str, families: HashMap<Pairing<Person<'a>>, Pairing<CloseFamily<'a>>>) -> Self { Self { name, families } }
}
