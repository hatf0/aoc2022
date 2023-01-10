use std::vec::Vec;

fn main() {
  let input = std::fs::read_to_string("./input").unwrap();
  let groups = input.split("\n\n").map(|x| x.split('\n'));
  let iter = groups.enumerate()
                   .map(|(i, value)| (i, value.map(|x| x.parse::<i32>().unwrap_or(0)).collect::<Vec<i32>>()));
                   
  let mut calorie_totals : Vec<(usize, i32)> = iter.map(|x| (x.0, x.1.iter().sum::<i32>()))
                                                   .collect();
                  
  calorie_totals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
          
  println!("top elf is carrying {} calories", calorie_totals.get(0).unwrap().1);
  println!("top three elves are carrying {} calories", &calorie_totals[0..3].iter().map(|x| x.1).sum::<i32>());
}
