use std::sync::mpsc::Sender;

pub struct RadixSorter;

use super::{send_message, SortResult, Sorter};

impl Sorter for RadixSorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>) {
        if values.is_empty() {
            return;
        }

        let max_num = *values.iter().max().unwrap();
        let mut exp = 1;

        while max_num / exp > 0 {
            self.counting_sort(values, exp, &tx);
            exp *= 10;
        }
    }
}

impl RadixSorter {
    fn counting_sort(&self, values: &mut [usize], exp: usize, tx: &Sender<Option<SortResult>>) {
        let mut output = values.to_vec();
        let mut count = vec![0; 10];

        for &num in values.iter() {
            count[(num / exp) % 10] += 1;
        }

        for i in 1..10 {
            count[i] += count[i - 1];
        }

        for &num in values.iter().rev() {
            let index = (num / exp) % 10;
            output[count[index] - 1] = num;
            count[index] -= 1;
            send_message(&tx, &output, &[count[index]]);
        }

        values.copy_from_slice(&output);
        send_message(&tx, values, &[]);
    }
}
