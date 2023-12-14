use std::sync::mpsc::Sender;

pub struct BubbleSorter;

use super::{send_message, SortResult, Sorter};

impl Sorter for BubbleSorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>) {
        send_message(&tx, values, &[]);
        let len = values.len();
        let mut swapped;

        for i in 0..len {
            swapped = false;

            for j in 0..len - i - 1 {
                if values[j] > values[j + 1] {
                    values.swap(j, j + 1);
                    swapped = true;
                }
                send_message(&tx, values, &[j, j + 1]);
            }

            if !swapped {
                break;
            }
        }
        send_message(&tx, values, &[]);
    }
}
