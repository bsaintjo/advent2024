use std::collections::HashMap;

pub fn distance(left: &[usize], right: &[usize]) -> usize {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum()
}

pub fn parse_nums<S: AsRef<str>, I: Iterator<Item = S>>(s: I) -> (Vec<usize>, Vec<usize>) {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for line in s {
        let mut num_iter = line.as_ref().split_whitespace();
        let left = num_iter.next().unwrap().parse::<usize>().unwrap();
        let right = num_iter.next().unwrap().parse::<usize>().unwrap();
        lefts.push(left);
        rights.push(right);
    }
    (lefts, rights)
}

fn counts(xs: &[usize]) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();
    for x in xs {
        *counts.entry(*x).or_insert(0) += 1;
    }
    counts
}

pub fn similarity_score(left: &[usize], right: &[usize]) -> usize {
    let right_counts = counts(right);
    left.iter()
        .flat_map(|l| right_counts.get(l).map(|r| r * l))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counts() {
        let xs = [4usize, 3, 5, 3, 9, 3];
        let counts = counts(&xs);
        assert_eq!(counts.get(&3), Some(&3));
        assert_eq!(counts.get(&4), Some(&1));
        assert_eq!(counts.get(&2), None);
    }

    #[test]
    fn test_day01() {
        let example = "3   4
4   3
2   5
1   3
3   9
3   3";
        let (mut left, mut right) = parse_nums(example.lines());
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);

        assert_eq!(similarity_score(&left, &right), 31);

        left.sort();
        right.sort();
        assert_eq!(distance(&left, &right), 11)
    }
}
