use rand::Rng;

pub fn unsort(values: &[usize]) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut result = values.to_vec();

    for i in (1..result.len()).rev() {
        let j = rng.gen_range(0..=i);
        result.swap(i, j)
    }

    result
}

pub fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

pub trait Sorter {
    fn new(values: &[usize]) -> Self;
    fn next_step(&mut self) -> Option<Vec<usize>>;
    fn current_state(&self) -> &[usize];
    fn used_indices(&self) -> Vec<usize>;
}

pub struct InsertionSorter {
    step_result: Vec<usize>,
    i: usize,
    j: usize,
}

impl Sorter for InsertionSorter {
    fn new(values: &[usize]) -> Self {
        Self {
            step_result: values.to_vec(),
            i: 1,
            j: 1,
        }
    }

    fn current_state(&self) -> &[usize] {
        &self.step_result
    }

    fn used_indices(&self) -> Vec<usize> {
        vec![self.j]
    }

    fn next_step(&mut self) -> Option<Vec<usize>> {
        let len = self.step_result.len();

        if self.i >= len {
            return None;
        }

        if self.j > 0 && self.step_result[self.j - 1] > self.step_result[self.j] {
            self.step_result.swap(self.j - 1, self.j);
            self.j -= 1;
        } else {
            self.i += 1;
            self.j = self.i;
        }

        Some(self.step_result.clone())
    }
}

pub struct MergeSorter {
    step_result: Vec<usize>,
    left: Vec<usize>,
    right: Vec<usize>,
    current_size: usize,
    start: usize,
    mid: usize,
    end: usize,
    i: usize,
    j: usize,
    k: usize,
}

impl Sorter for MergeSorter {
    fn new(values: &[usize]) -> Self {
        let left = if values.len() == 1 {
            values.to_vec()
        } else {
            values[0..0].to_vec()
        };
        let right = if values.len() == 1 {
            values.to_vec()
        } else {
            values[0..1].to_vec()
        };
        Self {
            step_result: values.to_vec(),
            left,
            right,
            current_size: 1,
            start: 0,
            mid: 0,
            end: 1,
            i: 0,
            j: 0,
            k: 0,
        }
    }

    fn current_state(&self) -> &[usize] {
        &self.step_result
    }

    fn used_indices(&self) -> Vec<usize> {
        vec![self.start + self.i, self.mid + self.j, self.k]
    }

    fn next_step(&mut self) -> Option<Vec<usize>> {
        let len = self.step_result.len();

        if self.current_size > len - 1 {
            return None;
        }

        if self.start < len - 1 {
            if self.i < self.left.len() && self.j < self.right.len() {
                if self.left[self.i] < self.right[self.j] {
                    self.step_result[self.k] = self.left[self.i];
                    self.i += 1;
                } else {
                    self.step_result[self.k] = self.right[self.j];
                    self.j += 1;
                }
                self.k += 1;
            } else if self.i < self.left.len() {
                self.step_result[self.k] = self.left[self.i];
                self.i += 1;
                self.k += 1;
            } else if self.j < self.right.len() {
                self.step_result[self.k] = self.right[self.j];
                self.j += 1;
                self.k += 1;
            } else {
                self.start += 2 * self.current_size;

                if self.start >= len - 1 {
                    return Some(self.step_result.clone());
                }

                self.mid = min(self.start + self.current_size - 1, len - 1);
                self.end = min(self.start + 2 * self.current_size - 1, len - 1);

                self.left = (&self.step_result[self.start..self.mid + 1]).to_vec();
                self.right = (&self.step_result[self.mid + 1..self.end + 1]).to_vec();

                self.i = 0;
                self.j = 0;
                self.k = self.start;
            }
        } else {
            self.start = 0;
            self.current_size *= 2;

            self.i = 0;
            self.j = 0;
            self.k = self.start;

            self.mid = min(self.start + self.current_size - 1, len - 1);
            self.end = min(self.start + 2 * self.current_size - 1, len - 1);

            self.left = (&self.step_result[self.start..self.mid + 1]).to_vec();
            self.right = (&self.step_result[self.mid + 1..self.end + 1]).to_vec();
        }

        Some(self.step_result.clone())
    }
}

// pub fn merge_sort(arr: &mut [usize]) {
//     let len = arr.len();
//     let mut current_size = 1;
//     while current_size <= len - 1 {
//         let mut start = 0;
//         while start < len - 1 {
//             let mid = min(start + current_size - 1, len - 1);
//             let end = min(start + 2 * current_size - 1, len - 1);

//             let left = (&arr[start..mid + 1]).to_vec();
//             let right = (&arr[mid + 1..end + 1]).to_vec();

//             let mut i = 0;
//             let mut j = 0;
//             let mut k = start;

//             while i < left.len() && j < right.len() {
//                 if left[i] < right[j] {
//                     arr[k] = left[i];
//                     k += 1;
//                     i += 1;
//                 } else {
//                     arr[k] = right[j];
//                     k += 1;
//                     j += 1;
//                 }
//             }

//             while i < left.len() {
//                 arr[k] = left[i];
//                 k += 1;
//                 i += 1;
//             }

//             while j < right.len() {
//                 arr[k] = right[j];
//                 k += 1;
//                 j += 1;
//             }

//             start += 2 * current_size;
//         }

//         current_size *= 2;
//     }
// }

pub struct SelectionSorter {
    step_result: Vec<usize>,
    i: usize,
    j: usize,
    min_index: usize,
}

impl Sorter for SelectionSorter {
    fn new(values: &[usize]) -> Self {
        Self {
            step_result: values.to_vec(),
            i: 0,
            j: 1,
            min_index: 0,
        }
    }

    fn current_state(&self) -> &[usize] {
        &self.step_result
    }

    fn used_indices(&self) -> Vec<usize> {
        vec![self.i, self.j, self.min_index]
    }

    fn next_step(&mut self) -> Option<Vec<usize>> {
        let len = self.step_result.len();

        if self.i >= len - 1 {
            return None;
        }

        if self.j < len {
            if self.step_result[self.j] < self.step_result[self.min_index] {
                self.min_index = self.j;
            }
            self.j += 1;
        } else {
            self.step_result.swap(self.i, self.min_index);
            self.i += 1;
            self.min_index = self.i;
            self.j = self.i + 1;
            self.min_index = self.i;
        }

        Some(self.step_result.clone())
    }
}

pub struct BubbleSorter {
    step_result: Vec<usize>,
    i: usize,
    j: usize,
    swapped: bool,
}

impl Sorter for BubbleSorter {
    fn new(values: &[usize]) -> Self {
        Self {
            step_result: values.to_vec(),
            i: 0,
            j: 1,
            swapped: false,
        }
    }

    fn current_state(&self) -> &[usize] {
        &self.step_result
    }

    fn used_indices(&self) -> Vec<usize> {
        vec![self.j, self.j - 1]
    }

    fn next_step(&mut self) -> Option<Vec<usize>> {
        let len = self.step_result.len();
        if self.i >= len {
            return None;
        }

        if self.j < len - self.i {
            if self.step_result[self.j] < self.step_result[self.j - 1] {
                self.step_result.swap(self.j, self.j - 1);
                self.swapped = true;
            }
            self.j += 1;
        } else {
            if !self.swapped {
                return None;
            }
            self.i += 1;
            self.j = 1;
            self.swapped = false;
        }
        Some(self.step_result.clone())
    }
}
