use crate::Person;
use std::fmt;

static mut FAMILY_ID: u16 = 0;

/**
 * Struct defining a close family.
 * Describes a very close family with two parents and their children.
 * It does not describe an extended family like cousins, aunts, uncles, grandparents, ...
 * - name:      Family name
 * - parents:   Parents parents of the family (0 <= 2)
 * - children:  The children of the family
 */
#[derive(Clone)]
pub struct CloseFamily<'a> {
    id: u16,
    pub name: &'a str,
    pub parents: Vec<Person<'a>>,
    pub children: Vec<Person<'a>>,
}

impl<'a> CloseFamily<'a> {
    pub fn new(name: &'a str) -> CloseFamily<'a> {
        unsafe {
            FAMILY_ID += 1;
            CloseFamily {
                id: FAMILY_ID,
                name: name,
                parents: Vec::new(),
                children: Vec::new(),
            }
        }
    }

    /**
     * Instanciates a CloseFamily with empty parents and children
     * - name:      Family name
     * - parents:   The parents of the family. This is an already existing vector.
     * - children:  The children of the family. This is an already existing vector.  
     */
    pub fn new_existing(
        name: &'a str,
        parents: Vec<Person<'a>>,
        children: Vec<Person<'a>>,
    ) -> CloseFamily<'a> {
        unsafe {
            CloseFamily {
                id: FAMILY_ID,
                name: name,
                parents: parents,
                children: children,
            }
        }
    }

    /**  
     * Adds a parent to the family
     * TODO : Error if more than two
     * - self
     * - parent: A parent
     */
    pub fn add_parent(&mut self, parent: Person<'a>) {
        self.parents.push(parent);
    }

    /**
     * Replaces parents
     * TODO : Error if more than two
     * - self
     * - parents : a collection of parents
     */
    pub fn replace_parents(&mut self, parents: Vec<Person<'a>>) {
        self.parents = parents;
    }

    /**
     * Adds a child to the family
     * - self
     * - child: A child born from the parents
     */
    pub fn add_child(&mut self, child: Person<'a>) {
        self.children.push(child);
    }

    /**
     * Replaces children
     * - self
     * - children : a collection of children
     */
    pub fn replace_children(&mut self, children: Vec<Person<'a>>) {
        self.children = children;
    }

    /**
     * Adds multiple children
     * - self
     * - children
     */
    pub fn add_children(&mut self, children: Vec<Person<'a>>) {
        for elem in children {
            self.add_child(elem);
        }
    }
}

impl<'a> fmt::Display for CloseFamily<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = String::from("[Parents]\n");
        for elem in &self.parents {
            result.push_str(&elem.to_string());
        }
        result += "[Children]\n";
        let mut i: u8 = 0;
        for elem in &self.children {
            result += "Child n°";
            result.push_str(&i.to_string());
            result += "\n";
            result.push_str(&elem.to_string());
            i += 1;
        }
        writeln!(f, "{}", result)
    }
}
