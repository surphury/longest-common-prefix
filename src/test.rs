#[cfg(test)]
use crate::longest_common_prefix;

#[test]
fn test_1() {
    let arr = vec!["flower", "flow", "flight"]
        .iter()
        .map(|str| str.to_string())
        .collect();

    let result = longest_common_prefix(arr);
    assert_eq!(result, "fl");
}

#[test]
fn test_2() {
    let arr = vec!["dog", "racecar", "car"]
        .iter()
        .map(|str| str.to_string())
        .collect();

    let result = longest_common_prefix(arr);
    assert_eq!(result, "");
}
