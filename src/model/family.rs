use crate::Person;
use std::fmt;

/**
 * Struct defining a close family.
 * Describes a very close family with two parents and their children.
 * It does not describe an extended family like cousins, aunts, uncles, grandparents, ...
 * - name:      Family name
 * - parents:   Parents parents of the family (0 <= 2)
 * - children:  The children of the family
 */
pub struct CloseFamily {
    pub name: String,
    pub parents: Vec<Person>,
    pub children: Vec<Person>,
}

impl CloseFamily {
    /** 
     * Instanciates a CloseFamily with empty parents and children
     * - name:  Family name
     */
    pub fn new(name: &str) -> CloseFamily {
        CloseFamily {
            name: String::from(name),
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

    /**  
     * Adds a parent to the family
     * TODO : Error if more than two
     * - self
     * - parent: A parent
     */
    pub fn add_parent(&mut self, parent: Person) {
        self.parents.push(parent);
    }

    /**
     * Adds a child to the family
     * - self
     * - child: A child born from the parents
     */
    pub fn add_child(&mut self, child: Person) {
        self.children.push(child);
    }
}

impl fmt::Display for CloseFamily {
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
