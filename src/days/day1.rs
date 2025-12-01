pub fn day1(input: &mut String) -> String {
    // R/L <int> \n
    // parts: start at 50, count when it reaches 0
    let mut count_1 = 0;
    let mut count_2 = 0;
    let mut rotation = 50;
    for line in input.lines() {
        let rot = line.chars().next().unwrap();
        let num: i32 = line[1..].parse().unwrap();
        match rot {
            'L' => rotation -= num,
            'R' => rotation += num,
            _ => println!("idk | {} | {}", rot, num),
        }
        // println!("rotation {} direction {}", rotation, num);
        // modulo 100, read: dial is only from 0 - 100 so just letting it go free would cause it to
        // be super negative
        rotation = ((rotation % 100) + 100) % 100;
        if rotation == 0 {
            count_1 += 1;
        }
    }
    println!("{}", count_1.to_string());

    //part 2
    // Be careful: if the dial were pointing at 50, a single rotation like R1000 would cause the dial to point at 0 ten times before returning back to 50!
    // how many times does it count to 100? have to count position?
    rotation = 50;
    for line in input.lines() {
        let rot = line.chars().next().unwrap();
        let num: i32 = line[1..].parse().unwrap();

        for _ in 0..num {
            if rot == 'L' {
                rotation -= 1;
                if rotation == -1 {
                    rotation = 99;
                    // count_2 += 1; // crossed 0
                }
            } else if rot == 'R' {
                rotation += 1;
                if rotation == 100 {
                    rotation = 0;
                    // count_2 += 1; // crossed 0
                }
            }
            if rotation == 0 {
                count_2 += 1;
            }
        }
    }
    count_2.to_string() // 5935 too low 8873 too high
}
