use crate::{
    macros::{create_file, into_stderr, stderr_with_message},
    schema::Corpus,
};
use std::{ffi::OsStr, fs, io::Write, path::Path};

const FILE_EXT: &str = "corp";

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

    pub fn load(source: &Path) -> std::io::Result<Corporeum> {
        if source.extension().and_then(OsStr::to_str).unwrap() != FILE_EXT {
            return Err(stderr_with_message!("Invalid file extension"));
        }

        let file = fs::OpenOptions::new().read(true).open(source)?;
        let corpus: Corpus = into_stderr!(bincode::deserialize_from(file))?;

        Ok(Corporeum {
            original_file_path: source,
            corpus,
        })
    }

    pub fn save(&self) -> std::io::Result<()> {
        let buffer = into_stderr!(bincode::serialize(&self.corpus))?;
        let dest = self.original_file_path.with_extension(FILE_EXT);
        let mut file = create_file!(dest)?;

        file.write_all(&buffer)
    }

    pub fn save_as(&self, path: &Path) -> std::io::Result<()> {
        let buffer = into_stderr!(bincode::serialize(&self.corpus))?;
        let dest = path.with_extension(FILE_EXT);
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
