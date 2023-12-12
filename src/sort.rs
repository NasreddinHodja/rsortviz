use rand::Rng;
use std::sync::mpsc::Sender;

pub fn unsort(values: &[usize]) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut result = values.to_vec();

    for i in (1..result.len()).rev() {
        let j = rng.gen_range(0..=i);
        result.swap(i, j)
    }

    result
}

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

#[derive(Debug)]
pub struct SortResult {
    pub values: Vec<usize>,
    pub used_indices: Vec<usize>,
}

pub fn bubble_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: vec![0],
    }))
    .unwrap();

    let len = values.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if values[j] > values[j + 1] {
                values.swap(j, j + 1);
                tx.send(Some(SortResult {
                    values: values.to_vec(),
                    used_indices: vec![i, j],
                }))
                .unwrap();
            }
        }
    }

    tx.send(None).unwrap();
}

pub fn insertion_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: vec![0],
    }))
    .unwrap();

    let len = values.len();
    for i in 1..len {
        let mut j = i;

        while j > 0 && values[j] < values[j - 1] {
            values.swap(j, j - 1);
            tx.send(Some(SortResult {
                values: values.to_vec(),
                used_indices: vec![j, j - 1],
            }))
            .unwrap();
            j -= 1;
        }
    }

    tx.send(None).unwrap();
}

pub fn selection_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: vec![0],
    }))
    .unwrap();
    let len = values.len();

    for i in 0..len {
        // Assume the current index is the minimum
        let mut min_index = i;

        // Find the index of the minimum element in the unsorted part
        for j in (i + 1)..len {
            if values[j] < values[min_index] {
                min_index = j;
            }
            tx.send(Some(SortResult {
                values: values.to_vec(),
                used_indices: vec![i, j, min_index],
            }))
            .unwrap();
        }

        // Swap the found minimum element with the element at index i
        values.swap(i, min_index);
    }
}

pub fn merge_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: vec![0],
    }))
    .unwrap();

    let len = values.len();
    let mut current_size = 1;
    while current_size <= len - 1 {
        let mut start = 0;
        while start < len - 1 {
            let mid = min(start + current_size - 1, len - 1);
            let end = min(start + 2 * current_size - 1, len - 1);

            let left = (&values[start..mid + 1]).to_vec();
            let right = (&values[mid + 1..end + 1]).to_vec();

            let mut i = 0;
            let mut j = 0;
            let mut k = start;

            while i < left.len() && j < right.len() {
                if left[i] < right[j] {
                    values[k] = left[i];
                    tx.send(Some(SortResult {
                        values: values.to_vec(),
                        used_indices: vec![i, j, k],
                    }))
                    .unwrap();
                    k += 1;
                    i += 1;
                } else {
                    values[k] = right[j];
                    tx.send(Some(SortResult {
                        values: values.to_vec(),
                        used_indices: vec![i, j, k],
                    }))
                    .unwrap();
                    k += 1;
                    j += 1;
                }
            }

            while i < left.len() {
                values[k] = left[i];
                tx.send(Some(SortResult {
                    values: values.to_vec(),
                    used_indices: vec![i, j, k],
                }))
                .unwrap();
                k += 1;
                i += 1;
            }

            while j < right.len() {
                values[k] = right[j];
                tx.send(Some(SortResult {
                    values: values.to_vec(),
                    used_indices: vec![i, j, k],
                }))
                .unwrap();
                k += 1;
                j += 1;
            }

            start += 2 * current_size;
        }

        current_size *= 2;
    }

    tx.send(None).unwrap();
}

pub fn quicksort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: vec![],
    }))
    .unwrap();
    let mut stack = vec![(0, values.len())];

    while let Some((lo, hi)) = stack.pop() {
        if hi - lo <= 1 {
            continue;
        }
        let pivot_index = lo + (hi - lo) / 2;
        values.swap(pivot_index, hi - 1);

        let pivot = values[hi - 1];
        let mut i = lo;

        for j in lo..hi - 1 {
            if values[j] <= pivot {
                tx.send(Some(SortResult {
                    values: values.to_vec(),
                    used_indices: vec![i, j, pivot_index],
                }))
                .unwrap();
                values.swap(i, j);
                i += 1;
            }
        }

        values.swap(i, hi - 1);

        let pivot_index = i;

        stack.push((pivot_index + 1, hi));
        stack.push((lo, pivot_index));
    }
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: vec![],
    }))
    .unwrap();
    tx.send(None).unwrap();
}
