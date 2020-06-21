use std::cmp;
use std::collections::HashMap;

pub fn search(str: &str, pattern: &str) -> Option<usize> {
    let shift_table = create_shift_table(pattern);

    let mut i = pattern.len() - 1;

    while i < str.len() {
        let mut p = pattern.len() - 1;

        loop {
            if str.as_bytes()[i] == pattern.as_bytes()[p] {
                if p == 0 {
                    return Some(i);
                }
                i = i - 1;
                p = p - 1;
            } else {
                break;
            }
        }

        let shift: usize;
        match shift_table.get(&char::from(str.as_bytes()[i])) {
            Some(skip) => {
                shift = *skip;
            }
            None => {
                shift = pattern.len();
            }
        }

        let guard_shift = pattern.len() - p;
        i += cmp::max(shift, guard_shift);
    }

    None
}

fn create_shift_table(str: &str) -> HashMap<char, usize> {
    let mut table = HashMap::new();
    let length = str.len();
    for i in 0..length {
        table.insert(char::from(str.as_bytes()[i]), length - i - 1);
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_finds_pattern_from_str() {
        assert_eq!(Some(5), search("valorant", "an"));
        assert_eq!(Some(0), search("valorant", "v"));
        assert_eq!(Some(7), search("valorant", "t"));
    }

    #[test]
    fn search_returns_none_when_not_found_pattern() {
        assert_eq!(None, search("valorant", "lol"))
    }

    #[test]
    fn create_shift_table_returns_shift_table() {
        let mut map = HashMap::new();
        map.insert('a', 3);
        map.insert('b', 1);
        map.insert('c', 0);
        assert_eq!(map, create_shift_table(&String::from("abbc")))
    }
}
