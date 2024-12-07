use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut out = 0;
    for line in input.lines() {
        let mut line = line.split(':');
        let total = u64::from_str_radix(line.next().unwrap(), 10).unwrap();
        let row = line.next().unwrap().split_ascii_whitespace();
        let row: Vec<u64> = row
            .rev()
            .map(|a| u64::from_str_radix(a, 10).unwrap())
            .collect();
        if check_row(total, &row) {
            out += total;
        }
    }
    out
}
fn check_row(total: u64, row: &Vec<u64>) -> bool {
    let mut stack = Vec::new();
    let mut depht = 0;
    let mut branch_stack = Vec::new();
    branch_stack.reserve(row.len());
    stack.reserve(row.len());
    branch_stack.push(0);
    stack.push(total);
    'a: loop {
        let total = stack[depht];
        if depht == row.len() - 1 {
            if row[depht] == total {
                return true;
            }
            branch_stack.pop();
            stack.pop();
            depht -= 1;
            branch_stack[depht] += 1;
            continue 'a;
        }
        if branch_stack[depht] == 0 {
            if total % row[depht] != 0 {
                branch_stack[depht] = 1;
            } else {
                stack.push(total / row[depht]);
                branch_stack.push(0);
                depht += 1;
            }
        } else if branch_stack[depht] == 1 {
            if total < row[depht] {
                branch_stack[depht] = 2;
            } else {
                stack.push(total - row[depht]);
                branch_stack.push(0);
                depht += 1;
            }
        } else {
            if depht == 0 {
                break;
            }
            branch_stack.pop();
            stack.pop();
            depht -= 1;
            branch_stack[depht] += 1;
        }
    }
    return false;
}
pub fn part2(input: &str) -> impl Display {
    let mut out = 0;
    for line in input.lines() {
        let mut line = line.split(':');
        let total = u64::from_str_radix(line.next().unwrap(), 10).unwrap();
        let row = line.next().unwrap().split_ascii_whitespace();
        let row: Vec<u64> = row
            .rev()
            .map(|a| u64::from_str_radix(a, 10).unwrap())
            .collect();
        if check_row_concat(total, &row) {
            out += total;
        }
    }
    out
}
fn check_row_concat(total: u64, row: &Vec<u64>) -> bool {
    let mut stack = Vec::new();
    let mut depht = 0;
    let mut branch_stack = Vec::new();
    branch_stack.reserve(row.len());
    stack.reserve(row.len());
    branch_stack.push(0);
    stack.push(total);
    'a: loop {
        let total = stack[depht];
        if depht == row.len() - 1 {
            if row[depht] == total {
                return true;
            }
            branch_stack.pop();
            stack.pop();
            depht -= 1;
            branch_stack[depht] += 1;
            continue 'a;
        }
        if branch_stack[depht] == 0 {
            if total % row[depht] != 0 {
                branch_stack[depht] = 1;
            } else {
                stack.push(total / row[depht]);
                branch_stack.push(0);
                depht += 1;
            }
        } else if branch_stack[depht] == 1 {
            if total < row[depht] {
                branch_stack[depht] = 2;
            } else {
                stack.push(total - row[depht]);
                branch_stack.push(0);
                depht += 1;
            }
        } else if branch_stack[depht] == 2 {
            let mut digits_total = total;
            let mut digits_row = row[depht];
            let mut matches = true;
            'd: while digits_row > 0 {
                if digits_total % 10 != digits_row % 10 {
                    matches = false;
                    break 'd;
                }
                digits_total /= 10;
                digits_row /= 10;
            }
            if !matches {
                branch_stack[depht] = 3;
            } else {
                stack.push(digits_total);
                branch_stack.push(0);
                depht += 1;
            }
        } else {
            if depht == 0 {
                break;
            }
            branch_stack.pop();
            stack.pop();
            depht -= 1;
            branch_stack[depht] += 1;
        }
    }
    return false;
}
