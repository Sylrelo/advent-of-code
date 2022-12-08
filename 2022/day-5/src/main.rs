use regex::Regex;
use std::{collections::VecDeque, fs};

fn print_stacks(stacks: &Vec<VecDeque<char>>) {
    for (index, stack) in stacks.into_iter().enumerate() {
        println!("{} > {:?}", index + 1, stack);
    }
}

fn main() {
    let mut global_stacks: Vec<VecDeque<char>> = Vec::new();

    let file_content = fs::read_to_string("./example.txt").expect("Error reading file.");
    let move_regex = Regex::new(r"[\d+]*[\d+]*[\d+]").unwrap();

    let splatula: Vec<&str> = file_content.split("\n\n").collect();

    let stacks = splatula[0].split("\n").into_iter();
    let moves = splatula[1].split("\n").into_iter();

    for line in stacks {
        let stack = line.chars().collect::<Vec<char>>();
        let columns = stack.chunks(4).map(|c| c[1]).collect::<Vec<char>>();
        for (index, value) in columns.iter().enumerate() {
            if global_stacks.get(index).is_none() {
                global_stacks.push(VecDeque::new());
            }
            if value.is_ascii_alphabetic() {
                global_stacks[index].push_back(*value);
            }
        }
    }

    print_stacks(&global_stacks);

    for line in moves {
        let mut tmp: Vec<char> = Vec::new();
        let cap = move_regex
            .captures_iter(line)
            .map(|c| c.get(0).unwrap().as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        for _i in 0..cap[0] {
            let boxes = global_stacks[cap[1] - 1].pop_front();
            tmp.push(boxes.unwrap());
        }

        // Uncomment for Part 2.
        // if tmp.len() > 1 {
        //     tmp.reverse();
        // }

        for v in tmp {
            global_stacks[cap[2] - 1].push_front(v);
        }

        println!("{}", line);
        print_stacks(&global_stacks);
    }

    let res = global_stacks
        .iter()
        .map(|stack| *stack.get(0).unwrap_or(&' '))
        .collect::<Vec<char>>();

    println!("{}", String::from_iter(res));
}
