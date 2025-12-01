use std::fs;
const TEST: bool = false;
struct Safe {
    value: i16,
    tzero: u16,
}
impl Safe {
    fn new() -> Self {
        Safe {
            value: 50,
            tzero: 0,
        }
    }
    fn add(&mut self, mut v: i16) {
        let times100 = v / 100;
        v = v % 100;
        self.tzero += times100 as u16 + if v + self.value >= 100 { 1 } else { 0 };
        self.value = (self.value + v) % 100;
    }
    fn remove(&mut self, mut v: i16) {
        let times100 = v / 100;
        v = v % 100;
        self.tzero += times100 as u16 + if v >= self.value && self.value != 0 { 1 } else { 0 };
        self.value = (100 + self.value - v) % 100; // to avoid negative values since value and v are always between 0 and 99
    }
    fn get(&self) -> i16 {
        self.value
    }
    fn tzero(&self) -> u16 {
        self.tzero
    }
}
pub fn solve1() {
    let content =
        fs::read_to_string(if TEST { "01.test" } else { "01.data" }).expect("Unable to read file");
    let inputs = content.lines();
    let mut _answer: u16 = 0;
    let mut safe = Safe::new();
    for line in inputs {
        if line.get(0..1) == Some("L") {
            safe.remove(line[1..].parse::<i16>().unwrap());
        } else if line.get(0..1) == Some("R") {
            safe.add(line[1..].parse::<i16>().unwrap());
        } else {
            panic!("Invalid input");
        }
        if safe.get() == 0 {
            _answer += 1;
        }
        println!("Safe value: {}, tzero: {}", safe.get(), safe.tzero());
    }
    println!("Solution to problem 1: {:?}", _answer);
    println!("Solution to problem 2: {:?}", safe.tzero());
}
