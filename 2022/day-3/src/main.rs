use std::fs;

fn get_priority(c: char) -> u32 {
    if c as u8 >= 97 {
        (c as u8 - 96) as u32
    } else {
        (c as u8 - 64 + 26) as u32
    }
}

fn search_common_item(compartments: (&str, &str)) -> Vec<char> {
    return compartments
        .0
        .chars()
        .filter(|c| compartments.1.contains(*c))
        .collect::<Vec<char>>();
}

fn main() {
    let file_content = fs::read_to_string("./input.txt").expect("Error reading file.");
    let per_lines = file_content.split("\n").collect::<Vec<&str>>();

    // Part One ============================
    let total_score: u32 = per_lines
        .clone()
        .into_iter()
        .map(|rucksack| {
            let items = search_common_item(rucksack.split_at(rucksack.len() / 2));
            get_priority(items[0])
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .sum();
    println!("Part 1 : {}", total_score);

    // Part Two ============================
    let rucksacks_per_group = per_lines.chunks_exact(3).collect::<Vec<&[&str]>>();
    let sum_of_priorities: u32 = rucksacks_per_group
        .into_iter()
        .map(|rucksacks| {
            let item = rucksacks[0]
                .chars()
                .filter(|c| rucksacks[1].contains(*c) && rucksacks[2].contains(*c))
                .collect::<Vec<char>>();
            get_priority(item[0])
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .sum();

    println!("Part 2 : {}", sum_of_priorities);
}
