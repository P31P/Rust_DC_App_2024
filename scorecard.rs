use std::io;

fn bonus1(arr: &[[i32; 6]; 10], over: usize) -> i32 {
    for i in 0..6 {
        if arr[over][i] < 4 {
            return 0;
        }
    }
    1
}

fn bonus2(arr: &[[i32; 6]; 10], over: usize) -> i32 {
    if arr[over][0] == 6 && arr[over][1] == 6 {
        2
    } else {
        0
    }
}

fn bonus3(arr: &[[i32; 6]; 10], over: usize) -> i32 {
    if arr[over][4] == 6 && arr[over][5] == 6 {
        3
    } else {
        0
    }
}

fn maiden(arr: &[[i32; 6]; 10], over: usize) -> i32 {
    for i in 0..6 {
        if arr[over][i] != 0 {
            return 0;
        }
    }
    4
}

fn main() {
    let mut overs = [[0; 6]; 10];
    let mut max = [0; 10];
    let mut score = 0;

    let stdin = io::stdin();
    
    for i in 0..10 {
        max[i] = 0;
        for j in 0..6 {
            let mut input = String::new();
            stdin.read_line(&mut input).expect("Failed to read line");
            overs[i][j] = input.trim().parse().expect("Please type a number!");

            score += overs[i][j];
            if overs[i][j] > max[i] {
                max[i] = overs[i][j];
            }
        }
    }

    let mut bonus = [0; 10];
    let mut v: Vec<i32> = Vec::new();

    for i in 0..10 {
        if maiden(&overs, i) == 3 {
            bonus[i] = 3;
            bonus[i + 1] = 0;
        } else if max[i] > 4 {
            if bonus1(&overs, i) == 1 {
                bonus[i] = 1;
            } else {
                bonus[i] = bonus2(&overs, i) + bonus3(&overs, i);
            }
        }

        match bonus[i] {
            1 => {
                for j in i + 1..i + 3 {
                    if j < 10 {
                        for k in 0..6 {
                            score += overs[j][k];
                        }
                        v.push(j as i32);
                    }
                }
            }
            2 => {
                v.push(-(i as i32));
                for j in 3..6 {
                    score += overs[i][j];
                }
            }
            3 => {
                if i != 9 {
                    for j in 0..6 {
                        score += overs[i + 1][j];
                        v.push((i + 1) as i32);
                    }
                }
            }
            5 => {
                for j in 3..6 {
                    score += overs[i][j];
                }
                if i != 9 {
                    for j in 0..6 {
                        score += overs[i + 1][j];
                        v.push((i + 1) as i32);
                    }
                }
                v.push(-(i as i32));
            }
            _ => {}
        }
    }

    for i in 0..10 {
        let mut flag = false;
        for &val in &v {
            if val == i as i32 {
                flag = true;
                break;
            }
        }

        for j in 0..6 {
            if flag {
                if overs[i][j] >= 4 {
                    print!("*{}x2 ", overs[i][j]);
                } else {
                    print!(" {}x2 ", overs[i][j]);
                }
            } else {
                if overs[i][j] >= 4 {
                    print!("*{} ", overs[i][j]);
                } else {
                    print!(" {} ", overs[i][j]);
                }
            }
        }
        println!();
    }

    if bonus[8] == 1 || bonus[9] == 3 {
        let mut extra_over = [0; 6];
        for i in 0..6 {
            let mut input = String::new();
            stdin.read_line(&mut input).expect("Failed to read line");
            extra_over[i] = input.trim().parse().expect("Please type a number!");

            score += 2 * extra_over[i];
            if extra_over[i] >= 4 {
                print!("*{}x2 ", extra_over[i]);
            } else {
                print!(" {}x2 ", extra_over[i]);
            }
        }
        println!();
    } else if bonus[9] == 1 {
        let mut extra1 = [0; 6];
        let mut extra2 = [0; 6];
        for i in 0..6 {
            let mut input = String::new();
            stdin.read_line(&mut input).expect("Failed to read line");
            extra1[i] = input.trim().parse().expect("Please type a number!");

            score += 2 * extra1[i];
            if extra1[i] >= 4 {
                print!("*{}x2 ", extra1[i]);
            } else {
                print!(" {}x2 ", extra1[i]);
            }
        }
        println!();

        for i in 0..6 {
            let mut input = String::new();
            stdin.read_line(&mut input).expect("Failed to read line");
            extra2[i] = input.trim().parse().expect("Please type a number!");

            score += 2 * extra2[i];
            if extra2[i] >= 4 {
                print!("*{}x2 ", extra2[i]);
            } else {
                print!(" {}x2 ", extra2[i]);
            }
        }
        println!();
    }

    println!("{}", score);
}
