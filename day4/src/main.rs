use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Range(i32, i32);

impl Range {
    fn contained_within(&self, other: &Range) -> bool {
        if self.0 < other.0 {
            return false;
        }

        if self.1 > other.1 {
            return false;
        }

        true
    }

    fn overlaps(&self, other: &Range) -> bool {
        if self.0 <= other.0 && other.0 <= self.1 {
            return true;
        }

        if other.0 <= self.0 && self.0 <= other.1 {
            return true;
        }

        false
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<i32> = s.split('-').map(|x| x.parse::<i32>().unwrap()).collect();
        Ok(Range(*items.first().unwrap(), *items.get(1).unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_containment() {
        assert_eq!(Range(2, 3).contained_within(&Range(1, 5)), true);
        assert_eq!(Range(5, 7).contained_within(&Range(2, 3)), false);
    }

    #[test]
    fn test_range_from_str() {
        assert_eq!(Range::from_str("2-3").unwrap() == Range(2, 3), true);
        assert_eq!(Range::from_str("5-7").unwrap() == Range(5, 7), true);
    }
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let ranges = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.split(',').map(Range::from_str));

    let mut result_0 = 0;
    let mut result_1 = 0;
    for mut range in ranges {
        let first = range.next().unwrap().unwrap();
        let second = range.next().unwrap().unwrap();

        if first.contained_within(&second) || second.contained_within(&first) {
            result_0 += 1;
        }

        if first.overlaps(&second) {
            result_1 += 1;
        }
    }

    println!("first answer: {result_0:?}");
    println!("second answer: {result_1:?}");
}
