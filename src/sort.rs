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

pub struct BubbleSorter {
    values: Vec<usize>,
    step_result: Vec<usize>,
    current_step: usize,
    inner_step: usize,
    swapped: bool,
}

impl Sorter for BubbleSorter {
    fn new(values: &[usize]) -> Self {
        Self {
            values: values.to_vec(),
            step_result: values.to_vec(),
            current_step: 0,
            inner_step: 1,
            swapped: false,
        }
    }

    fn current_state(&self) -> &[usize] {
        &self.values
    }

    fn used_indices(&self) -> Vec<usize> {
        vec![self.inner_step, self.inner_step - 1]
    }

    fn next_step(&mut self) -> Option<Vec<usize>> {
        if self.current_step >= self.values.len() {
            return None;
        }

        if self.inner_step < self.values.len() - self.current_step {
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
        self.values = self.step_result.clone();
        Some(self.step_result.clone())
    }

    // fn next_step(&mut self) -> Option<Vec<usize>> {
    //     if self.current_step >= self.values.len() {
    //         return None;
    //     }

    //     let mut result = self.values.to_vec();
    //     let mut swapped = false;

    //     for i in 1..self.values.len() - self.current_step {
    //         if result[i] < result[i - 1] {
    //             result.swap(i, i - 1);
    //             swapped = true;
    //         }
    //     }

    //     if !swapped {
    //         return None;
    //     }

    //     self.values = result.clone();
    //     self.current_step += 1;
    //     Some(result)
    // }
}
