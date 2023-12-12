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

fn send_message(tx: &Sender<Option<SortResult>>, values: &[usize], used_indices: &[usize]) {
    tx.send(Some(SortResult {
        values: values.to_vec(),
        used_indices: used_indices.to_vec(),
    }))
    .unwrap();
}

pub fn bubble_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    send_message(&tx, values, &[]);
    let len = values.len();
    let mut swapped;

    for i in 0..len {
        swapped = false;

        for j in 0..len - i - 1 {
            if values[j] > values[j + 1] {
                values.swap(j, j + 1);
                send_message(&tx, values, &[j, j + 1]);
            }
        }

        if !swapped {
            break;
        }
    }
    tx.send(None).unwrap();
}

pub fn insertion_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
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

    tx.send(None).unwrap();
}

pub fn selection_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    send_message(&tx, values, &[]);
    let len = values.len();

    for i in 0..len {
        let mut min_index = i;

        for j in (i + 1)..len {
            if values[j] < values[min_index] {
                min_index = j;
            }
            send_message(&tx, values, &[i, j, min_index]);
        }

        values.swap(i, min_index);
    }
}

pub fn merge_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    send_message(&tx, values, &[]);
    let len = values.len();
    if len <= 1 {
        return;
    }
    let mut temp = values.to_vec();

    merge_sort_helper(values, &mut temp, 0, len - 1, &tx);
}

fn merge_sort_helper(
    values: &mut [usize],
    temp: &mut [usize],
    start: usize,
    end: usize,
    tx: &Sender<Option<SortResult>>,
) {
    if start < end {
        let mid = (start + end) / 2;

        merge_sort_helper(values, temp, start, mid, &tx);
        merge_sort_helper(values, temp, mid + 1, end, &tx);

        merge(values, temp, start, mid, end, &tx);
    }
}

fn merge(
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

pub fn quicksort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
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
                send_message(&tx, values, &[low + i, low + j, pivot_index]);
                values.swap(i, j);
                i += 1;
            }
        }

        values.swap(i, high - 1);

        let pivot_index = i;

        stack.push((pivot_index + 1, high));
        stack.push((low, pivot_index));
    }

    send_message(&tx, values, &[]);
    tx.send(None).unwrap();
}

pub fn heap_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    send_message(&tx, values, &[]);

    for i in (0..values.len() / 2).rev() {
        max_heapify(values, i, values.len(), &tx);
    }

    for end in (1..values.len()).rev() {
        values.swap(0, end);
        max_heapify(values, 0, end, &tx);
        send_message(&tx, values, &[end]);
    }
}

fn max_heapify(values: &mut [usize], i: usize, heap_size: usize, tx: &Sender<Option<SortResult>>) {
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let mut largest = i;

    if left < heap_size && values[left] > values[largest] {
        largest = left;
        send_message(&tx, values, &[left, right, largest]);
    }

    if right < heap_size && values[right] > values[largest] {
        largest = right;
        send_message(&tx, values, &[left, right, largest]);
    }

    if largest != i {
        values.swap(i, largest);
        send_message(&tx, values, &[left, right, largest]);
        max_heapify(values, largest, heap_size, &tx);
    }
}

pub fn shell_sort(values: &mut [usize], tx: Sender<Option<SortResult>>) {
    let len = values.len();
    let mut gap = len / 2;
    send_message(&tx, values, &[gap]);

    while gap > 0 {
        for i in gap..len {
            let mut j = i;
            while j >= gap && values[j - gap] > values[j] {
                values.swap(j - gap, j);
                j -= gap;
                send_message(&tx, values, &[gap, i, j]);
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
        send_message(&tx, values, &[index]);
    }

    values.copy_from_slice(&output);
    send_message(&tx, values, &[]);
}
