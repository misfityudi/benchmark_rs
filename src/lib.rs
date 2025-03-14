pub mod benchmark;
pub mod definitions;
use definitions::{Collection, Operation};

#[derive(Debug)]
pub struct Target {
    pub operation: Operation,
    pub collection: Collection,
    pub num_of_items: i32,
}

impl Target {
    pub fn new(
        collection: Option<Collection>,
        operation: Option<Operation>,
        num_of_items: Option<i32>,
    ) -> Self {
        Self {
            collection: collection.unwrap_or(Collection::Vec),
            operation: operation.unwrap_or(Operation::Insert),
            num_of_items: num_of_items.unwrap_or(1_000_000),
        }
    }

    pub fn benchmark(&self) {
        match (&self.collection, &self.operation) {
            (Collection::Vec, Operation::Insert) => {
                let mut vec_benchmark = benchmark::vec::VecBenchmark::new(self.num_of_items);
                vec_benchmark.benchmark_insert();
            }
            (Collection::Vec, Operation::Lookup) => {
                let mut vec_benchmark = benchmark::vec::VecBenchmark::new(self.num_of_items);
                vec_benchmark.benchmark_lookup();
            }
            (Collection::Vec, Operation::Delete) => {
                let mut vec_benchmark = benchmark::vec::VecBenchmark::new(self.num_of_items);
                vec_benchmark.benchmark_delete();
            }
            (Collection::BTreeMap, Operation::Insert) => {
                let mut btreemap_benchmark =
                    benchmark::btreemap::BTreeMapBenchmark::new(self.num_of_items);
                btreemap_benchmark.benchmark_insert();
            }
            (Collection::BTreeMap, Operation::Lookup) => {
                let btreemap_benchmark =
                    benchmark::btreemap::BTreeMapBenchmark::new(self.num_of_items);
                btreemap_benchmark.benchmark_lookup();
            }
            (Collection::BTreeMap, Operation::Delete) => {
                let mut btreemap_benchmark =
                    benchmark::btreemap::BTreeMapBenchmark::new(self.num_of_items);
                btreemap_benchmark.benchmark_delete();
            }
            (Collection::HashMap, Operation::Insert) => {
                let mut hashmap_benchmark =
                    benchmark::hashmap::HashMapBenchmark::new(self.num_of_items);
                hashmap_benchmark.benchmark_insert();
            }
            (Collection::HashMap, Operation::Lookup) => {
                let hashmap_benchmark =
                    benchmark::hashmap::HashMapBenchmark::new(self.num_of_items);
                hashmap_benchmark.benchmark_lookup();
            }
            (Collection::HashMap, Operation::Delete) => {
                let mut hashmap_benchmark =
                    benchmark::hashmap::HashMapBenchmark::new(self.num_of_items);
                hashmap_benchmark.benchmark_delete();
            }
            (Collection::BTreeSet, Operation::Insert) => {
                let mut btreeset_benchmark =
                    benchmark::btreeset::BTreeSetBenchmark::new(self.num_of_items);
                btreeset_benchmark.benchmark_insert();
            }
            (Collection::BTreeSet, Operation::Lookup) => {
                let btreeset_benchmark =
                    benchmark::btreeset::BTreeSetBenchmark::new(self.num_of_items);
                btreeset_benchmark.benchmark_lookup();
            }
            (Collection::BTreeSet, Operation::Delete) => {
                let mut btreeset_benchmark =
                    benchmark::btreeset::BTreeSetBenchmark::new(self.num_of_items);
                btreeset_benchmark.benchmark_delete();
            }
            (Collection::HashSet, Operation::Insert) => {
                let mut hashset_benchmark =
                    benchmark::hashset::HashSetBenchmark::new(self.num_of_items);
                hashset_benchmark.benchmark_insert();
            }
            (Collection::HashSet, Operation::Lookup) => {
                let hashset_benchmark =
                    benchmark::hashset::HashSetBenchmark::new(self.num_of_items);
                hashset_benchmark.benchmark_lookup();
            }
            (Collection::HashSet, Operation::Delete) => {
                let mut hashset_benchmark =
                    benchmark::hashset::HashSetBenchmark::new(self.num_of_items);
                hashset_benchmark.benchmark_delete();
            }
        }
    }
}

pub fn operation(name: String) -> Operation {
    match name.to_lowercase().as_str() {
        "insert" => Operation::Insert,
        "lookup" => Operation::Lookup,
        "delete" => Operation::Delete,
        _ => Operation::Insert,
    }
}

pub fn collection(name: String) -> Collection {
    match name.to_lowercase().as_str() {
        "vec" => Collection::Vec,
        "hashmap" => Collection::HashMap,
        "hashset" => Collection::HashSet,
        "btreemap" => Collection::BTreeMap,
        "btreeset" => Collection::BTreeSet,
        _ => Collection::Vec,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_benchmark() {
        let target = Target::new(None, None, None);
        assert_eq!(target.operation, Operation::Insert);
        assert_eq!(target.collection, Collection::Vec);
        assert_eq!(target.num_of_items, 1_000_000);
    }

    #[test]
    fn custom_benchmark() {
        let target = Target::new(
            Some(Collection::HashMap),
            Some(Operation::Lookup),
            Some(1_000),
        );
        assert_eq!(target.operation, Operation::Lookup);
        assert_eq!(target.collection, Collection::HashMap);
        assert_eq!(target.num_of_items, 1_000);
    }

    #[test]
    fn test_operation() {
        assert_eq!(operation("insert".to_string()), Operation::Insert);
        assert_eq!(operation("lookup".to_string()), Operation::Lookup);
        assert_eq!(operation("delete".to_string()), Operation::Delete);
        assert_eq!(operation("unknown".to_string()), Operation::Insert); // Default case
    }

    #[test]
    fn test_collection() {
        assert_eq!(collection("vec".to_string()), Collection::Vec);
        assert_eq!(collection("hashmap".to_string()), Collection::HashMap);
        assert_eq!(collection("hashset".to_string()), Collection::HashSet);
        assert_eq!(collection("btreemap".to_string()), Collection::BTreeMap);
        assert_eq!(collection("btreeset".to_string()), Collection::BTreeSet);
        assert_eq!(collection("unknown".to_string()), Collection::Vec); // Default case
    }
}
