use std::sync::mpsc::Sender;

pub struct SelectionSorter;

use super::{send_message, SortResult, Sorter};

impl Sorter for SelectionSorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>) {
        send_message(&tx, values, &[]);
        let len = values.len();

        for i in 0..len {
            let mut min_index = i;

            for j in (i + 1)..len {
                if values[j] < values[min_index] {
                    min_index = j;
                }
                send_message(&tx, values, &[i, j, min_index]);
            }

            values.swap(i, min_index);
        }
    }
}
