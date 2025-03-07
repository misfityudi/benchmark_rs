use std::collections::BTreeSet;
use std::time::Instant;

pub struct BTreeSetBenchmark {
    btree_set: BTreeSet<i32>,
    num_of_items: i32,
}

impl BTreeSetBenchmark {
    pub fn new(num_of_items: i32) -> Self {
        Self {
            btree_set: BTreeSet::new(),
            num_of_items,
        }
    }

    pub fn benchmark_insert(&mut self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.btree_set.insert(i);
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Operation | Num of Items | Time Taken |\n|------------|-----------|--------------|------------|\n| BTreeSet        | Insert    | {}           | {:?}       |",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_lookup(&self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.btree_set.get(&i);
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Operation | Num of Items | Time Taken |\n|------------|-----------|--------------|------------|\n| BTreeSet        | Lookup    | {}           | {:?}       |",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_delete(&mut self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.btree_set.remove(&i);
        }

        let duration = start.elapsed();
        println!(
            "| Collection | Operation | Num of Items | Time Taken |\n|------------|-----------|--------------|------------|\n| BTreeSet        | Delete    | {}           | {:?}       |",
            self.num_of_items, duration
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_btreemap() {
        let mut benchmark = BTreeSetBenchmark::new(1_000);
        benchmark.benchmark_insert();
        benchmark.benchmark_lookup();
        benchmark.benchmark_delete();
    }
}
