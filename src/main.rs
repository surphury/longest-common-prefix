mod test;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = strs[0].clone();

    loop {
        let is_prefix = strs.iter().all(|string| string.starts_with(&prefix));

        if is_prefix {
            break;
        }

        prefix.pop();
    }

    prefix
}

fn main() {}
