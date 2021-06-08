use super::{family::Family, pairing::Pairing};
use crate::model::person::Person;

/**
Structure made to store families in a tree-like structure.
The third node is a tree, in order to continue the genealogy.
* node_1: First family,
* node_2: Second family,
* linking: the two people linkong the two families
* node_3: Third family, resulting from the union between the first and second family.
*/
pub struct Tree<'a> {
    node_1: &'a Family<'a>,
    node_2: &'a Family<'a>,
    linking: Pairing<&'a Person<'a>>,
    node_3: Box<Tree<'a>>,
}

impl<'a> Tree<'a> {
    pub fn new(
        node_1: &'a Family<'a>,
        node_2: &'a Family<'a>,
        linking: Pairing<&'a Person<'a>>,
        node_3: Box<Tree<'a>>,
    ) -> Self {
        Self {
            node_1,
            node_2,
            linking,
            node_3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Family;
    pub fn new_test_1() {
        let fam1: Family;
        let fam2: Family;
        
    }
}