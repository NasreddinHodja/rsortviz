use std::sync::mpsc::Sender;

pub struct InsertionSorter;

use super::{send_message, SortResult, Sorter};

impl Sorter for InsertionSorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>) {
        send_message(&tx, values, &[]);

        let len = values.len();
        for i in 1..len {
            let mut j = i;

            while j > 0 && values[j] < values[j - 1] {
                values.swap(j, j - 1);
                send_message(&tx, values, &[j, j - 1]);
                j -= 1;
            }
        }

        self.scan(values, &tx);
        send_message(&tx, values, &[]);
        tx.send(None).unwrap();
    }
}
