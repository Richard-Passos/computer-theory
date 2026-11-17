pub struct Word {
    size: usize,
    content: String,
}

impl Word {
    pub fn new(content: String) -> Self {
        return Word {
            size: content.len(),
            content,
        };
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn content(&self) -> &str {
        return &self.content;
    }
}
