
pub struct Player<'a, 'b, 'c> {
    pub name: &'a str,
    pub skills: &'a Vec<&'b String>,
    pub characters: &'a Vec<&'b String>,
    pub resources: &'a Vec<&'b String>,
    pub mark: &'a String,
    pub memories: &'a Vec<&'b Memory<'b, 'c>>
}

pub struct Memory<'a, 'b> {
    pub experiences: &'a Vec<&'b String>
}

trait Verified {
    fn validate(&self) -> bool;
}

impl Verified for Memory<'_, '_> {
    fn validate(&self) -> bool {
        if self.experiences.len() > 3 {
            return false;
        }
        true
    }
}
