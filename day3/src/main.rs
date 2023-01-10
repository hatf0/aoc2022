use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

#[derive(Debug, Eq)]
struct RucksackItem {
    item: char,
}

impl RucksackItem {
    fn get_priority(&self) -> i32 {
        if self.item.is_ascii_lowercase() {
            (self.item as i32 - 'a' as i32) + 1
        } else if self.item.is_ascii_uppercase() {
            (self.item as i32 - 'A' as i32) + 27
        } else {
            panic!("unexpected rucksack item")
        }
    }
}

impl Ord for RucksackItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_priority().cmp(&other.get_priority())
    }
}

impl PartialOrd for RucksackItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RucksackItem {
    fn eq(&self, other: &Self) -> bool {
        self.get_priority() == other.get_priority()
    }
}

#[derive(Debug, PartialEq)]
struct RucksackItemErr;

impl FromStr for RucksackItem {
    type Err = RucksackItemErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item: char = s.chars().next().unwrap();

        if !item.is_ascii_lowercase() && !item.is_ascii_uppercase() {
            Err(RucksackItemErr)
        } else {
            Ok(RucksackItem { item })
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let sacks = input.split('\n').filter(|x| !x.is_empty());

    let sum = sacks
        .clone()
        .map(|sack| {
            let lhs = &sack[0..sack.len() / 2];
            let rhs = &sack[sack.len() / 2..sack.len()];

            let lhs_heap = lhs
                .chars()
                .map(|x| RucksackItem::from_str(x.to_string().as_str()).unwrap())
                .collect::<BinaryHeap<RucksackItem>>();
            let rhs_heap = rhs
                .chars()
                .map(|x| RucksackItem::from_str(x.to_string().as_str()).unwrap())
                .collect::<BinaryHeap<RucksackItem>>();

            // time complexity: O(fuck you)
            let duplicates = lhs_heap
                .iter()
                .filter(|x| {
                    rhs_heap
                        .iter()
                        .any(|y| y.get_priority() == x.get_priority())
                })
                .unique_by(|x| x.get_priority());

            duplicates.map(RucksackItem::get_priority).sum::<i32>()
        })
        .sum::<i32>();

    println!("{sum:?}");

    // part 2
    let chunks = sacks.clone().into_iter().chunks(3);
    let mut group_ids: Vec<RucksackItem> = Vec::new();

    for chunk in &chunks {
        let freq_map = chunk.map(|x| x.chars().unique()).flatten().counts();

        let result = freq_map
            .iter()
            .filter(|x| x.1 == &3usize)
            .exactly_one()
            .unwrap();

        group_ids.push(RucksackItem::from_str(result.0.to_string().as_str()).unwrap());
    }

    let sum_2 = group_ids.iter().map(|x| x.get_priority()).sum::<i32>();
    println!("{sum_2:?}");
}
