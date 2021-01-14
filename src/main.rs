use clap::Clap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "taku-ando",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    /// Sets the lebel of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        // Result.unwrap()
        // Okなら中の値を返して、Errならpanicを返す
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        // ファイルを指定しなかった場合
        println!("No file is specified.");
    }
}
