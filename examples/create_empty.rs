use corporum::Corporeum;

fn main() {
    const PATH: &str = "test";
    const LOAD_PATH: &str = "test.rkyv.lzma";
    {
        let mut corp = Corporeum::new(PATH);
        let mut doc = corp.corpus_mut().create_doc();
        let mut sent = doc.create_sentence("en");
        let tok = sent.create_token("hello");
        sent.add_token(tok);
        doc.add_sentence(sent);
        corp.corpus_mut().add_doc(doc);

        corp.save().unwrap();
    }

    {
        let _corp = Corporeum::load(LOAD_PATH).unwrap();
    }
}
