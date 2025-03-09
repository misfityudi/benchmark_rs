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
            "| Collection | Operation | Num of Items | Time Taken |\n|------------|-----------|--------------|------------|\n| Vec        | Insert    | {}           | {:?}       |",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_lookup(&mut self) {
        if self.vec.is_empty() {
            for i in 0..self.num_of_items {
                self.vec.push(i);
            }
        }

        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.vec[i as usize];
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Operation | Num of Items | Time Taken |\n|------------|-----------|--------------|------------|\n| Vec        | Lookup    | {}           | {:?}       |",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_delete(&mut self) {
        if self.vec.is_empty() {
            for i in 0..self.num_of_items {
                self.vec.push(i);
            }
        }

        let start = Instant::now();

        for _ in 0..self.num_of_items {
            self.vec.pop();
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Operation | Num of Items | Time Taken |\n|------------|-----------|--------------|------------|\n| Vec        | Delete    | {}           | {:?}       |",
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
