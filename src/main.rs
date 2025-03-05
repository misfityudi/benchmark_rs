use benchmark::Target;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let collection = args
        .get(1)
        .map(|arg| arg.to_string())
        .unwrap_or_else(|| "vec".to_string());
    let operation = args
        .get(2)
        .map(|arg| arg.to_string())
        .unwrap_or_else(|| "insert".to_string());
    let num_of_items = args
        .get(3)
        .and_then(|arg| arg.parse::<i32>().ok())
        .unwrap_or(1_000_000);

    let collection_enum = benchmark::collection(collection);
    let operation_enum = benchmark::operation(operation);

    let target = Target::new(
        Some(collection_enum),
        Some(operation_enum),
        Some(num_of_items),
    );
    target.benchmark();
}
