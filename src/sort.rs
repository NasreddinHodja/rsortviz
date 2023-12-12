#![allow(dead_code)]

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
                    used_indices: vec![i, j, j + 1],
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
        let mut min_index = i;

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
                        used_indices: vec![i + start, j + mid, k],
                    }))
                    .unwrap();
                    k += 1;
                    i += 1;
                } else {
                    values[k] = right[j];
                    tx.send(Some(SortResult {
                        values: values.to_vec(),
                        used_indices: vec![i + start, j + mid, k],
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
                    used_indices: vec![i + start, j + mid, k],
                }))
                .unwrap();
                k += 1;
                i += 1;
            }

            while j < right.len() {
                values[k] = right[j];
                tx.send(Some(SortResult {
                    values: values.to_vec(),
                    used_indices: vec![i + start, j + mid, k],
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

pub fn heap_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: vec![],
    }))
    .unwrap();

    for i in (0..values.len() / 2).rev() {
        max_heapify(values, i, values.len(), &tx);
    }

    for end in (1..values.len()).rev() {
        values.swap(0, end);
        max_heapify(values, 0, end, &tx);
        tx.send(Some(SortResult {
            values: values.to_vec(),
            used_indices: vec![end],
        }))
        .unwrap();
    }
}

fn max_heapify(values: &mut [usize], i: usize, heap_size: usize, tx: &Sender<Option<SortResult>>) {
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let mut largest = i;

    if left < heap_size && values[left] > values[largest] {
        largest = left;
        tx.send(Some(SortResult {
            values: values.to_vec(),
            used_indices: vec![left, right, largest],
        }))
        .unwrap();
    }

    if right < heap_size && values[right] > values[largest] {
        largest = right;
        tx.send(Some(SortResult {
            values: values.to_vec(),
            used_indices: vec![left, right, largest],
        }))
        .unwrap();
    }

    if largest != i {
        values.swap(i, largest);
        tx.send(Some(SortResult {
            values: values.to_vec(),
            used_indices: vec![left, right, largest],
        }))
        .unwrap();
        max_heapify(values, largest, heap_size, &tx);
    }
}

pub fn shell_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    let len = values.len();
    let mut gap = len / 2;
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: vec![gap],
    }))
    .unwrap();

    while gap > 0 {
        for i in gap..len {
            let mut j = i;
            while j >= gap && values[j - gap] > values[j] {
                values.swap(j - gap, j);
                j -= gap;
                tx.send(Some(SortResult {
                    values: values.to_vec(),
                    used_indices: vec![gap, i, j],
                }))
                .unwrap();
            }
        }

        gap /= 2;
    }

    tx.send(None).unwrap();
}

pub fn radix_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    if values.is_empty() {
        return;
    }

    let max_num = *values.iter().max().unwrap();
    let mut exp = 1;

    while max_num / exp > 0 {
        counting_sort(values, exp, &tx);
        exp *= 10;
    }
}

fn counting_sort(values: &mut [usize], exp: usize, tx: &Sender<Option<SortResult>>) {
    let mut output = vec![0; values.len()];
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
        tx.send(Some(SortResult {
            values: output.to_vec(),
            used_indices: vec![index],
        }))
        .unwrap();
    }

    values.copy_from_slice(&output);
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: vec![],
    }))
    .unwrap();
}
