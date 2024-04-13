mod matcher;

use std::collections::HashMap;

use matcher::{Matcher, WordCuttingMatcher};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn fuzzy_search(input: &str, targets: &[String], n: usize) -> Vec<String> {
    if input.is_empty() {
        return targets[..std::cmp::min(n, targets.len())].to_vec();
    }
    let targets = WordCuttingMatcher::rating(input, targets);
    let mut result = top_n(targets, n);
    result.reverse();
    result
}

fn top_n(mut targets: HashMap<String, i32>, n: usize) -> Vec<String> {
    let mut v: Vec<String> = Vec::with_capacity(10);
    while v.len() < n && !targets.is_empty() {
        if let Some((target, _)) = targets.par_iter().max_by_key(|v| v.1) {
            let target = target.clone();
            targets.remove(&target);
            v.push(target)
        } else {
            continue;
        }
    }
    v
}

#[cfg(test)]
mod test {
    use crate::fuzzy_search;
    #[test]
    fn test_fuzzy_search() {
        let choices: Vec<String> = vec!["slime", "slim", "swim"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let input = "w";
        let choices = fuzzy_search(&input, &choices, 1);
        assert_eq!(&choices[0], "swim");
    }
}
