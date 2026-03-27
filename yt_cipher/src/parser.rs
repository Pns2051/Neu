#[derive(Debug, Clone)]
pub enum Op {
    Reverse,
    Slice(usize),
    Swap(usize),
}

pub fn parse_js(_js: &str) -> anyhow::Result<Vec<Op>> {
    // Advanced parsing of JS goes here.
    // Finding decipher function and reverse engineering its operations.
    let ops = vec![Op::Reverse, Op::Slice(1), Op::Swap(2)];
    Ok(ops)
}

pub fn decipher(sig: &str, ops: &[Op]) -> String {
    let mut chars: Vec<char> = sig.chars().collect();
    for op in ops {
        match op {
            Op::Reverse => chars.reverse(),
            Op::Slice(n) => {
                if *n < chars.len() {
                    chars.drain(0..*n);
                }
            }
            Op::Swap(n) => {
                if !chars.is_empty() && *n < chars.len() {
                    chars.swap(0, *n);
                }
            }
        }
    }
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decipher() {
        let sig = "abcdefg";
        let ops = vec![Op::Reverse, Op::Slice(1), Op::Swap(2)]; // gfedcba -> fedcba -> fecdba
        let res = decipher(sig, &ops);
        assert_eq!(res, "fecdba");
    }
}
