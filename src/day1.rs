pub struct Day1 {
    pub val: u32,
}


impl Day1 {
    pub fn numeric_counts(input: &Vec<String>) -> u32 {
        let mut val = 0;
        for line in input.iter() {
            let mut first_num = None;
            let mut last_num = None;
            for c in line.chars() {
                if c.is_numeric() {
                    let segment = c.to_digit(10).unwrap();
                    if first_num == None {
                        first_num = Some(segment);
                    } else {
                        last_num = Some(segment);
                    }
                }
            }
            if let None = last_num {
                last_num = first_num
            }
            val += (first_num.unwrap() * 10) + last_num.unwrap();
        }
        val
    }

    pub fn part1() -> Day1 {
        let input = std::fs::read_to_string("./data/day1.txt").unwrap();
        let mut lines = vec![];
        for line in input.lines() {
            lines.push(line.to_string());
        }
        let val = Day1::numeric_counts(&lines);
        Day1 { val }
    }

    pub fn part2() -> Day1 {
        let input = std::fs::read_to_string("./data/day1.txt").unwrap();
        let mut lines = vec![];
        let words = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        for line in input.lines() {
            let mut i = 0;
            let mut seq = String::new();
            while i < line.len() {
                let mut found = false;
                for (j, w) in words.iter().enumerate() {
                    if line[i..].starts_with(w) {
                        seq.push_str((j + 1).to_string().as_str());
                        found = true;
                        break;
                    }
                }
                if !found {
                    seq.push(line.chars().nth(i).unwrap()); 
                }
                // words are legally allowed to overlap (twone, etc)
                // always skip 1, so we catch these
                i += 1;
            }
            lines.push(seq);
        }
        let val = Day1::numeric_counts(&lines);
        Day1 { val }
    }
}

// test for day1
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let day1 = Day1::part1();
        assert_eq!(day1.val, 0);
    }

    #[test]
    fn test_day1_part2() {
        let day1 = Day1::part2();
        assert_eq!(day1.val, 0);
    }
}
