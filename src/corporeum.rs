use crate::{
    macros::{create_file, into_stderr, stderr_with_message},
    schema::Corpus,
};
use std::{ffi::OsStr, fs, io::Write, path::Path};

pub const FILE_EXT: &str = "corp";

pub struct Corporeum<'a> {
    original_file_path: &'a Path,
    corpus: Corpus,
}

impl Corporeum<'_> {
    /// Creates a new empty [`Corporeum`](Corporeum).
    ///
    /// After modifying the [`Corpus`](Corpus), you can use [`save()`](Self::save)
    /// to save it into a file (`buffer`), which can be later loaded with [`load()`](Self::load).  
    pub fn new(buffer: &Path) -> Corporeum {
        let corpus = Corpus::default();

        Corporeum {
            original_file_path: buffer,
            corpus,
        }
    }

    /// Loads a [`Corporeum`](Corporeum) from a file.
    ///
    /// # Errors
    /// - The file extention must match [FILE_EXT](FILE_EXT), otherwise an error is
    /// returned.
    /// - If the `source` file cannot be opened, an error is returned.
    /// - Lastly, an error shall be returned if the deserialization fails.
    pub fn load(source: &Path) -> std::io::Result<Corporeum> {
        if source.extension().and_then(OsStr::to_str).unwrap() != FILE_EXT {
            return Err(stderr_with_message!("Invalid file extension"));
        }

        /* let file = fs::OpenOptions::new().read(true).open(source)?;
        let corpus: Corpus = into_stderr!(bincode::deserialize_from(file))?; */

        let data = fs::read(source).unwrap();
        let corpus: Corpus = bincode::deserialize(&data).unwrap();

        Ok(Corporeum {
            original_file_path: source,
            corpus,
        })
    }

    /// Saves the current instance into the file from which the [`Corporeum`](Corporeum)
    /// was created from.
    ///
    /// # Errors
    /// This method may return an error if:
    /// - The serialization fails
    /// - The destination file could not be opened
    /// - An error occurred while writing to the file.
    pub fn save(&self) -> std::io::Result<()> {
        let buffer = into_stderr!(bincode::serialize(&self.corpus))?;
        let dest = self.original_file_path.with_extension(FILE_EXT);
        let mut file = create_file!(dest)?;

        file.write_all(&buffer)
    }

    /// Saves the current instance into the specified file.
    ///
    /// # Errors
    /// Same as [save()](Self::save).
    pub fn save_as(&self, path: &Path) -> std::io::Result<()> {
        let buffer = into_stderr!(bincode::serialize(&self.corpus))?;
        let dest = path.with_extension(FILE_EXT);
        let mut file = create_file!(dest)?;

        file.write_all(&buffer)
    }

    /// Returns a reference to the [Corpus](Corpus).
    pub const fn corpus(&self) -> &Corpus {
        &self.corpus
    }

    /// Returns a mutable reference to the [Corpus](Corpus).
    pub fn corpus_mut(&mut self) -> &mut Corpus {
        &mut self.corpus
    }
}
