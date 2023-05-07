use corporum::Corporeum;
use std::{env::args, path::PathBuf, process::exit};

fn main() {
    let args: Vec<String> = args().collect();

    let Some(file) = args.get(1) else {
        eprintln!("Please specify a file:");
        eprintln!("{} FILE", args[0]);
        exit(1);
    };

    let _corp = Corporeum::load(&PathBuf::from(file)).unwrap();
}
