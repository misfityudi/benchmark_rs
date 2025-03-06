use std::time::Instant;

pub struct VecBenchmark {
    vec: Vec<i32>,
    num_of_items: i32,
}

impl VecBenchmark {
    pub fn new(num_of_items: i32) -> Self {
        Self {
            vec: Vec::<i32>::new(),
            num_of_items,
        }
    }

    pub fn benchmark_insert(&mut self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.vec.push(i);
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Vec       |\n| Operation  | Insert    |\n| Num of Items | {}       |\n| Time Taken | {:?}      |",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_lookup(&self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.vec[i as usize];
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Vec       |\n| Operation  | Lookup    |\n| Num of Items | {}       |\n| Time Taken | {:?}      |",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_delete(&mut self) {
        let start = Instant::now();

        for _ in 0..self.num_of_items {
            self.vec.pop();
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Vec       |\n| Operation  | Delete    |\n| Num of Items | {}       |\n| Time Taken | {:?}      |",
            self.num_of_items, duration
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_btreemap() {
        let mut benchmark = VecBenchmark::new(1_000);
        benchmark.benchmark_insert();
        benchmark.benchmark_lookup();
        benchmark.benchmark_delete();
    }
}
