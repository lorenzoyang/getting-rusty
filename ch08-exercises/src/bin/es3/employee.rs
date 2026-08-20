#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Employee {
    name: String,
    title: String,
}

impl Employee {
    pub fn new(name: String, title: String) -> Employee {
        Employee { name, title }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn display_line(&self) -> String {
        format!("{}（{}）", self.name, self.title)
    }
}
