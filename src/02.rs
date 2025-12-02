use std::vec;

pub fn solve2(input: &str) -> crate::Solutions {
    let ranges: Vec<&str> = input.split(",").collect();
    let mut invalids: Vec<u64> = vec![];
    let mut complete_invalids: Vec<u64> = vec![];
    for range in ranges {
        let (min, max) = match numbers(range) {
            None => {
                continue;
            }
            Some((a, b)) => (a, b),
        };

        for n in min..max + 1 {
            let stringn = n.to_string();
            let len = stringn.len();

            // check if a pattern repeats exactly twice
            let half_len = len / 2;
            if len % 2 == 0 {
                let pattern = &stringn[0..half_len];
                let mut is_repeating = true;

                for i in (half_len..len).step_by(half_len) {
                    if &stringn[i..i + half_len] != pattern {
                        is_repeating = false;
                        break;
                    }
                }

                if is_repeating {
                    invalids.push(n);
                    continue;
                }
            }

            // check for patterns with more repetitions
            for substring_len in 1..=len / 2 {
                if len % substring_len != 0 {
                    continue;
                }

                let pattern = &stringn[0..substring_len];
                let mut is_repeating = true;

                for i in (substring_len..len).step_by(substring_len) {
                    if &stringn[i..i + substring_len] != pattern {
                        is_repeating = false;
                        break;
                    }
                }

                if is_repeating {
                    complete_invalids.push(n);
                    break;
                }
            }
        }
    }
    let sum1 = invalids.iter().sum::<u64>();
    return Solutions::Two {
        sum: sum1,
        elements: invalids,
        sum_complete: complete_invalids.iter().sum::<u64>() + sum1,
        complete_elements: complete_invalids,
    };
}

fn numbers(s: &str) -> Option<(u64, u64)> {
    let nums = s.split('-').collect::<Vec<&str>>();
    let min = match nums[0].parse::<u64>() {
        Ok(v) => v,
        Err(e) => panic!("Invalid number on input: {}", e),
    };
    let max = match nums[1].parse::<u64>() {
        Ok(v) => v,
        Err(e) => panic!("Invalid number on input: {}", e),
    };
    if nums[0][0..1] == *"0" || nums[1][0..1] == *"0" {
        return None;
    }
    Option::Some((min, max))
}

#[cfg(test)]
mod test2 {
    use super::*;
    #[test]
    fn test_numbers() {
        assert_eq!(numbers("123-456"), Some((123, 456)));
        assert_eq!(numbers("001-456"), None);
    }
    #[test]
    fn test_solve2() {
        let s1 = solve2("10-25");
        match s1 {
            crate::Solutions::Two { elements, .. } => {
                assert_eq!(elements, vec![11, 22]);
            }
            _ => panic!("Expected Solutions::Two"),
        }

        let s2 = solve2("11-22");
        match s2 {
            crate::Solutions::Two {
                sum,
                elements,
                sum_complete: _,
                complete_elements: _,
            } => {
                assert_eq!(sum, 33);
                assert_eq!(elements, vec![11, 22]);
            }
            _ => panic!("Expected Solutions::Two"),
        }

        let s3 = solve2("1188511880-1188511890");
        match s3 {
            crate::Solutions::Two {
                sum,
                elements,
                sum_complete,
                complete_elements,
            } => {
                assert_eq!(sum, 1188511885);
                assert_eq!(elements, vec![1188511885]);
                assert_eq!(sum_complete, sum);
                assert_eq!(complete_elements, vec![]);
            }
            _ => panic!("Expected Solutions::Two"),
        }
        let s4 = solve2("222220-222224");
        match s4 {
            crate::Solutions::Two {
                sum,
                elements,
                sum_complete,
                complete_elements,
            } => {
                assert_eq!(sum, 222222);
                assert_eq!(elements, vec![222222]);
                assert_eq!(sum_complete, sum);
                assert_eq!(complete_elements, vec![]);
            }
            _ => panic!("Expected Solutions::Two"),
        }
    }
}
