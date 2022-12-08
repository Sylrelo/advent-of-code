use std::fs;

fn get_marker(buffer: &str, distinct_count: usize) -> Option<usize> {
    let chars: Vec<char> = buffer.chars().collect();
    let len = buffer.len();

    for index in 0..len {
        let cur = &chars[index..std::cmp::min(index + distinct_count, len - 1)];
        let has_same_chars = cur
            .into_iter()
            .any(|char| cur.into_iter().filter(|f| **f == *char).count() > 1);

        if !has_same_chars {
            println!("{} {:?} {}", index, cur, (index + distinct_count) as i32);
            return Some(index + distinct_count);
        }
    }
    return None;
}
fn main() {
    let file_content = fs::read_to_string("./input.txt").expect("Error reading file.");

    let res = get_marker(&file_content, 14);

    match res {
        Some(r) => {
            println!("{}", r);
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(get_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), Some(7));
        assert_eq!(get_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
        assert_eq!(get_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
        assert_eq!(get_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), Some(10));
        assert_eq!(get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), Some(11));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(get_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
        assert_eq!(get_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
        assert_eq!(get_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), Some(23));
        assert_eq!(
            get_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            Some(29)
        );
        assert_eq!(get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), Some(26));
    }
}
