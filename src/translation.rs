use crate::schema::{SentenceType, Token, Translation};

impl Translation {
    #[allow(unused)]
    pub const fn get_translation_id(&self) -> u32 {
        self.id
    }

    #[allow(unused)]
    pub fn new(id: u32, tokens: &[&str], lang: &str, sent_type: SentenceType) -> Self {
        let words = tokens
            .iter()
            .enumerate()
            .map(|(i, token)| Token::new(i as u32, token))
            .collect();

        Self {
            lang: lang.to_string(),
            id,
            sentence_type: sent_type,
            tokens: words,
        }
    }
}
