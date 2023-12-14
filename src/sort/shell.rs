use std::sync::mpsc::Sender;

pub struct ShellSorter;

use super::{send_message, SortResult, Sorter};

impl Sorter for ShellSorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>) {
        let len = values.len();
        let mut gap = len / 2;
        send_message(&tx, values, &[gap]);

        while gap > 0 {
            for i in gap..len {
                let mut j = i;
                while j >= gap && values[j - gap] > values[j] {
                    values.swap(j - gap, j);
                    send_message(&tx, values, &[j, j - gap]);
                    j -= gap;
                }
            }

            gap /= 2;
        }

        tx.send(None).unwrap();
    }
}
