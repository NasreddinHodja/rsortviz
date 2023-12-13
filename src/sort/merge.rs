use std::sync::mpsc::Sender;
pub struct MergeSorter;

use super::{send_message, SortResult, Sorter};

impl Sorter for MergeSorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>) {
        send_message(&tx, values, &[]);
        let len = values.len();
        if len <= 1 {
            return;
        }
        let mut temp = values.to_vec();

        self.merge_sort(values, &mut temp, 0, len - 1, &tx);
    }
}

impl MergeSorter {
    fn merge_sort(
        &self,
        values: &mut [usize],
        temp: &mut [usize],
        start: usize,
        end: usize,
        tx: &Sender<Option<SortResult>>,
    ) {
        if start < end {
            let mid = (start + end) / 2;

            self.merge_sort(values, temp, start, mid, &tx);
            self.merge_sort(values, temp, mid + 1, end, &tx);

            self.merge(values, temp, start, mid, end, &tx);
        }
    }

    fn merge(
        &self,
        values: &mut [usize],
        temp: &mut [usize],
        start: usize,
        mid: usize,
        end: usize,
        tx: &Sender<Option<SortResult>>,
    ) {
        let (mut i, mut j, mut k) = (start, mid + 1, start);

        while i <= mid && j <= end {
            if values[i] <= values[j] {
                temp[k] = values[i];
                i += 1;
            } else {
                temp[k] = values[j];
                j += 1;
            }
            k += 1;
            send_message(&tx, temp, &[i, j, k]);
        }

        while i <= mid {
            temp[k] = values[i];
            i += 1;
            k += 1;
            send_message(&tx, temp, &[i, j, k]);
        }

        while j <= end {
            temp[k] = values[j];
            j += 1;
            k += 1;
            send_message(&tx, temp, &[i, j, k]);
        }

        for i in start..=end {
            values[i] = temp[i];
        }
    }
}
