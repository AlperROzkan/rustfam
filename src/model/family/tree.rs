use Tree::Node;

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
pub enum Tree<'a> {
    Empty,
    Node {
        node_1: Family<'a>,
        node_2: Family<'a>,
        linking: Pairing<&'a Person<'a>>,
        node_3: Box<Tree<'a>>,
    },
}

impl<'a> Tree<'a> {
    /**
    Instanciates a new tree.
    The third node is filled by an empty node. 
    * node_1: first family
    * node_2: second family
    * linking: the two people linking the family
    */
    pub fn new(
        node_1: Family<'a>,
        node_2: Family<'a>,
        linking: Pairing<&'a Person<'a>>,
    ) -> Tree<'a> {
        Node {
            node_1,
            node_2,
            linking,
            node_3: Box::new(Tree::Empty),
        }
    }

    /// Get a reference to the tree's node_1
    pub fn node_1(&self) -> &'a Family<'a> {
        &self.node_1()
    }

    /// Get a reference to the tree's node_2
    pub fn node_2(&self) -> &'a Family<'a> {
        &self.node_2()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::family::pairing::Pairing;

    use super::*;

    #[test]
    pub fn new_test_1() -> Result<(), String> {
        let fam1: Family = Family::init_1();
        let fam2: Family = Family::init_2();
        let mut fam1_clone = fam1.clone();
        let mut fam2_clone = fam2.clone();
        let person_1 = fam1_clone
            .get_child_firstname(String::from("Artie"))
            .ok_or_else(|| "Error when searching child")?;
        let person_2 = fam2_clone
            .get_child_firstname(String::from("Perinne"))
            .ok_or_else(|| "Error when searching child")?;

        let tree: Tree = Tree::new(fam1, fam2, Pairing::new(&person_1, &person_2));

        Err(String::from("Test not finished"))
        
    }
}
