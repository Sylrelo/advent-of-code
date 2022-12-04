use std::fs;

fn main() {
    let file_content = fs::read_to_string("./input.txt").expect("Error reading file.");
    let per_lines = file_content.split("\n").into_iter();

    let mut total_part_1 = 0;
    let mut total_part_2 = 0;

    for line in per_lines {
        let pairs: Vec<Vec<i32>> = line
            .split(",")
            .map(|p| {
                p.split("-")
                    .map(|f| f.trim().parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        // Part One
        if pairs[0][0] <= pairs[1][0] && pairs[0][1] >= pairs[1][1]
            || pairs[1][0] <= pairs[0][0] && pairs[1][1] >= pairs[0][1]
        {
            total_part_1 += 1;
        }

        // Part Two
        // if (pairs[0][0] >= pairs[1][0] && pairs[0][0] <= pairs[1][1])
        //     || (pairs[0][1] >= pairs[1][0] && pairs[0][1] <= pairs[1][1])
        //     || (pairs[1][0] >= pairs[0][0] && pairs[1][0] <= pairs[0][1])
        //     || (pairs[1][1] >= pairs[0][0] && pairs[1][1] <= pairs[0][1])
        // {
        //     total_part_2 += 1;
        // }

        if !(pairs[0][1] < pairs[1][0] || pairs[1][1] < pairs[0][0]) {
            total_part_2 += 1;
        }
    }
    println!(
        "total_part_1 : {}\ntotal_part_2 : {}",
        total_part_1, total_part_2
    );
}
