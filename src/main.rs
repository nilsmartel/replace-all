use std::io::Write;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Config {
    /// Input file to use
    #[structopt()]
    file: String,

    /// String to find and replace
    #[structopt()]
    word: String,

    /// Replacement
    #[structopt()]
    replacement: String,
}

fn main() {
    let Config {
        file,
        word,
        replacement,
    } = Config::from_args();

    // open file
    let content = std::fs::read_to_string(&file).expect("to read file");

    if !content.contains(&replacement) {
        eprintln!("no occurence found in file {}", file);
        std::process::exit(0);
    }

    let content = content.replace(&word, &replacement);

    let mut file = std::fs::File::create(file).expect("overwrite file");

    write!(&mut file, "{}", content).expect("write content to file");
}
