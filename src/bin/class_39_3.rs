use std::collections::BTreeMap;

fn main() {
    todo!();
}

pub struct Solution {}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let chars: Vec<char> = formula.chars().collect();
        let mut i = 0;

        fn parse_formula(chars: &[char], i: &mut usize) -> BTreeMap<String, i32> {
            let mut map = BTreeMap::new();

            while *i < chars.len() {
                if chars[*i] == '(' {
                    *i += 1;
                    let inner = parse_formula(chars, i);
                    let count = parse_count(chars, i);
                    for (atom, num) in inner {
                        *map.entry(atom).or_insert(0) += num * count;
                    }
                } else if chars[*i] == ')' {
                    *i += 1;
                    break;
                } else {
                    let atom = parse_atom(chars, i);
                    let count = parse_count(chars, i);
                    *map.entry(atom).or_insert(0) += count;
                }
            }

            map
        }

        fn parse_atom(chars: &[char], i: &mut usize) -> String {
            let mut atom = String::new();
            atom.push(chars[*i]);
            *i += 1;
            while *i < chars.len() && chars[*i].is_ascii_lowercase() {
                atom.push(chars[*i]);
                *i += 1;
            }
            atom
        }

        fn parse_count(chars: &[char], i: &mut usize) -> i32 {
            if *i >= chars.len() || !chars[*i].is_ascii_digit() {
                return 1;
            }
            let mut count = 0;
            while *i < chars.len() && chars[*i].is_ascii_digit() {
                count = count * 10 + (chars[*i] as i32 - '0' as i32);
                *i += 1;
            }
            count
        }

        let map = parse_formula(&chars, &mut i);

        let mut result = String::new();

        for (atom, count) in map {
            result.push_str(&atom);
            if count > 1 {
                result.push_str(&count.to_string());
            }
        }

        result
    }
}
