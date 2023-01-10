use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let start_of_packet = input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, x)| x.iter().unique().count() == 4)
        .unwrap();

    println!("part 1: {:?}", start_of_packet.0 + 4);

    let start_of_message = input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, x)| x.iter().unique().count() == 14)
        .unwrap();

    println!("part 2: {:?}", start_of_message.0 + 14);
}
