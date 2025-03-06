use std::collections::HashSet;
use std::time::Instant;

pub struct HashSetBenchmark {
    hash_set: HashSet<i32>,
    num_of_items: i32,
}

impl HashSetBenchmark {
    pub fn new(num_of_items: i32) -> Self {
        Self {
            hash_set: HashSet::new(),
            num_of_items,
        }
    }

    pub fn benchmark_insert(&mut self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.hash_set.insert(i);
        }

        let duration = start.elapsed();
        println!(
            "| Collection | HashSet   |\n| Operation  | Insert    |\n| Num of Items | {}       |\n| Time Taken | {:?}      |",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_lookup(&self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.hash_set.get(&i);
        }

        let duration = start.elapsed();
        println!(
            "| Collection | HashSet   |\n| Operation  | Lookup    |\n| Num of Items | {}       |\n| Time Taken | {:?}      |",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_delete(&mut self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.hash_set.remove(&i);
        }

        let duration = start.elapsed();
        println!(
            "| Collection | HashSet   |\n| Operation  | Delete    |\n| Num of Items | {}       |\n| Time Taken | {:?}      |",
            self.num_of_items, duration
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_btreemap() {
        let mut benchmark = HashSetBenchmark::new(1_000);
        benchmark.benchmark_insert();
        benchmark.benchmark_lookup();
        benchmark.benchmark_delete();
    }
}
