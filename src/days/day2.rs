pub fn day2(input: &mut String) -> String {
    let mut total = 0;
    let mut stringy = String::new();
    let mut thing = input
        .split(',')
        .map(|pair| {
            let (a, b) = pair.split_once('-').unwrap();
            (
                a.trim().parse::<u64>().unwrap(),
                b.trim().parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<(u64, u64)>>();
    for tuple in thing {
        for number in tuple.0..=tuple.1 {
            stringy = number.to_string();
            let len = stringy.len();
            // part 1
            // let a = stringy.split_at(stringy.len() / 2);
            // if a.0 == a.1 {
            //     total += number;
            // }

            // part 2
            // check for any substrings
            // go up to half of original string, construct array from 0-string/2.len(), compare
            // against
            // if true, total += number
            if stringy.starts_with("0") {
                stringy.remove(0);
            }
            for substring_len in 1..=(len / 2) {
                if len % substring_len != 0 {
                    continue;
                }
                let pattern = &stringy[0..substring_len];
                if pattern.repeat(len / substring_len) == stringy {
                    total += number;
                    break;
                }
            }
        }
    }
    total.to_string()
}

// _ -> and no dash and no comma -> middle of sequence
// _ -> and no dash but comma -> start of thing
// , -> reset ranges, set comma to true
// dash -> populate to 2nd vec
