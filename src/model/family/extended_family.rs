use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use super::{close_family::CloseFamily, pairing::Pairing};

/**
 * Struct defining an extended family.
 * Describes an extended family with a network of connected families.
 * Will link mulitple families.
 * It is a liaison between two children of different families. 
 * - name: Name of the extended family
 * - families: A collection of families with a pairing of married persons as keys
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedFamily<'a> {
    name: &'a str,
    families: HashMap<Pairing<'a>, CloseFamily<'a>>
}

impl<'a> ExtendedFamily<'a> {
    pub fn new(name: &'a str, families: HashMap<Pairing<'a>, CloseFamily<'a>>) -> Self { Self { name, families } }

    /// Get a reference to the extended family's name.
    pub fn name(&self) -> &&'a str {
        &self.name
    }

    /// Get a mutable reference to the extended family's name.
    pub fn name_mut(&mut self) -> &mut &'a str {
        &mut self.name
    }

    /// Set the extended family's name.
    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }

    /// Get a reference to the extended family's families.
    pub fn families(&self) -> &HashMap<Pairing<'a>, CloseFamily<'a>> {
        &self.families
    }

    /// Get a mutable reference to the extended family's families.
    pub fn families_mut(&mut self) -> &mut HashMap<Pairing<'a>, CloseFamily<'a>> {
        &mut self.families
    }

    /// Set the extended family's families.
    pub fn set_families(&mut self, families: HashMap<Pairing<'a>, CloseFamily<'a>>) {
        self.families = families;
    }
}