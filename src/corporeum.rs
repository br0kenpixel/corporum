use crate::{
    macros::{create_file, into_stderr},
    schema::Corpus,
};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use rkyv::Deserialize;
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub const FILE_EXT: &str = "rkyv.gz";

pub struct Corporeum {
    original_file_path: PathBuf,
    corpus: Corpus,
}

impl Corporeum {
    /// Creates a new empty [`Corporeum`](Self).
    ///
    /// After modifying the [`Corpus`](Corpus), you can use [`save()`](Self::save)
    /// to save it into a file (`buffer`), which can be later loaded with [`load()`](Self::load).  
    pub fn new<P: AsRef<Path>>(buffer: P) -> Self {
        let corpus = Corpus::default();

        Self {
            original_file_path: buffer.as_ref().to_path_buf(),
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
    pub fn load<P: AsRef<Path>>(source: P) -> std::io::Result<Self> {
        let source = source.as_ref();
        let mut data = Vec::new();
        let file = fs::OpenOptions::new().read(true).open(source)?;

        {
            let mut decompressor = GzDecoder::new(file);
            decompressor.read_to_end(&mut data)?;
        }

        // TODO: Use safe api instead
        let archived = unsafe { rkyv::archived_root::<Corpus>(&data[..]) };
        let corpus = into_stderr!(archived.deserialize(&mut rkyv::Infallible))?;

        Ok(Self {
            original_file_path: source.to_path_buf(),
            corpus,
        })
    }

    /// Saves the current instance into the file from which the [`Corporeum`](Self)
    /// was created from.
    ///
    /// # Errors
    /// This method may return an error if:
    /// - The serialization fails
    /// - The destination file could not be opened
    /// - An error occurred while writing to the file.
    pub fn save(&self) -> std::io::Result<()> {
        self.save_as(&self.original_file_path)
    }

    /// Saves the current instance into the specified file.
    ///
    /// # Errors
    /// Same as [save()](Self::save).
    pub fn save_as<P: AsRef<Path>>(&self, path: &P) -> std::io::Result<()> {
        let buffer = rkyv::to_bytes::<_, 1024>(self.corpus()).unwrap();

        let dest = path.as_ref().with_extension(FILE_EXT);
        let file = create_file!(dest)?;
        let mut compressor = GzEncoder::new(file, Compression::best());
        compressor.write_all(&buffer)
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
