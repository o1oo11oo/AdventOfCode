use regex::Regex;
use serde_json::Value;

pub(crate) fn part_1(input: &str) -> String {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(input)
        .map(|n| n.as_str().parse::<i64>().unwrap())
        .sum::<i64>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    count(&serde_json::from_str(input).unwrap()).to_string()
}

fn count(json: &Value) -> i64 {
    match json {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(count).sum::<i64>(),
        Value::Object(o) => {
            if o.values().all(|v| v != &Value::String("red".to_string())) {
                o.values().map(count).sum::<i64>()
            } else {
                0
            }
        }
    }
}
