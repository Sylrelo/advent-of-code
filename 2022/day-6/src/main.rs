use std::fs;

fn get_marker(buffer: &str) -> Option<usize> {
    let chars: Vec<char> = buffer.chars().collect();
    let len = buffer.len();

    for index in 0..len {
        let cur = &chars[index..std::cmp::min(index + 4, len - 1)];
        let has_same_chars = cur
            .into_iter()
            .any(|char| cur.into_iter().filter(|f| **f == *char).count() > 1);

        if !has_same_chars {
            println!("{} {:?} {}", index, cur, (index + 4) as i32);
            return Some(index + 4);
        }
    }
    return None;
}
fn main() {
    let file_content = fs::read_to_string("./input.txt").expect("Error reading file.");

    let res = get_marker(&file_content);

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
    fn test() {
        assert_eq!(get_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(get_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(get_marker("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(get_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }
}
