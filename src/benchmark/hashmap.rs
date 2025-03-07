use std::collections::HashMap;
use std::time::Instant;

pub struct HashMapBenchmark {
    hash_map: HashMap<i32, i32>,
    num_of_items: i32,
}

impl HashMapBenchmark {
    pub fn new(num_of_items: i32) -> Self {
        Self {
            hash_map: HashMap::new(),
            num_of_items,
        }
    }

    pub fn benchmark_insert(&mut self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.hash_map.insert(i, i);
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Operation | Num of Items | Time Taken |\n|------------|-----------|--------------|------------|\n| HashMap        | Insert    | {}           | {:?}       |",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_lookup(&self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.hash_map.get(&i);
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Operation | Num of Items | Time Taken |\n|------------|-----------|--------------|------------|\n| HashMap        | Lookup    | {}           | {:?}       |",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_delete(&mut self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.hash_map.remove(&i);
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Operation | Num of Items | Time Taken |\n|------------|-----------|--------------|------------|\n| HashMap        | Delete    | {}           | {:?}       |",
            self.num_of_items, duration
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_btreemap() {
        let mut benchmark = HashMapBenchmark::new(1_000);
        benchmark.benchmark_insert();
        benchmark.benchmark_lookup();
        benchmark.benchmark_delete();
    }
}
