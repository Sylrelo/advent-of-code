use std::fs;

fn main() {
    let file_content = fs::read_to_string("./input.txt").expect("Error reading file.");
    let per_lines = file_content.split("\n");

    let mut total_score_part1 = 0;
    let mut total_score_part2 = 0;

    let win_table = [3, 1, 2];
    let loose_table = [2, 3, 1]; // Part Two

    for line in per_lines.into_iter() {
        let opponent = line.chars().nth(0).unwrap() as i32 - 64;
        let yourself = line.chars().nth(2).unwrap() as i32 - 87;

        // ==== Part One
        let round_result = opponent - yourself;
        total_score_part1 += yourself;

        if round_result == 0 {
            total_score_part1 += 3;
        }

        if round_result != 0 && win_table[yourself as usize - 1] == opponent {
            total_score_part1 += 6;
        }

        // ==== Part Two
        if yourself == 1 {
            total_score_part2 += win_table[opponent as usize - 1];
        }

        if yourself == 2 {
            total_score_part2 += 3 + opponent;
        }

        if yourself == 3 {
            total_score_part2 += 6 + loose_table[opponent as usize - 1];
        }
    }

    println!(
        "Part 1 : {}\nPart 2 : {}",
        total_score_part1, total_score_part2
    );
}
