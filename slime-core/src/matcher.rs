use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

pub trait Matcher {
    fn rating(input: &str, targets: &[String]) -> HashMap<String, i32>;
}

pub struct WordCuttingMatcher;

impl Matcher for WordCuttingMatcher {
    fn rating(input: &str, targets: &[String]) -> HashMap<String, i32> {
        let input = input.to_lowercase();
        let eles = cut_word(&input);
        targets
            .par_iter()
            .map(|target| {
                let score = eles
                    .clone()
                    .into_iter()
                    .par_bridge()
                    .map(|ele| {
                        let number = target.to_lowercase().matches(&ele).count();
                        number as f64 * (2 ^ ele.len()) as f64
                    })
                    .sum::<f64>();
                (target, score)
            })
            .filter(|(_, score)| *score != 0.)
            .map(|(target, score)| (target.clone(), (score * 1000.) as i32))
            .collect()
    }
}

fn cut_word(input: &str) -> Vec<String> {
    let input: Vec<char> = input.chars().collect();
    let mut eles = vec![];
    let mut window = input.len();
    while window > 0 {
        let mut start = 0;
        while start + window <= input.len() {
            eles.push(input[start..start + window].iter().collect());
            start += 1;
        }
        window -= 1;
    }
    eles
}

#[cfg(test)]
mod test {
    use crate::matcher::cut_word;
    #[test]
    fn test_cut_word() {
        let eles = cut_word("hello");
        let target = vec![
            "hello", "hell", "ello", "hel", "ell", "llo", "he", "el", "ll", "lo", "h", "e", "l",
            "l", "o",
        ];
        assert_eq!(eles, target);
    }
}
