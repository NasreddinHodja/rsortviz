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

pub trait Sorter {
    fn new(values: &[usize]) -> Self;
    fn next_step(&mut self) -> Option<Vec<usize>>;
    fn current_state(&self) -> &[usize];
    fn used_indices(&self) -> Vec<usize>;
}

pub struct SelectionSorter {
    step_result: Vec<usize>,
    current_step: usize,
    inner_step: usize,
    min_index: usize,
}

impl Sorter for SelectionSorter {
    fn new(values: &[usize]) -> Self {
        Self {
            step_result: values.to_vec(),
            current_step: 0,
            inner_step: 1,
            min_index: 0,
        }
    }

    fn current_state(&self) -> &[usize] {
        &self.step_result
    }

    fn used_indices(&self) -> Vec<usize> {
        vec![self.current_step, self.inner_step, self.min_index]
    }

    fn next_step(&mut self) -> Option<Vec<usize>> {
        let len = self.step_result.len();

        if self.current_step >= len - 1 {
            return None;
        }

        if self.inner_step < len {
            if self.step_result[self.inner_step] < self.step_result[self.min_index] {
                self.min_index = self.inner_step;
            }
            self.inner_step += 1;
        } else {
            self.step_result.swap(self.current_step, self.min_index);
            self.current_step += 1;
            self.min_index = self.current_step;
            self.inner_step = self.current_step + 1;
            self.min_index = self.current_step;
        }

        Some(self.step_result.clone())
    }
}

pub struct BubbleSorter {
    step_result: Vec<usize>,
    current_step: usize,
    inner_step: usize,
    swapped: bool,
}

impl Sorter for BubbleSorter {
    fn new(values: &[usize]) -> Self {
        Self {
            step_result: values.to_vec(),
            current_step: 0,
            inner_step: 1,
            swapped: false,
        }
    }

    fn current_state(&self) -> &[usize] {
        &self.step_result
    }

    fn used_indices(&self) -> Vec<usize> {
        vec![self.inner_step, self.inner_step - 1]
    }

    fn next_step(&mut self) -> Option<Vec<usize>> {
        let len = self.step_result.len();
        if self.current_step >= len {
            return None;
        }

        if self.inner_step < len - self.current_step {
            let i = self.inner_step;
            let j = i - 1;
            if self.step_result[i] < self.step_result[j] {
                self.step_result.swap(i, j);
                self.swapped = true;
            }
            self.inner_step += 1;
        } else {
            if !self.swapped {
                return None;
            }
            self.current_step += 1;
            self.inner_step = 1;
            self.swapped = false;
        }
        Some(self.step_result.clone())
    }
}
