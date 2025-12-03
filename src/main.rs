include!("01.rs");
include!("02.rs");
include!("03.rs");
pub enum Solutions {
    One {},
    Two {
        sum: u64,
        elements: Vec<u64>,
        sum_complete: u64,
        complete_elements: Vec<u64>,
    },
    Three {
        sum1: u32,
        batteries1: Vec<u32>,
        sum2: u64,
        batteries2: Vec<u64>,
    },
}

impl std::fmt::Debug for Solutions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solutions::One {} => write!(f, "Solution One"),
            Solutions::Two {
                sum,
                elements,
                sum_complete,
                complete_elements,
            } => {
                write!(
                    f,
                    "Solution Two: sum = {}, elements = {:?}, sum_complete = {}, complete_elements = {:?}",
                    sum,
                    if elements.len() < 10 {
                        format!("{:?}", elements)
                    } else {
                        format!("[{} ... {}]", elements[0], elements[elements.len() - 1])
                    },
                    sum_complete,
                    if complete_elements.len() < 10 {
                        format!("{:?}", complete_elements)
                    } else {
                        format!(
                            "[{} ... {}]",
                            complete_elements[0],
                            complete_elements[complete_elements.len() - 1]
                        )
                    }
                )
            }
            Solutions::Three {
                sum1,
                batteries1,
                sum2,
                batteries2,
            } => {
                write!(
                    f,
                    "Solution Three: sum1 = {}, batteries1 = {:?}, sum2 = {}, batteries2 = {:?}",
                    sum1,
                    if batteries1.len() < 10 {
                        format!("{:?}", batteries1)
                    } else {
                        format!(
                            "[{} ... {}]",
                            batteries1[0],
                            batteries1[batteries1.len() - 1]
                        )
                    },
                    sum2,
                    if batteries2.len() < 10 {
                        format!("{:?}", batteries2)
                    } else {
                        format!(
                            "[{} ... {}]",
                            batteries2[0],
                            batteries2[batteries2.len() - 1]
                        )
                    }
                )
            }
        }
    }
}

use std::env;
// Args should be <exercise> <test>
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Put at least the exercise number in format 01, 02, ...");
    }
    let exercise = &args[1];
    let test = if args.contains(&"-t".to_string()) {
        true
    } else {
        false
    };
    solve(exercise, test);
}

fn solve(exercise: &str, test: bool) {
    let content = fs::read_to_string(filepath(exercise, test))
        .expect(format!("Failed to read file {}", filepath(exercise, test)).as_str());
    let solution = match exercise {
        "01" => solve1(test),
        "02" => solve2(&content),
        "03" => solve3(&content),
        _ => {
            panic!("Exercise not found");
        }
    };
    println!("Solution to exercise {}: {:?}", exercise, solution);
}

fn filepath(exercise: &str, test: bool) -> String {
    if test {
        format!("data/{}.test", exercise)
    } else {
        format!("data/{}.data", exercise)
    }
}

#[cfg(test)]
mod test_main {
    use super::*;
    #[test]
    fn test_solve() {
        solve("01", true);
        solve("02", true);
    }
    #[test]
    fn test_filepath() {
        assert_eq!(filepath("01", true), "data/01.test");
        assert_eq!(filepath("02", false), "data/02.data");
    }

    #[test]
    #[should_panic]
    fn test_solve_invalid_exercise() {
        solve("3", true);
    }
}
