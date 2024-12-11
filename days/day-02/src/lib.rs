pub fn is_safe(xs: &[isize]) -> bool {
    (all_increasing(xs) || all_decreasing(xs)) && differing_adjacent(xs)
}

pub fn is_safe_tolerable(xs: &[isize]) -> bool {
    (0..xs.len()).any(|idx| {
        let ys = xs
            .iter()
            .copied()
            .enumerate()
            .flat_map(move |ix| if ix.0 == idx { None } else { Some(ix.1) })
            .collect::<Vec<_>>();
        is_safe(&ys)
    })
}

fn all_increasing(xs: &[isize]) -> bool {
    xs.windows(2).all(|xs| xs[1] > xs[0])
}

fn all_decreasing(xs: &[isize]) -> bool {
    xs.windows(2).all(|xs| xs[0] > xs[1])
}

fn differing_adjacent(xs: &[isize]) -> bool {
    xs.windows(2).all(|xs| {
        let res = (xs[0] - xs[1]).abs();
        (1..=3).contains(&res)
    })
}

pub fn parse_level(s: &str) -> Vec<isize> {
    s.split_ascii_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_safe() {
        let level = [7, 6, 4, 2, 1];
        assert!(is_safe(&level));
        assert!(is_safe_tolerable(&level));

        let level = [1, 2, 7, 8, 9];
        assert!(!is_safe(&level));
        assert!(!is_safe_tolerable(&level));

        let level = [9, 7, 6, 2, 1];
        assert!(!is_safe(&level));
        assert!(!is_safe_tolerable(&level));

        let level = [1, 3, 2, 4, 5];
        assert!(!is_safe(&level));
        assert!(is_safe_tolerable(&level));

        let level = [8, 6, 4, 4, 1];
        assert!(!is_safe(&level));
        assert!(is_safe_tolerable(&level));

        let level = [1, 3, 6, 7, 9];
        assert!(is_safe(&level));
        assert!(is_safe_tolerable(&level));
    }
}
