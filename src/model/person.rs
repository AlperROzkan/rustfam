use std::fmt;

#[derive(Clone, Copy)]
pub struct Person<'a> {
    pub lastname: &'a str,
    pub surname: &'a str,
}

impl<'a> Person<'a> {
    pub fn new(lastname: &'a str, surname: &'a str) -> Person<'a> {
        Person {
            lastname: lastname,
            surname: surname,
        }
    }
}

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "lastname: {} \nsurname: {}", self.lastname, self.surname)
    }
}