pub fn solve3(input: &str) -> crate::Solutions {
    let banks = input.lines().collect::<Vec<&str>>();
    let mut best_batteries: Vec<u32> = Vec::new();
    for bank in &banks {
        let first_index = find_max(&bank[..bank.len() - 1]);
        let second_index = find_max(&bank[first_index as usize + 1..]); //offset by first_index
        best_batteries.push(
            bank.get(first_index as usize..first_index as usize + 1)
                .unwrap_or("0")
                .parse::<u32>()
                .unwrap()
                * 10
                + &bank[first_index as usize + 1..]
                    .get(
                        second_index as usize
                            ..second_index as usize + 1,
                    )
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap(),
        );
    }
    let mut indexes: [usize;12] = [0;12];
    let mut batteries2: Vec<u64> = Vec::new();
    for bank in &banks{
        indexes[0] = find_max(&bank[..bank.len() - 11]) as usize;
        for i in 1..12{
            indexes[i] = find_max(&bank[indexes[i-1]+1..bank.len() -(11-i)]) as usize + indexes[i-1]+1;
        }
        let mut b2 = 0;
        for index in indexes.iter(){
            b2 = b2*10 + bank.get(*index..*index+1).unwrap_or("0").parse::<u64>().unwrap();
        }
        batteries2.push(b2);
    }
    Solutions::Three {
        sum1: best_batteries.iter().sum(),
        batteries1: best_batteries,
        sum2: batteries2.iter().sum(),
        batteries2: batteries2,
    }
}

//Return the index of the maximum value in the bank
fn find_max(bank: &str) -> u32 {
    let mut max = 0;
    for i in 0..bank.len() {
        let val = bank.get(i..i + 1).unwrap_or("0");
        if val > bank.get(max..max + 1).unwrap_or("0") {
            max = i;
        }
    }
    max as u32
}

#[cfg(test)]
mod test_03 {
    use super::*;
    #[test]
    fn test_find_max() {
        let bank = "8749";
        assert_eq!(find_max(bank), 3);
        assert_eq!(find_max(&bank[..bank.len() - 1]), 0);
        assert_eq!(find_max(&bank[1..bank.len()-1]), 0);
        let bank2 = "1111198";
        assert_eq!(find_max(bank2), 5);
        assert_eq!(find_max(&bank2[5..])+5+1,6);
    }
}
