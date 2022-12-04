fn main() {
    //day_1();
    //day_2();
    //day_3();
    day_4();
}

fn read_input(day: i32) -> String {
    std::fs::read_to_string(format!("input{day}.txt")).expect("couldn't read input file")
}

fn day_4() {
    let input = read_input(4);
    let lines: Vec<&str> = input.lines().collect();

    let mut overlaps = 0;
    for line in lines {
        let ranges: Vec<&str> = line.split(",").collect();
        let r1: Vec<&str> = ranges[0].split("-").collect();
        let l1 = r1[0].parse::<i32>().unwrap();
        let u1 = r1[1].parse::<i32>().unwrap();

        let r2: Vec<&str> = ranges[1].split("-").collect();
        let l2 = r2[0].parse::<i32>().unwrap();
        let u2 = r2[1].parse::<i32>().unwrap();

        // part 1
        // if l1 <= l2 && u1 >= u2 || l1 >= l2 && u1 <= u2 {
        //     overlaps += 1;
        // }
        
        if l1 <= u2 && u1 >= u2 || l2 <= u1 && u2 >= u1 {
            overlaps += 1;
        }
    }

    println!("{}", overlaps);
}

fn day_3() {
    let input = read_input(3);
    let lines: Vec<&str> = input.lines().collect();

    let mut score = 0;

    // part 1
    // for line in lines {
    //     let first = &line[..(line.len() / 2)];
    //     let last = &line[(line.len() / 2)..];

    //     let mut ch: char = ' ';
    //     for c1 in first.chars() {
    //         for c2 in last.chars() {
    //             if c1 == c2 {
    //                 ch = c1;
    //                 break;
    //             }
    //         }
    //         if ch != ' ' {
    //             break;
    //         }
    //     }

    //     if ch.is_uppercase() {
    //         score += (ch as i32) - ('A' as i32) + 27;
    //     }

    //     if ch.is_lowercase() {
    //         score += (ch as i32) - ('a' as i32) + 1; 
    //     }
    // }

    for i in 0..lines.len() {
        if i % 3 != 0 {
            continue;
        }

        for ch in lines[i].chars() {
            if lines[i + 1].contains(ch) && lines[i + 2].contains(ch) {
                if ch.is_uppercase() {
                    score += (ch as i32) - ('A' as i32) + 27;
                }

                if ch.is_lowercase() {
                    score += (ch as i32) - ('a' as i32) + 1; 
                }

                break;
            }
        }
    }

    println!("{}", score);
}

fn day_2() {
    let input = read_input(2);
    let lines: Vec<&str> = input.lines().collect();

    let mut score = 0;
    // part 1
    //for line in lines {
    //    let other = line.chars().nth(0).unwrap();
    //    let your = line.chars().nth(2).unwrap();

    //    score += match your {
    //        'X' => 1,
    //        'Y' => 2,
    //        'Z' => 3,
    //        _ => panic!("")
    //    };
    //    
    //    score += match (other, your) {
    //        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
    //        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
    //        _ => 0
    //    };
    //}
    
    for line in lines {
        let other = line.chars().nth(0).unwrap();
        let your = line.chars().nth(2).unwrap();

        score += match your {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!()
        };

        score += match (other, your) {
            ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3,
            ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1,
            _ => 2
        };
    }
    println!("{}", score);
}

fn day_1() {
    let input = read_input(1);

    let mut calories: Vec<i32> = vec![];
    let mut current = 0;

    for line in input.lines() {
        if line == "" {
            calories.push(current);
            current = 0;
        }
        else {
            current += line.parse::<i32>().unwrap();
        }
    }

    let mut max = [0; 3];
    for cal in calories {
        if cal > max[0] {
            max[2] = max[1];
            max[1] = max[0];
            max[0] = cal;
        }
        else if cal > max[1] {
            max[2] = max[1];
            max[1] = cal;
        }
        else if cal > max[2] {
            max[2] = cal;
        }
    }
    // just use max[0] for part 1
    println!("{}", max[0] + max[1] + max[2]);
}
