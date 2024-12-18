use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let a = u64::from_str_radix(&lines.next().unwrap()[12..], 10).unwrap();
    let b = u64::from_str_radix(&lines.next().unwrap()[12..], 10).unwrap();
    let c = u64::from_str_radix(&lines.next().unwrap()[12..], 10).unwrap();
    lines.next();
    let program = lines.next().unwrap()[9..]
        .as_bytes()
        .iter()
        .filter_map(|&x| {
            if x >= b'0' && x <= b'9' {
                Some(x - b'0')
            } else {
                None
            }
        })
        .collect_vec();
    let mut out = Vec::new();
    run_machine(&program, a, b, c, &mut out);
    out.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    lines.next();
    let b = u64::from_str_radix(&lines.next().unwrap()[12..], 10).unwrap();
    let c = u64::from_str_radix(&lines.next().unwrap()[12..], 10).unwrap();
    lines.next();

    let program = lines.next().unwrap()[9..]
        .as_bytes()
        .iter()
        .filter_map(|&x| {
            if x >= b'0' && x <= b'9' {
                Some(x - b'0')
            } else {
                None
            }
        })
        .collect_vec();

    let mut out = Vec::new();
    let mut stack = Vec::new();
    stack.push((0, 0));
    let program_len = program.len();
    let mut good_a = Vec::new();
    loop {
        let Some((depth, a)) = stack.pop() else {
            break;
        };
        for i in 0..8 {
            let new_a = (a << 3) + i;
            run_machine(&program, new_a, b, c, &mut out);
            /* println!("a: {}", new_a);
            println!(
                "out: {}",
                out.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ); */
            /* println!(
                "goal: {}",
                program
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ); */
            if out.len() < depth + 1 {
                continue;
            }
            //if out[0..depth + 1] == program[0..depth + 1] {
            if out[(out.len() - depth - 1)..] == program[(program_len - depth - 1)..] {
                if depth == program_len - 1 {
                    good_a.push(new_a);
                    continue;
                }
                stack.push((depth + 1, new_a));
            }
            out.clear();
        }
    }
    /*for a in good_a.iter() {
        println!("valid_a:{}", a);
    }*/
    *good_a.iter().min().unwrap()
}

fn run_machine(program: &[u8], a: u64, b: u64, c: u64, out: &mut Vec<u8>) {
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut ip = 0;
    let progam_len = program.len();
    while ip < progam_len {
        let literal = program[ip + 1];
        let combo = match literal {
            4 => a,
            5 => b,
            6 => c,
            7 => {
                unreachable!()
            }
            x => x as u64,
        };
        /* println!(
            "{} {} {} {} {} {} {}",
            ip,
            program[ip],
            program[ip + 1],
            operand,
            a,
            b,
            c
        ); */
        match program[ip] {
            0 => {
                a = a >> combo;
            }
            1 => {
                b ^= literal as u64;
            }
            2 => {
                b = combo % 8;
            }
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                out.push((combo % 8) as u8);
            }
            6 => {
                b = a >> combo;
            }
            7 => {
                c = a >> combo;
            }
            _ => {
                unreachable!()
            }
        }
        ip += 2;
    }
}
