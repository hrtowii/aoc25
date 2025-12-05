pub fn day3(input: &mut String) -> String {
    // need 2 pointers
    // given line of numbers, find which 2 in order (but not consecutively)
    // meaning: i < j AND i * 10 + j is greatest in that line
    // have the largest value
    // when added
    let mut sum = 0;
    let _ = input
        .lines()
        .map(|line| {
            // while i < j {
            //     let mut left = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            //     let mut right = line.chars().nth(j).unwrap().to_digit(10).unwrap();
            //     if sum <= left + right {
            //         sum = left + right;
            //     }
            //     i += 1;
            //     j -= 1;
            // } 2 pointers wont work cus its symmetric
            let digits: Vec<u32> = line
                .chars()
                .filter_map(|character| character.to_digit(10))
                .collect();
            // part 1
            // let mut best = 0;
            // let length = digits.len();
            // let mut i = digits[0];
            // for j in 1..length {
            //     let value = i * 10 + digits[j];
            //     if value > best {
            //         best = value
            //     };
            //     if digits[j] > i {
            //         i = digits[j]
            //     }
            // }
            // sum += best;

            // part 2
            let mut picked: Vec<u32> = Vec::new();
            // picked has 12 numbers.
            // check remaining and compare vs now
            // at start: 0 vs 12
            // halfway: 6 vs 6
            // if there's something halfway thts btr than previous num, kick out
            for i in 0..digits.len() {
                let d = digits[i];
                while !picked.is_empty() {
                    let last = picked[picked.len() - 1];
                    let remaining = digits.len() - i;
                    if picked.len() - 1 + remaining < 12 {
                        break;
                    }
                    if d > last {
                        picked.pop();
                    } else {
                        break;
                    }
                }
                picked.push(d);
            }
            let chosen = &picked[..12];
            let mut value = 0;
            for &d in chosen {
                value = value * 10 + d as u128;
            }
            sum += value;
        })
        .collect::<Vec<_>>();
    sum.to_string()
}
