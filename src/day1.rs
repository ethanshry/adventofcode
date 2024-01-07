struct Day1 {
    pub val: u32
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
            println!("{}", line);
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
        Day1 {
            val
        }
    }

    pub fn part2() -> Day1 {
        let input = std::fs::read_to_string("./data/day1.txt").unwrap();
        let mut lines = vec![];
        let words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for line in input.lines() {
            let i = 0;
            let mut word = "";
            while i < line.len() {
                // TODO
            }
            for c in 0..line.len() {
               
            }
            lines.push(line.to_string());
        }
        let val = Day1::numeric_counts(&lines);
        Day1 {
            val
        }
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