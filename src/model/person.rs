use std::fmt;

static mut person_id: u32 = 0;

#[derive(Clone, Copy)]
pub struct Person<'a> {
    id: u32,
    pub lastname: &'a str,
    pub surname: &'a str,
}

impl<'a> Person<'a> {
    pub unsafe fn new(lastname: &'a str, surname: &'a str) -> Person<'a> {
        person_id += 1;
        return Person {
            id: person_id,
            lastname: lastname,
            surname: surname,
        };
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
