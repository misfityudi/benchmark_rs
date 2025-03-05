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
        println!("{:?}", self);
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
}
