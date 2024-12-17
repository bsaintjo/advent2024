use std::{ascii::AsciiExt, collections::{HashMap, HashSet}};

#[derive(Debug, Clone, Default)]
struct PageOrdering {
    befores: HashMap<usize, HashSet<usize>>,
}

impl PageOrdering {
    fn add_rule(&mut self, s: &str) {
        let mut parsed = s.split('|');
        let fst = parsed.next().unwrap().parse::<usize>().unwrap();
        let snd = parsed.next().unwrap().parse::<usize>().unwrap();
        self.befores.entry(fst).or_default().insert(snd);
    }

    fn valid(&self, page_num: usize, ahead: &HashSet<usize>) -> bool {
        let rules = &self.befores[&page_num];
        ahead.is_subset(rules)
    }
}

struct PageNumbers(Vec<usize>);

impl PageNumbers {
    fn middle_page(&self) -> usize {
        self.0[(self.0.len() - 1) / 2]
    }
    fn page_order(&self) -> impl Iterator<Item = (usize, HashSet<usize>)> + use<'_> {
        (0..self.0.len() - 1).map(|idx| (self.0[idx], self.0[idx + 1..].iter().copied().collect()))
    }

    fn parse_page_numbers(s: &str) -> Self {
        let pnumbers = s
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Self(pnumbers)
    }
}

fn parse_rules_pages(s: &str) -> (PageOrdering, Vec<PageNumbers>) {
    let mut lines = s.lines();
    let mut next_line = lines.next();
    while !next_line.unwrap().is_empty() {
        next_line = lines.next();
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_rule() {
        let rule = "72|26";
        let mut pordering = PageOrdering::default();
        pordering.add_rule(rule);
        assert_eq!(
            pordering.befores.get(&72),
            Some(&[26].into_iter().collect::<HashSet<_>>())
        );
    }

    #[test]
    fn test_page_num_order() {
        let pnums = PageNumbers(vec![75, 47, 61, 53, 29]);
        assert_eq!(pnums.middle_page(), 61);

        let mut orders = pnums.page_order();
        assert_eq!(orders.next(), Some((75, HashSet::from([47, 61, 53, 29]))));
        assert_eq!(orders.next(), Some((47, HashSet::from([61, 53, 29]))));
        assert_eq!(orders.next(), Some((61, HashSet::from([53, 29]))));
        assert_eq!(orders.next(), Some((53, HashSet::from([29]))));
        // assert_eq!(orders.next(), Some((29, HashSet::from([]))));
    }
}
