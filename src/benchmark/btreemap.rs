use std::collections::BTreeMap;
use std::time::Instant;

pub struct BTreeMapBenchmark {
    btree_map: BTreeMap<i32, i32>,
    num_of_items: i32,
}

impl BTreeMapBenchmark {
    pub fn new(num_of_items: i32) -> Self {
        Self {
            btree_map: BTreeMap::new(),
            num_of_items,
        }
    }

    pub fn benchmark_insert(&mut self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.btree_map.insert(i, i);
        }

        let duration = start.elapsed();
        println!(
            "Time taken to insert {} items into BTreeMap: {:?}",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_lookup(&self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.btree_map.get(&i);
        }

        let duration = start.elapsed();
        println!(
            "Time taken to lookup {} items in BTreeMap: {:?}",
            self.num_of_items, duration
        );
    }

    pub fn benchmark_delete(&mut self) {
        let start = Instant::now();

        for i in 0..self.num_of_items {
            self.btree_map.remove(&i);
        }

        let duration = start.elapsed();
        println!(
            "Time taken to delete {} items from BTreeMap: {:?}",
            self.num_of_items, duration
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_btreemap() {
        let mut benchmark = BTreeMapBenchmark::new(1_000);
        benchmark.benchmark_insert();
        benchmark.benchmark_lookup();
        benchmark.benchmark_delete();
    }
}
