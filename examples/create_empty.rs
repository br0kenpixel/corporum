use corporum::Corporeum;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("test.corp");
    {
        let mut corp = Corporeum::new(&path);
        let mut doc = corp.corpus_mut().create_doc();
        let mut sent = doc.create_sentence("en");
        let tok = sent.create_token("hello");
        sent.add_token(tok);
        doc.add_sentence(sent);
        corp.corpus_mut().add_doc(doc);

        corp.save().unwrap();
    }

    {
        let _corp = Corporeum::load(&path).unwrap();
    }
}
