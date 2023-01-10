use scan_fmt::scan_fmt;
use std::{cell::RefCell, collections::VecDeque, str::FromStr};

struct CratesStack {
    stack: Vec<RefCell<VecDeque<char>>>,
}

#[derive(Debug, PartialEq, Eq)]
struct CratesStackParseError;

impl FromStr for CratesStack {
    type Err = CratesStackParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split('\n').filter(|x| !x.is_empty());

        let mut crates = CratesStack { stack: Vec::new() };
        for _ in 0..9 {
            crates.stack.push(RefCell::new(VecDeque::new()));
        }

        let mut last_line = 0;

        for (index, line) in lines.clone().enumerate() {
            // dirty, dirty, dirty hack
            if line.chars().nth(1).unwrap() == '1' {
                last_line = index;
                break;
            }

            let mut start = 0;
            let mut col = 0;
            while (start + 3) <= line.len() {
                let window = &line[start..start + 3];
                let val = window.chars().nth(1).unwrap();

                if val != ' ' {
                    crates.stack.get(col).unwrap().borrow_mut().push_back(val);
                }

                start += 4;
                col += 1;
            }
        }

        for line in lines.skip(last_line + 1) {
            let elems = scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).unwrap();
            let mut src = crates.stack.get(elems.1 - 1).unwrap().borrow_mut();
            let mut dst = crates.stack.get(elems.2 - 1).unwrap().borrow_mut();

            // part 1:
            // for _ in 0..elems.0 {
            //     dst.push_front(src.pop_front().unwrap())
            // }

            src.range(..elems.0).rev().for_each(|x| dst.push_front(*x));

            for _ in 0..elems.0 {
                src.pop_front();
            }
        }

        Ok(crates)
    }
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let crate_stack = CratesStack::from_str(input.as_str()).unwrap();
    let answer: String = crate_stack
        .stack
        .iter()
        .filter(|x| x.borrow().len() != 0)
        .map(|x| *x.borrow().front().unwrap())
        .collect();
    println!("{answer:?}");
}
