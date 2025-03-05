#[derive(Debug, PartialEq)]
pub enum Operation {
    Insert,
    Lookup,
    Delete,
}

#[derive(Debug, PartialEq)]
pub enum Collection {
    Vec,
    HashMap,
    HashSet,
    BTreeMap,
    BTreeSet,
}
