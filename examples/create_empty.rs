use corporum::Corporeum;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("test.corp");
    {
        let mut corp = Corporeum::new(&path);
        corp.corpus_mut().add_metadata("hello");
        corp.save().unwrap();
    }

    {
        let _corp = Corporeum::load(&path).unwrap();
    }
}
