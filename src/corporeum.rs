use crate::{
    macros::{create_file, into_stderr, stderr_with_message},
    schema::Corpus,
};
use serde_pickle::{DeOptions, SerOptions};
use std::{ffi::OsStr, fs, io::Write, path::Path};

const JSON: &str = "json";
const PICKLE: &str = "pickle";
const MSGPACK: &str = "msgpack";
const CBOR: &str = "cbor";
const BINCODE: &str = "bincode";

pub struct Corporeum<'a> {
    original_file_path: &'a Path,
    corpus: Corpus,
}

impl Corporeum<'_> {
    pub fn new(buffer: &Path) -> Corporeum {
        let corpus = Corpus::default();

        Corporeum {
            original_file_path: buffer,
            corpus,
        }
    }

    // function to load an already existing corpus
    pub fn load(source: &Path) -> std::io::Result<Corporeum> {
        let data = fs::read_to_string(source)?;

        // parse json file
        let corpus = match source.extension().and_then(OsStr::to_str).unwrap() {
            JSON => into_stderr!(serde_json::from_str(&data)),
            PICKLE => into_stderr!(serde_pickle::from_slice(
                data.as_bytes(),
                DeOptions::default()
            )),
            MSGPACK => into_stderr!(rmp_serde::from_slice(data.as_bytes())),
            CBOR => into_stderr!(serde_cbor::from_slice(data.as_bytes())),
            BINCODE => into_stderr!(bincode::deserialize(data.as_bytes())),
            _ => Err(stderr_with_message!("Unsupported file format")),
        }?;
        // let mut corpus: Corpus = serde_json::from_str(&data).unwrap();
        // let mut corp: Corpus =
        // serde_pickle::from_slice(&data.as_bytes(), Default::default()).unwrap();
        // iterate over docs and setup last sentence id,
        // so we do not have search for last available id every time we add new sentence
        // corpus
        //     .documents
        //     .iter_mut()
        //     .for_each(|doc| doc.last_sentence_id = doc.setup_last_sentence_id());

        Ok(Corporeum {
            original_file_path: source,
            corpus,
        })
    }

    pub fn save_json(&self, pretty: bool) -> std::io::Result<()> {
        let buffer = if pretty {
            serde_json::to_vec_pretty(&self.corpus)
        } else {
            serde_json::to_vec(&self.corpus)
        }?;

        let dest = self.original_file_path.with_extension(JSON);
        let mut file = create_file!(dest)?;

        file.write_all(&buffer)
    }

    pub fn save_as_json(&self, destination: &Path, pretty: bool) -> std::io::Result<()> {
        let buffer = if pretty {
            serde_json::to_vec_pretty(&self.corpus)
        } else {
            serde_json::to_vec(&self.corpus)
        }?;

        let dest = destination.with_extension(JSON);
        let mut file = create_file!(dest)?;

        file.write_all(&buffer)
    }

    pub fn save_pickle(&self) -> std::io::Result<()> {
        let buffer = into_stderr!(serde_pickle::to_vec(&self.corpus, SerOptions::default()))?;

        let dest = self.original_file_path.with_extension(PICKLE);
        let mut file = create_file!(dest)?;

        file.write_all(&buffer)
    }

    pub fn save_as_pickle(&self, destination: &Path) -> std::io::Result<()> {
        let buffer = into_stderr!(serde_pickle::to_vec(&self.corpus, SerOptions::default()))?;

        let dest = destination.with_extension(PICKLE);
        let mut file = create_file!(dest)?;

        file.write_all(&buffer)
    }

    pub fn save_msgpack(&self) -> std::io::Result<()> {
        let buffer = into_stderr!(rmp_serde::to_vec(&self.corpus))?;
        let dest = self.original_file_path.with_extension(MSGPACK);

        let mut file = create_file!(dest)?;
        file.write_all(&buffer)
    }

    pub fn save_as_msgpack(&self, destination: &Path) -> std::io::Result<()> {
        let buffer = into_stderr!(rmp_serde::to_vec(&self.corpus))?;
        let dest = destination.with_extension(MSGPACK);

        let mut file = create_file!(dest)?;
        file.write_all(&buffer)
    }

    pub fn save_cbor(&self, packed: bool) -> std::io::Result<()> {
        let buffer = into_stderr!(if packed {
            serde_cbor::ser::to_vec_packed(&self.corpus)
        } else {
            serde_cbor::ser::to_vec(&self.corpus)
        })?;

        let dest = self.original_file_path.with_extension(CBOR);
        let mut file = create_file!(dest)?;

        file.write_all(&buffer)
    }

    pub fn save_as_cbor(&self, destination: &Path, packed: bool) -> std::io::Result<()> {
        let buffer = into_stderr!(if packed {
            serde_cbor::ser::to_vec_packed(&self.corpus)
        } else {
            serde_cbor::ser::to_vec(&self.corpus)
        })?;

        let dest = destination.with_extension(CBOR);
        let mut file = create_file!(dest)?;

        file.write_all(&buffer)
    }

    pub fn save_bincode(&self) -> std::io::Result<()> {
        let buffer = into_stderr!(bincode::serialize(&self.corpus))?;

        let dest = self.original_file_path.with_extension(BINCODE);
        let mut file = create_file!(dest)?;

        file.write_all(&buffer)
    }

    pub fn save_as_bincode(&self, destination: &Path) -> std::io::Result<()> {
        let buffer = into_stderr!(bincode::serialize(&self.corpus))?;
        let dest = destination.with_extension(BINCODE);

        let mut file = create_file!(dest)?;
        file.write_all(&buffer)
    }

    pub const fn corpus(&self) -> &Corpus {
        &self.corpus
    }

    pub fn corpus_mut(&mut self) -> &mut Corpus {
        &mut self.corpus
    }
}
