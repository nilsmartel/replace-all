use std::io::Write;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Config {
    /// String to find and replace
    #[structopt()]
    word: String,

    /// Replacement
    #[structopt()]
    replacement: String,

    /// One or more input files to use
    #[structopt()]
    files: Vec<String>,
}

fn main() {
    let Config {
        files,
        word,
        replacement,
    } = Config::from_args();

    // open file
    for file in files {
        let content = std::fs::read_to_string(&file);
        if content.is_err() {
            eprintln!("error: {:?}", content);
            continue;
        }

        let content = content.unwrap();

        if !content.contains(&replacement) {
            continue;
        }

        let content = content.replace(&word, &replacement);

        let mut file = std::fs::File::create(file).expect("overwrite file");

        write!(&mut file, "{}", content).expect("write content to file");
    }
}
