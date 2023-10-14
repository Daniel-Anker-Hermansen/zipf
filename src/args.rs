use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(name = "Output file")]
    pub output: String,

    #[arg(name = "Input files", help = "Files to include. Folders are included recursively")]
    pub inputs: Vec<String>,

    #[arg(short = 'x', long = "exclude", name = "Exclude", help = "Excludes a file path or folder")]
    pub excludes: Vec<String>,
}
