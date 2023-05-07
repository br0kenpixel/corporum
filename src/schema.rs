use serde::Deserialize;
use serde::Serialize;
// use serde_derive::Deserialize;
// use serde_derive::Serialize;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Corpus {
    pub(crate) metadata: Option<Metadata>,
    pub(crate) documents: Vec<Document>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub(crate) authors: Vec<Author>,
    // this does not follow semantic versioning, it is just a number
    pub(crate) created: Option<i64>,
    pub(crate) description: Option<String>,
    pub(crate) modified: Option<i64>,
    pub(crate) name: String,
    pub(crate) version: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) mail: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Document {
    pub(crate) description: Option<String>,
    pub(crate) id: u32,
    pub(crate) sentences: Vec<Sentence>,
    pub(crate) source: Option<String>,
    // #[serde(skip_serializing)]
    // pub(crate) last_sentence_id: u32, // cache next id
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sentence {
    pub(crate) id: u32,
    // TODO enum - could be either 'source' or 'target'
    pub(crate) lang: String, // TODO features, labels
    // pub(crate) sentence_type: SentenceType, // language identifier
    pub(crate) tokens: Vec<Token>,
    pub(crate) translations: Vec<Sentence>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub(crate) id: u32,
    pub(crate) form: String,
    pub(crate) lemma: Option<String>,
    pub(crate) upos: Option<String>,
    pub(crate) xpos: Option<String>,
    pub(crate) feats: Option<String>,
    pub(crate) head: Option<String>,
    pub(crate) deprel: Option<String>,
    pub(crate) deps: Option<String>,
    pub(crate) misc: Option<String>, // this is here only for compatibilty reasons with CoNLL-U
}

// #[derive(Deserialize, Serialize, Debug)]
// pub struct Translations {
//     #[serde(rename = "translation")]
//     pub(crate) translations: Vec<Translation>,
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct Translation {
    pub(crate) id: u32,
    pub(crate) lang: String,
    pub(crate) sentence_type: SentenceType,
    pub(crate) tokens: Vec<Token>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SentenceType {
    Source,
    Translation,
}
