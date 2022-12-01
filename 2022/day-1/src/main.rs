use std::fs;

fn main() {
    let file_content = fs::read_to_string("./input.txt").expect("Error reading file.");
    let per_lines = file_content.split("\n");

    let mut tmp = 0;
    let mut max = 0;
    let mut per_elves: Vec<u32> = Vec::new(); // Part 2

    for line in per_lines.into_iter() {
        if line.is_empty() {
            per_elves.push(tmp); // Part 2
            tmp = 0;
            continue;
        }
        tmp += line.parse::<u32>().unwrap_or(0);
        max = if tmp > max { tmp } else { max };
    }

    println!("Max : {}", max);

    // Part 2
    per_elves.sort();
    let top_three = &per_elves[per_elves.len() - 3..];
    let total_top_three: u32 = top_three.iter().sum();

    println!("Total top three : {:?}", total_top_three);
}
