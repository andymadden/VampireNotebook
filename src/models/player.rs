use crate::Memory;

pub struct Player<'a, 'b, 'c> {
    pub name: &'a str,
    pub skills: &'a Vec<&'b String>,
    pub characters: &'a Vec<&'b String>,
    pub resources: &'a Vec<&'b String>,
    pub mark: &'a String,
    pub memories: &'a Vec<&'b Memory<'b, 'c>>
}

