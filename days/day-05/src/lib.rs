use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct PageOrdering {
    befores: HashMap<usize, HashSet<usize>>,
}

impl PageOrdering {
    fn add_rule<S: AsRef<str>>(&mut self, s: S) {
        let mut parsed = s.as_ref().split(['|', '\n']);
        let fst = parsed.next().unwrap().parse::<usize>().unwrap();
        let snd = parsed.next().unwrap().parse::<usize>().unwrap();
        self.befores.entry(fst).or_default().insert(snd);
    }

    pub fn valid(&self, page_num: usize, ahead: &HashSet<usize>) -> bool {
        let default = HashSet::default();
        let rules = self.befores.get(&page_num).unwrap_or(&default);
        ahead.is_subset(rules)
    }

    fn count_ahead(&self, page_num: usize, ahead: &HashSet<usize>) -> usize {
        let default = HashSet::default();
        let rules = self.befores.get(&page_num).unwrap_or(&default);
        ahead.intersection(rules).count()
    }
}

#[derive(Debug, Clone)]
pub struct PageNumbers(Vec<usize>);

impl PageNumbers {
    pub fn middle_page(&self) -> usize {
        self.0[(self.0.len() - 1) / 2]
    }
    pub fn page_order(&self) -> impl Iterator<Item = (usize, HashSet<usize>)> + use<'_> {
        (0..self.0.len() - 1).map(|idx| (self.0[idx], self.0[idx + 1..].iter().copied().collect()))
    }

    fn all_vs_others(&self) -> impl Iterator<Item = (usize, HashSet<usize>)> + use<'_> {
        (0..self.0.len()).map(|idx| {
            (
                self.0[idx],
                self.0
                    .iter()
                    .enumerate()
                    .filter_map(|(idy, n)| if idy == idx { None } else { Some(n) })
                    .copied()
                    .collect(),
            )
        })
    }

    pub fn parse_page_numbers<S: AsRef<str> + ?Sized>(s: &S) -> Self {
        let pnumbers = s
            .as_ref()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Self(pnumbers)
    }

    pub fn reorder_pages(&self, page_order: &PageOrdering) -> Self {
        let mut order_pos: Vec<(usize, usize)> = Vec::new();
        for (x, others) in self.all_vs_others() {
            let idx = page_order.count_ahead(x, &others);
            order_pos.push((x, idx));
        }
        order_pos.sort_by_key(|x| x.1);
        order_pos.reverse();
        PageNumbers(order_pos.into_iter().map(|x| x.0).collect())
    }
}

pub fn parse_rules_pages(s: &str) -> (PageOrdering, Vec<PageNumbers>) {
    let mut porder = PageOrdering::default();
    let mut lines = s.lines();
    let mut next_line = lines.next();
    while !next_line.unwrap().is_empty() {
        porder.add_rule(next_line.unwrap());
        next_line = lines.next();
    }
    let pnums = lines.map(PageNumbers::parse_page_numbers).collect();
    (porder, pnums)
}

pub fn count_wrong_middle_pages<I: Iterator<Item = S>, S: AsRef<str>>(mut iter: I) -> usize {
    let mut porder = PageOrdering::default();
    let mut binding = iter.next().unwrap();
    let mut next_line = binding.as_ref();

    while !next_line.is_empty() {
        porder.add_rule(next_line);
        binding = iter.next().unwrap();
        next_line = binding.as_ref();
    }
    let pnums: Vec<_> = iter.map(|s| PageNumbers::parse_page_numbers(&s)).collect();
    let mut middle_pages = 0;
    for pnum in pnums {
        if !pnum
            .page_order()
            .all(|(idx, ahead)| porder.valid(idx, &ahead))
        {
            let reordered = pnum.reorder_pages(&porder);
            middle_pages += reordered.middle_page();
        }
    }
    middle_pages
}

pub fn count_middle_pages<I: Iterator<Item = S>, S: AsRef<str>>(mut iter: I) -> usize {
    let mut porder = PageOrdering::default();
    let mut binding = iter.next().unwrap();
    let mut next_line = binding.as_ref();

    while !next_line.is_empty() {
        porder.add_rule(next_line);
        binding = iter.next().unwrap();
        next_line = binding.as_ref();
    }
    let pnums: Vec<_> = iter.map(|s| PageNumbers::parse_page_numbers(&s)).collect();
    let mut middle_pages = 0;
    for pnum in pnums {
        if pnum
            .page_order()
            .all(|(idx, ahead)| porder.valid(idx, &ahead))
        {
            middle_pages += pnum.middle_page();
        }
    }
    middle_pages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        let (porder, pnums) = parse_rules_pages(example);
        assert_eq!(porder.befores.len(), 6);

        let mut middle_pages = 0;
        for pnum in pnums.clone() {
            if pnum
                .page_order()
                .all(|(idx, ahead)| porder.valid(idx, &ahead))
            {
                middle_pages += pnum.middle_page();
            }
        }
        assert_eq!(middle_pages, 143);

        let mut middle_pages = 0;
        for pnum in pnums {
            if !pnum
                .page_order()
                .all(|(idx, ahead)| porder.valid(idx, &ahead))
            {
                let reordered = pnum.reorder_pages(&porder);
                middle_pages += reordered.middle_page();
            }
        }
        assert_eq!(middle_pages, 123)
    }

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
    fn test_reorder() {
        let example = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        let (porder, _) = parse_rules_pages(example);
        let pnum = PageNumbers(vec![75, 97, 47, 61, 53]);
        assert!(!pnum
            .page_order()
            .all(|(page_num, ahead)| porder.valid(page_num, &ahead)));
        assert_eq!(pnum.reorder_pages(&porder).0, vec![97, 75, 47, 61, 53]);
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
