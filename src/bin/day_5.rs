use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_5").unwrap();
    let (rules, manuals) = input.split_once("\n\n").unwrap();

    let rules_iter = rules.lines().map(|s| {
        let (before, after) = s.split_once('|').unwrap();
        (before.parse::<u8>().unwrap(), after.parse::<u8>().unwrap())
    });

    let rules = Rules::new(rules_iter);

    let manuals: Vec<_> = manuals
        .lines()
        .map(|l| {
            let page_nums: Vec<_> = l.split(',').map(|n| n.parse::<u8>().unwrap()).collect();
            Manual::new(page_nums)
        })
        .collect();

    let total: u16 = manuals
        .iter()
        .filter(|m| m.is_valid(&rules))
        .map(Manual::get_middle)
        .map(u16::from)
        .sum();

    println!("Middle Sum: {}", total);

    let total_fixed: u16 = manuals
        .iter()
        .filter(|m| !m.is_valid(&rules))
        .map(|m| m.fix(&rules))
        .map(|m| m.get_middle())
        .map(u16::from)
        .sum();

    println!("Fixed Middle Sum: {}", total_fixed);
}

struct Rules {
    before_to_after: HashMap<u8, HashSet<u8>>,
    after_to_before: HashMap<u8, HashSet<u8>>,
}

impl Rules {
    pub fn new(it: impl Iterator<Item = (u8, u8)>) -> Self {
        let mut before_to_after: HashMap<u8, HashSet<u8>> = HashMap::new();
        let mut after_to_before: HashMap<u8, HashSet<u8>> = HashMap::new();

        it.for_each(|(before, after)| {
            match before_to_after.get_mut(&before) {
                Some(set) => {
                    set.insert(after);
                }
                None => {
                    let mut set = HashSet::new();
                    set.insert(after);
                    before_to_after.insert(before, set);
                }
            };

            match after_to_before.get_mut(&after) {
                Some(set) => {
                    set.insert(before);
                }
                None => {
                    let mut set = HashSet::new();
                    set.insert(before);
                    after_to_before.insert(after, set);
                }
            };
        });

        Self {
            before_to_after,
            after_to_before,
        }
    }

    pub fn before_page(&self, page: u8) -> Option<&HashSet<u8>> {
        self.after_to_before.get(&page)
    }

    pub fn after_page(&self, page: u8) -> Option<&HashSet<u8>> {
        self.before_to_after.get(&page)
    }
}

struct Manual {
    pages: Vec<u8>,
}

impl Manual {
    pub fn new(nums: Vec<u8>) -> Self {
        Self { pages: nums }
    }

    pub fn is_valid(&self, rules: &Rules) -> bool {
        let mut before_current_page = vec![];

        for page in &self.pages {
            if let Some(pages_after_rules) = rules.after_page(*page) {
                for page in pages_after_rules {
                    if let Ok(_) = before_current_page.binary_search(page) {
                        return false;
                    }
                }
            }

            if let Err(index) = before_current_page.binary_search(page) {
                before_current_page.insert(index, *page);
            }
        }

        true
    }

    pub fn get_middle(&self) -> u8 {
        let index = self.pages.len() / 2;

        self.pages[index]
    }

    pub fn fix(&self, rules: &Rules) -> Self {
        let mut ordered_pages = vec![];

        for page in &self.pages {
            if let Some(page_after_rules) = rules.after_page(*page) {
                let mut index = 0;

                for ordered_page in &ordered_pages {
                    if page_after_rules.contains(ordered_page) {
                        break;
                    }

                    index += 1;
                }

                ordered_pages.insert(index, *page);
            } else {
                ordered_pages.push(*page);
            }
        }

        Self::new(ordered_pages)
    }
}
