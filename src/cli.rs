use clap::Parser;

use crate::sort::{
    BubbleSorter, HeapSorter, InsertionSorter, MergeSorter, QuickSorter, RadixSorter,
    SelectionSorter, ShellSorter, Sorter,
};

const MAX_LEN: usize = 500;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    algorithm: String,

    #[arg(short, long)]
    length: usize,
}

pub struct Args {
    pub sorter: Box<dyn Sorter>,
    pub length: usize,
}

pub fn parse() -> Args {
    let cli = CliArgs::parse();

    assert!(cli.length <= MAX_LEN);

    let sorter: Box<dyn Sorter> = match cli.algorithm.as_str() {
        "bubble" => Box::new(BubbleSorter),
        "insertion" => Box::new(InsertionSorter),
        "selection" => Box::new(SelectionSorter),
        "merge" => Box::new(MergeSorter),
        "quick" => Box::new(QuickSorter),
        "heap" => Box::new(HeapSorter),
        "shell" => Box::new(ShellSorter),
        "radix" => Box::new(RadixSorter),
        _ => panic!("Please provide one of the available algorithms."),
    };

    Args {
        sorter,
        length: cli.length,
    }
}
