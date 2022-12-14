#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    //day_1();
    //day_2();
    //day_3();
    //day_4();
    //day_5();
    //day_6();
    //day_7();
    //day_8();
    day_9();
}

fn read_input(day: i32) -> String {
    std::fs::read_to_string(format!("input{day}.txt")).expect("couldn't read input file")
}

fn day_9() {
    let input = read_input(9);
    let lines: Vec<&str> = input.lines().collect();
    
    // part 1
    // let mut pos_h: (i32, i32) = (0, 0);
    // let mut pos_t: (i32, i32) = (0, 0);
    // let mut visited = vec![pos_t];
    // 
    // for line in lines {
    //     let count = line[2..line.len()].to_string().parse::<i32>().unwrap();
    //     let direction = match &line[0..1] {
    //         "L" => (-1, 0),
    //         "R" => (1, 0),
    //         "D" => (0, -1),
    //         "U" => (0, 1),
    //         c => panic!("invalid char {}", c),
    //     };
    //     for _ in 0..count {
    //         pos_h.0 += direction.0;
    //         pos_h.1 += direction.1;
    //         if (pos_h.0 - pos_t.0).abs() > 1 || (pos_h.1 - pos_t.1).abs() > 1 {
    //             pos_t.0 += (pos_h.0 - pos_t.0).clamp(-1, 1);
    //             pos_t.1 += (pos_h.1 - pos_t.1).clamp(-1, 1);
    //             if !visited.contains(&pos_t){
    //                 visited.push(pos_t);
    //             }
    //         }
    //     }
    // }

    let mut pos_k = [(0i32, 0i32); 10]; 
    let mut visited = vec![(0, 0)];

    for line in lines {
        let count = line[2..line.len()].to_string().parse::<i32>().unwrap();
        let direction = match &line[0..1] {
            "L" => (-1, 0),
            "R" => (1, 0),
            "D" => (0, -1),
            "U" => (0, 1),
            c => panic!("invalid char {}", c),
        };
        for _ in 0..count {
            // move head
            pos_k[0].0 += direction.0;
            pos_k[0].1 += direction.1;
            for i in 1..pos_k.len() {
                let pos_h = pos_k[i - 1];
                let pos_t = &mut pos_k[i];
                if (pos_h.0 - pos_t.0).abs() > 1 || (pos_h.1 - pos_t.1).abs() > 1 {
                    pos_t.0 += (pos_h.0 - pos_t.0).clamp(-1, 1);
                    pos_t.1 += (pos_h.1 - pos_t.1).clamp(-1, 1);
                }
            }

            if !visited.contains(&pos_k[pos_k.len() - 1]){
                visited.push(pos_k[pos_k.len() - 1]);
            }
        }
    }

    print!("{}", visited.len());
}

fn day_8() {
    let input = read_input(8);
    let lines: Vec<&str> = input.lines().collect();

    let mut grid: Vec<Vec<i32>> = vec![];
    for line in lines {
        let row = line.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        grid.push(row);
    }
    let mut count = 0;
    // part 1 
    // for k in 0..grid.len() {
    //     let row = &grid[k];
    //     for i in 0..row.len() {
    //         let mut visible = true;
    //         let num = row[i];

    //         //horizontal
    //         for j in 0..i {
    //             if row[j] >= num {
    //                 visible = false;
    //             } 
    //         }
    //         if !visible {
    //             visible = true;
    //             for j in i + 1..row.len() {
    //                 if row[j] >= num {
    //                     visible = false;
    //                 }
    //             }
    //         }

    //         // vertical
    //         if !visible {
    //             visible = true;
    //             for j in 0..k {
    //                 if grid[j][i] >= num {
    //                     visible = false;
    //                 }
    //             }
    //         }
    //         if !visible {
    //             visible = true;
    //             for j in k + 1..grid.len() {
    //                 if grid[j][i] >= num {
    //                     visible = false;
    //                 }
    //             }
    //         }
    //         if visible {
    //             count += 1;
    //         }
    //     }
    // }
    let mut top = 0;
    for k in 0..grid.len() {
        let row = &grid[k];
        for i in 0..row.len() {
            let mut score = 1;
            let num = row[i];

            //horizontal
            for j in (0..i).rev() {
                if row[j] >= num {
                    score *= i - j;
                    break;
                } 
                if j == 0 {
                score *= i;
                }
            }
            for j in i + 1..row.len() {
                if row[j] >= num {
                    score *= j - i;
                    break;
                }
                if j == row.len() - 1 {
                score *= row.len() - i - 1;
                }
            }

            // vertical
            for j in (0..k).rev() {
                if grid[j][i] >= num {
                    score *= k - j;
                    break;
                }
                if j == 0 {
                score *= k;
                }
            }
            for j in k + 1..grid.len() {
                if grid[j][i] >= num {
                    score *= j - k;
                    break;
                }
                if j == grid.len() - 1 {
                score *= grid.len() - k - 1;
                }
            }

            if score > top {
                top = score;
            }
        }
    }
    println!("{}", top);
}

fn day_7() {
    let input = read_input(7);
    let lines: Vec<&str> = input.lines().collect();

    let mut current: Vec<i32> = vec![];
    let mut done: Vec<i32> = vec![];

    for line in lines {
        if line.starts_with("$ cd") {
            if line.contains("..") {
                done.push(current.pop().unwrap());
            }
            else {
                current.push(0);
            }
        }
        else if !line.starts_with("$ ls") && !line.starts_with("dir") {
            let num = line[0..line.find(char::is_whitespace).unwrap()].parse::<i32>().unwrap();
            for i in &mut current {
                *i += num;
            }
        }
    }
    done.extend(&current);
    //part 1
    //println!("{}", done.iter().filter(|n| **n <= 100000).sum::<i32>());

    let max = done.iter().max().unwrap();
    let unused = 70000000 - max;
    let required = 30000000 - unused;
    println!("{}", done.iter().filter(|n| **n >= required).min().unwrap());
}

fn day_6() {
    let input = read_input(6);

    // part 1
    const MESSAGE_COUNT: usize = 14;

    let mut last = [' '; MESSAGE_COUNT];
    let ch = input.as_bytes().iter().map(|b| *b as char).collect::<Vec<char>>();
    for i in 0..input.len() {
        if i < MESSAGE_COUNT {
            last[i] = ch[i];
            continue;
        }
        let mut contains: bool = false;
        for i in 0..last.len() {
            for j in 0..last.len() {
                if last[i] == last[j] && i != j {
                    contains = true;
                }
            }
        }
        if !contains {
            println!("{}", i);
            return;
        }
        last[i % MESSAGE_COUNT] = ch[i];
    }
}

fn day_5() {
    let input = read_input(5);
    let lines: Vec<&str> = input.lines().collect();

    let mut crates: HashMap<usize, Vec<char>> = HashMap::new();
    for i in (0..8).rev().collect::<Vec<usize>>() {
        let line = lines[i];
        for j in 0..9 {
            if !crates.contains_key(&j) {
                crates.insert(j, vec![]);
            }
            let index = 1 + j * 4;
            if line.as_bytes()[index] as char != ' ' {
                crates.get_mut(&j).unwrap().push(line.as_bytes()[index] as char);
            }
        }
    }

    for line in &lines[10..lines.len()] {
         let count_index = line.find(|c| char::is_digit(c, 10)).unwrap();
         let count_end_index = line[count_index..line.len()].find(char::is_whitespace).unwrap() + count_index;
         let count = &line[count_index..count_end_index].parse::<usize>().unwrap();

         let from_index = line[count_end_index..line.len()].find(|c| char::is_digit(c, 10)).unwrap() + count_end_index;
         let from = &line[from_index..from_index + 1].parse::<usize>().unwrap();

         let to_index = line[from_index + 1..line.len()].find(|c| char::is_digit(c, 10)).unwrap() + from_index + 1;
         let to = &line[to_index..to_index + 1].parse::<usize>().unwrap();

         //part 1
         //for _ in 0..*count {
         //     let ch = crates.get_mut(&(from - 1)).unwrap().pop().unwrap();
         //     crates.get_mut(&(to - 1)).unwrap().push(ch);
         // }
         let vec = crates.get_mut(&(from - 1)).unwrap();
         let len = vec.len();
         for _ in 0..*count {
             let vec = crates.get_mut(&(from - 1)).unwrap();
             let ch = vec.remove(len - count);
             crates.get_mut(&(to - 1)).unwrap().push(ch);
         }
    }

    for i in 0..crates.len() {
        print!("{}", crates.get(&i).unwrap()[crates.get(&i).unwrap().len() - 1]);
    }
    println!();
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
