use corporum::Corporeum;
use std::{env::args, process::exit};

fn main() {
    let args: Vec<String> = args().collect();

    let Some(file) = args.get(1) else {
        eprintln!("Please specify a file:");
        eprintln!("{} FILE", args[0]);
        exit(1);
    };

    let corp = Corporeum::load(file).unwrap();
    println!("{:?}", corp.corpus());
    //corp.save_as(&"test").unwrap();
}
