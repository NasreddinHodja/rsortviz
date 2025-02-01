fn generate_scale_frequencies(floor: f64, ceil: f64, intervals: &[usize]) -> Vec<f64> {
    let mut frequencies = Vec::new();
    let mut curr_freq = floor;

    while curr_freq <= ceil {
        frequencies.push(curr_freq);
        for &interval in intervals {
            curr_freq *= 2f64.powf(interval as f64 / 12.0);

            if curr_freq > ceil {
                break;
            }

            frequencies.push(curr_freq);
        }
    }

    frequencies
}

#[derive(Debug)]
pub struct Scale {
    // intervals: &'a [usize],
    // name: &'a str,
    // floor: f64,
    // ceil: f64,
    frequencies: Vec<f64>,
}

impl Scale {
    pub fn new(name: &str, floor: f64, ceil: f64) -> Self {
        let intervals = match name {
            "major" => &[2, 2, 1, 2, 2, 2, 1],
            "minor" => &[2, 1, 2, 2, 1, 2, 2],
            "dorian" => &[2, 1, 2, 2, 2, 1, 2],
            "phrygian" => &[1, 2, 2, 2, 1, 2, 2],
            "lydian" => &[2, 2, 2, 1, 2, 2, 1],
            "mixolydian" => &[2, 2, 1, 2, 2, 1, 2],
            "locrian" => &[1, 2, 2, 1, 2, 2, 2],
            _ => panic!("Please provide one of the available scales."),
        };
        let frequencies = generate_scale_frequencies(floor, ceil, intervals);

        Self { frequencies }
    }

    pub fn frequencies(&self) -> &[f64] {
        &self.frequencies
    }

    pub fn frequency(&self, index: usize) -> f64 {
        assert!(index < self.frequencies.len());
        self.frequencies[index]
    }
}
