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
    fn is_sorted(&self) -> bool;
}

pub struct BubbleSorter {
    values: Vec<usize>,
    current_step: usize,
}

impl Sorter for BubbleSorter {
    fn new(values: &[usize]) -> Self {
        Self {
            values: values.to_vec(),
            current_step: 0,
        }
    }

    fn current_state(&self) -> &[usize] {
        &self.values
    }

    fn next_step(&mut self) -> Option<Vec<usize>> {
        if self.current_step >= self.values.len() {
            return None;
        }

        let mut result = self.values.to_vec();
        let mut swapped = false;

        for i in 1..self.values.len() - self.current_step {
            if result[i] < result[i - 1] {
                result.swap(i, i - 1);
                swapped = true;
            }
        }

        if !swapped {
            return None;
        }

        self.values = result.clone();
        self.current_step += 1;
        Some(result)
    }

    fn is_sorted(&self) -> bool {
        true
    }
}
