use std::sync::mpsc::Sender;

pub struct GnomeSorter;

use super::{send_message, SortResult, Sorter};

impl Sorter for GnomeSorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>) {
        send_message(&tx, values, &[]);

        let len = values.len();
        let mut pos = 0;
        while pos < len {
            if pos == 0 || values[pos] >= values[pos - 1] {
                pos += 1;
            } else {
                values.swap(pos, pos - 1);
                pos -= 1;
            }
            send_message(&tx, values, &[pos]);
        }

        send_message(&tx, values, &[]);
    }
}
