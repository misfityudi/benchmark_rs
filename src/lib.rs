mod definitions;
use definitions::{Collection, Operation};

pub struct Target {
    pub operation: Operation,
    pub collection: Collection,
    pub num_of_items: i32,
}

impl Target {
    pub fn new(
        operation: Option<Operation>,
        collection: Option<Collection>,
        num_of_items: Option<i32>,
    ) -> Self {
        Self {
            operation: operation.unwrap_or(Operation::Insert),
            collection: collection.unwrap_or(Collection::Vector),
            num_of_items: num_of_items.unwrap_or(1_000_000),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_benchmark() {
        let target = Target::new(None, None, None);
        assert_eq!(target.operation, Operation::Insert);
        assert_eq!(target.collection, Collection::Vector);
        assert_eq!(target.num_of_items, 1_000_000);
    }

    #[test]
    fn custom_benchmark() {
        let target = Target::new(
            Some(Operation::Lookup),
            Some(Collection::HashMap),
            Some(1_000),
        );
        assert_eq!(target.operation, Operation::Lookup);
        assert_eq!(target.collection, Collection::HashMap);
        assert_eq!(target.num_of_items, 1_000);
    }
}
