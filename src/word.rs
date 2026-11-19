use std::fmt::{Display, Formatter, Result};

pub struct Word {
    content: String,
}

impl Word {
    pub fn new(content: String) -> Self {
        return Word { content };
    }

    pub fn content(&self) -> &str {
        return &self.content;
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return f.pad(self.content());
    }
}
