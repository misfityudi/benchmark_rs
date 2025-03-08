#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operation {
    Insert,
    Lookup,
    Delete,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Collection {
    Vec,
    HashMap,
    HashSet,
    BTreeMap,
    BTreeSet,
}
