use std::fmt;

static mut PERSON_ID: u16 = 0;

#[derive(Clone, Copy)]
pub struct Person<'a> {
    id: u16,
    pub lastname: &'a str,
    pub surname: &'a str,
}

impl<'a> Person<'a> {
    pub unsafe fn new(lastname: &'a str, surname: &'a str) -> Person<'a> {
        PERSON_ID += 1;
        Person {
            id: PERSON_ID,
            lastname: lastname,
            surname: surname,
        }
    }
}

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "id: {} \nlastname: {} \nsurname: {}",
            self.id, self.lastname, self.surname
        )
    }
}
