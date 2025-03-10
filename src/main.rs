use benchmark::Target;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let collection = args
        .get(1)
        .map(|arg| arg.to_string())
        .unwrap_or_else(|| "vec".to_string());
    let operation = args.get(2).map(|arg| arg.to_string());
    let num_of_items = args
        .get(3)
        .and_then(|arg| arg.parse::<i32>().ok())
        .unwrap_or(100_000);

    let collection_enum = benchmark::collection(collection);

    if operation.is_none() {
        let operations = vec![
            benchmark::operation("insert".to_string()),
            benchmark::operation("lookup".to_string()),
            benchmark::operation("delete".to_string()),
        ];

        for operation_enum in &operations {
            let target = Target::new(
                Some(collection_enum),
                Some(*operation_enum),
                Some(num_of_items),
            );
            target.benchmark();
        }
    } else {
        let operation_enum = benchmark::operation(operation.unwrap());
        let target = Target::new(
            Some(collection_enum),
            Some(operation_enum),
            Some(num_of_items),
        );
        target.benchmark();
    }
}
