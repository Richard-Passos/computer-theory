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
