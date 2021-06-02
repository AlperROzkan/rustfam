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
    /// Instanciates a new family
    pub fn new(
        name: &'a str,
    ) -> Self {
        Self { name, families: HashMap::new()}
    }

    /**
    Add a family to the extended family, must specify a pairing.
     * self
     * couple_pairing: two linked people that are the link between two families
     * family_pairing:
     */
    pub fn add_family(
        &mut self,
        couple_pairing: Pairing<Person<'a>>,
        family_pairing: Pairing<CloseFamily<'a>>,
    ) {
        self.families.insert(couple_pairing, family_pairing);
    }

    /// Get a reference to the extended family's name.
    pub fn name(&self) -> &&'a str {
        &self.name
    }

    /// Set the extended family's name.
    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }

    /// Get a reference to the extended family's families.
    pub fn families(&self) -> &HashMap<Pairing<Person<'a>>, Pairing<CloseFamily<'a>>> {
        &self.families
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_family_test_1() {
        let (gregor, gloria, jack, gislaine, artie, perinne, elena);
        let mut drouv_donia: CloseFamily = CloseFamily::new("drouv_donia");
        let mut tarot_montaig: CloseFamily = CloseFamily::new("tarot_montaig");
        let mut drouv_tarot: CloseFamily = CloseFamily::new("drouv_tarot");
        unsafe {
            gregor = Person::new("Gregor", "Drouv");
            gloria = Person::new("Gloria", "Donia");
            jack = Person::new("Jack", "Tarot");
            gislaine = Person::new("Gislaine", "Montaig");
            artie = Person::new("Artie", "Drouv");
            perinne = Person::new("Perinne", "Tarot");
            elena = Person::new("Elena", "Drouv");
        }
        drouv_donia.add_parent(gregor);
        drouv_donia.add_parent(gloria);
        drouv_donia.add_child(artie);
        tarot_montaig.add_parent(jack);
        tarot_montaig.add_parent(gislaine);
        tarot_montaig.add_child(perinne);
        drouv_tarot.add_parent(artie);
        drouv_tarot.add_parent(perinne);
        drouv_tarot.add_child(elena);
        let mut extended_family: ExtendedFamily = ExtendedFamily::new("drouv_tarov");
    }
}
