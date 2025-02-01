use std::sync::mpsc::Sender;

pub struct QuickSorter;

use super::{send_message, SortResult, Sorter};

impl Sorter for QuickSorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>) {
        send_message(&tx, values, &[]);
        let mut stack = vec![(0, values.len())];

        while let Some((low, high)) = stack.pop() {
            if high - low <= 1 {
                continue;
            }
            let pivot_index = low + (high - low) / 2;
            values.swap(pivot_index, high - 1);

            let pivot = values[high - 1];
            let mut i = low;

            for j in low..high - 1 {
                if values[j] <= pivot {
                    send_message(&tx, values, &[i, j, pivot_index]);
                    values.swap(i, j);
                    i += 1;
                }
            }

            values.swap(i, high - 1);

            let pivot_index = i;

            stack.push((pivot_index + 1, high));
            stack.push((low, pivot_index));
        }

        self.scan(values, &tx);
        send_message(&tx, values, &[]);
        tx.send(None).unwrap();
    }
}
