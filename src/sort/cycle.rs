use std::sync::mpsc::Sender;

pub struct CycleSorter;

use super::{send_message, SortResult, Sorter};

impl Sorter for CycleSorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>) {
        send_message(&tx, values, &[]);

        let len = values.len();

        for cycle_start in 0..=len - 2 {
            let mut item = values[cycle_start];
            let mut pos = cycle_start;

            for i in cycle_start + 1..len {
                if values[i] < item {
                    pos += 1;
                }
                send_message(&tx, values, &[cycle_start, i, pos]);
            }

            if pos == cycle_start {
                continue;
            }

            while item == values[pos] {
                pos += 1;
                send_message(&tx, values, &[cycle_start, pos]);
            }

            (item, values[pos]) = (values[pos], item);
            send_message(&tx, values, &[cycle_start, pos]);

            while pos != cycle_start {
                pos = cycle_start;

                for i in cycle_start + 1..len {
                    if values[i] < item {
                        pos += 1;
                        send_message(&tx, values, &[cycle_start, i, pos]);
                    }
                }

                while item == values[pos] {
                    pos += 1;
                    send_message(&tx, values, &[cycle_start, pos]);
                }

                (item, values[pos]) = (values[pos], item);
                send_message(&tx, values, &[cycle_start, pos]);
            }
        }

        send_message(&tx, values, &[]);
    }
}
