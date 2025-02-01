use clap::Parser;

use crate::{
    sort::{
        BubbleSorter, CycleSorter, GnomeSorter, HeapSorter, InsertionSorter, MergeSorter,
        QuickSorter, RadixSorter, SelectionSorter, ShellSorter, Sorter,
    },
    sound::Scale,
};

const MAX_LEN: usize = 500;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, default_value_t = String::from("quick"))]
    algorithm: String,

    #[arg(short, long, default_value_t = 50)]
    length: usize,

    #[arg(short, long)]
    scale: Option<String>,
}

pub struct Args {
    pub sorter: Box<dyn Sorter>,
    pub scale: Option<Scale>,
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
        "gnome" => Box::new(GnomeSorter),
        "cycle" => Box::new(CycleSorter),
        _ => panic!("Please provide one of the available algorithms."),
    };

    let freq_floor = 27.5;
    let freq_ceil = 4186.0;
    let scale: Option<Scale> = match cli.scale.as_deref() {
        Some("minor") => Some(Scale::new("minor", freq_floor, freq_ceil)),
        Some("major") => Some(Scale::new("major", freq_floor, freq_ceil)),
        Some("dorian") => Some(Scale::new("dorian", freq_floor, freq_ceil)),
        Some("phrygian") => Some(Scale::new("phrygian", freq_floor, freq_ceil)),
        Some("lydian") => Some(Scale::new("lydian", freq_floor, freq_ceil)),
        Some("mixolydian") => Some(Scale::new("mixolydian", freq_floor, freq_ceil)),
        Some("locrian") => Some(Scale::new("locrian", freq_floor, freq_ceil)),
        Some(_) => panic!("Please provide one of the available scales."),
        None => None,
    };

    Args {
        sorter,
        scale,
        length: cli.length.to_owned(),
    }
}
