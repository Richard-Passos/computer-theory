use crate::grammar::Grammar;
use crate::word::Word;

pub trait Recognizer {
    fn recognize(&self, word: &Word) -> bool;
}

pub struct WordRecognizer<'grammar, G: Grammar> {
    grammar: &'grammar G,
}

impl<'grammar, G: Grammar> WordRecognizer<'grammar, G> {
    pub fn new(grammar: &'grammar G) -> Self {
        return WordRecognizer { grammar };
    }
}

impl<'grammar, G: Grammar> Recognizer for WordRecognizer<'grammar, G> {
    fn recognize(&self, word: &Word) -> bool {
        todo!();
    }
}
