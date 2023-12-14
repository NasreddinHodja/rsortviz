use rand::Rng;
use std::sync::mpsc::Sender;

pub struct SortResult {
    pub values: Vec<usize>,
    pub used_indices: Vec<usize>,
}

pub trait Sorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>);
}

pub fn unsort(values: &[usize]) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut result = values.to_vec();

    for i in (1..result.len()).rev() {
        let j = rng.gen_range(0..=i);
        result.swap(i, j)
    }

    result
}

fn send_message(tx: &Sender<Option<SortResult>>, values: &[usize], used_indices: &[usize]) {
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: used_indices.to_vec(),
    }))
    .unwrap();
}

mod bubble;
mod gnome;
mod heap;
mod insertion;
mod merge;
mod quick;
mod radix;
mod selection;
mod shell;
pub use bubble::BubbleSorter;
pub use gnome::GnomeSorter;
pub use heap::HeapSorter;
pub use insertion::InsertionSorter;
pub use merge::MergeSorter;
pub use quick::QuickSorter;
pub use radix::RadixSorter;
pub use selection::SelectionSorter;
pub use shell::ShellSorter;
