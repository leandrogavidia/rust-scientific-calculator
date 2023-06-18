use regex::Regex;

fn create_regex(operator: &str) -> Regex {
    let regex_structure = format!(r"([+-]?\d+(\.\d+)?)\s?\{operator}\s?(\d+(\.\d+)?)");
    let regex = Regex::new(&regex_structure).unwrap();
    return regex
}

fn resolve_operation(mut operation: String, operator: &str) -> String {
    let regex = create_regex(operator);
    loop {
        let caps = regex.captures(&operation);

        if caps.is_none() { break }
        
        let caps = caps.unwrap();
        let caps_operation = caps.get(0).unwrap().as_str();
        let left_number: f64 = caps.get(1).unwrap().as_str().parse::<f64>().unwrap();
        let right_number: f64 = caps.get(3).unwrap().as_str().parse::<f64>().unwrap();
        let result = match operator {
            "*" => left_number * right_number,
            "/" => left_number / right_number,
            "+" => left_number + right_number,
            "-" => left_number - right_number,
            _ => 0.0
        };
        operation = operation.replace(caps_operation, &result.to_string());
    }
    return operation;
}

fn main() {
    let mut operation = String::new();
    println!("Please, enter your operation:");
    std::io::stdin().read_line(&mut operation).unwrap();

    operation = resolve_operation(operation.clone(), "*");
    operation = resolve_operation(operation.clone(), "/");
    operation = resolve_operation(operation.clone(), "+");
    operation = resolve_operation(operation.clone(), "-");

    println!("Final result {operation}")
}