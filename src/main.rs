use std::path::Path;

use clap::Parser;

use rind::{
    self,
    ExtensionFilter,
    SizeFilter,
    FileSystem,
    Filter,
};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// (Required) Path to the directory you're searching in
    #[arg(short, long)]
    path: String,

    /// (Optional) The file extension you are searching for
    #[arg(short, long)]
    ext: Option<String>,

    /// (Optional) Return only files whose size is greater than this value
    #[arg(short, long)]
    size: Option<u64>,
}

fn parse_filter_args(args: &Args) -> Vec<Box<dyn Filter>> {
    let mut filters: Vec<Box<dyn Filter>> = vec![];

    match &args.ext {
        Some(v) => {
            let filter = Box::new(ExtensionFilter::new(v.to_string()));
            filters.push(filter);
        },
        _ => ()
    }

    match &args.size {
        Some(v) => {
            let filter = Box::new(SizeFilter::new(v.to_owned()));
            filters.push(filter);
        },
        _ => ()
    }

    filters
}

fn main() {
    let args = Args::parse();

    let filters = parse_filter_args(&args);

    let fs = FileSystem::build(&Path::new(&args.path));

    for ele in fs.filter(&filters) {
        println!("{ele}")
    };

}
