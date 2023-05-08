use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct Corpus {
    pub(crate) metadata: Option<Metadata>,
    pub(crate) documents: Vec<Document>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Metadata {
    pub(crate) authors: Vec<Author>,
    // this does not follow semantic versioning, it is just a number
    pub(crate) created: Option<i64>,
    pub(crate) description: Option<String>,
    pub(crate) modified: Option<i64>,
    pub(crate) name: String,
    pub(crate) version: u16,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Author {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) mail: Option<String>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Document {
    pub(crate) description: Option<String>,
    pub(crate) id: u32,
    pub(crate) sentences: Vec<Sentence>,
    pub(crate) source: Option<String>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer"))]
pub struct Sentence {
    pub(crate) id: u32,
    // TODO enum - could be either 'source' or 'target'
    pub(crate) lang: String, // TODO features, labels
    // pub(crate) sentence_type: SentenceType, // language identifier
    pub(crate) tokens: Vec<Token>,
    #[omit_bounds]
    pub(crate) translations: Vec<Sentence>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
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

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Translation {
    pub(crate) id: u32,
    pub(crate) lang: String,
    pub(crate) sentence_type: SentenceType,
    pub(crate) tokens: Vec<Token>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum SentenceType {
    Source,
    Translation,
}
