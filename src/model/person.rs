use std::fmt;

pub struct Person {
    pub lastname: String,
    pub surname: String,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "lastname: {} \nsurname: {}", self.lastname, self.surname)
    }
}

