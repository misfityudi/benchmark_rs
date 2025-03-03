#[derive(Debug, PartialEq)]
pub enum Operation {
    Insert,
    Lookup,
    Delete,
}

#[derive(Debug, PartialEq)]
pub enum Collection {
    Vector,
    HashMap,
    HashSet,
    BTreeMap,
    BTreeSet,
}
