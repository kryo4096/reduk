use spin::Mutex;

const NUMBERS: &'static [u32] = &[
    10, 33, 10, 19, 99, 43, 95, 63, 35, 84, 28, 37, 78, 14, 36, 97, 40, 95, 83, 93, 98, 40, 9, 75,
    61, 35, 32, 86, 84, 96, 94, 46, 79, 30, 95, 40, 6, 27, 83, 28, 2, 20, 40, 27, 53, 52, 90, 91,
    60, 55, 11, 68, 55, 94, 58, 80, 63, 42, 56, 28, 88, 59, 99, 50, 89, 13, 90, 63, 22, 14, 63, 36,
    71, 34, 52, 49, 20, 97, 69, 35, 84, 45, 100, 34, 44, 98, 18, 63, 71, 80, 53, 31, 15, 7, 79, 75,
    40, 13, 72, 39,
];

pub struct Random {
    numbers: &'static [u32],
    counter: usize,
    steps: usize,
}

impl Random {
    pub fn new(numbers: &'static [u32], steps: usize) -> Self {
        assert!(numbers.len() > steps);

        Self {
            numbers,
            counter: 0,
            steps,
        }
    }

    pub fn next(&mut self) -> u32 {
        let mut sum: u32 = 0;

        for i in 0..self.steps {
            sum += self.numbers[self.counter % (self.numbers.len() - i)];
        }

        self.counter += 1;

        sum / self.steps as u32
    }
}

lazy_static! {
    pub static ref RNG: Mutex<Random> = Mutex::new(Random::new(NUMBERS, 1));
}
