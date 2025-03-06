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
            "| Collection | BTreeSet   |\n| Operation  | Insert    |\n| Num of Items | {}       |\n| Time Taken | {:?}      |",
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
            "| Collection | BTreeSet   |\n| Operation  | Lookup    |\n| Num of Items | {}       |\n| Time Taken | {:?}      |",
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
            "| Collection | BTreeSet   |\n| Operation  | Delete    |\n| Num of Items | {}       |\n| Time Taken | {:?}      |",
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
