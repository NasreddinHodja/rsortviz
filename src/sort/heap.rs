use std::sync::mpsc::Sender;

pub struct HeapSorter;

use super::{send_message, SortResult, Sorter};

impl Sorter for HeapSorter {
    fn sort(&self, values: &mut [usize], tx: Sender<Option<SortResult>>) {
        send_message(&tx, values, &[]);

        for i in (0..values.len() / 2).rev() {
            self.max_heapify(values, i, values.len(), &tx);
        }

        for end in (1..values.len()).rev() {
            values.swap(0, end);
            self.max_heapify(values, 0, end, &tx);
            send_message(&tx, values, &[end]);
        }

        self.scan(values, &tx);
        send_message(&tx, values, &[]);
        tx.send(None).unwrap();
    }
}

impl HeapSorter {
    fn max_heapify(
        &self,
        values: &mut [usize],
        i: usize,
        heap_size: usize,
        tx: &Sender<Option<SortResult>>,
    ) {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut largest = i;

        if left < heap_size && values[left] > values[largest] {
            largest = left;
            send_message(&tx, values, &[left, right, largest, i]);
        }

        if right < heap_size && values[right] > values[largest] {
            largest = right;
            send_message(&tx, values, &[left, right, largest, i]);
        }

        if largest != i {
            values.swap(i, largest);
            send_message(&tx, values, &[left, right, largest, i]);
            self.max_heapify(values, largest, heap_size, &tx);
        }
    }
}
