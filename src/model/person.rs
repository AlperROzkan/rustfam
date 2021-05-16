use std::fmt;

pub struct Person {
    pub lastname: String,
    pub surname: String,
}

impl Person {
    pub fn new(lastname: &str, surname: &str) -> Person {
        Person {
            lastname: String::from(lastname),
            surname: String::from(surname),
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "lastname: {} \nsurname: {}", self.lastname, self.surname)
    }
}
