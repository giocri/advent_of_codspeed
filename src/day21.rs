enum NumPadKey {
    A,
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
}
enum ArrowPadKey {
    A,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
const NUM_PAD_POSITIONS: [(u8, u8); 11] = [
    (3, 2),
    (3, 1),
    (2, 0),
    (2, 1),
    (2, 2),
    (1, 0),
    (1, 1),
    (1, 2),
    (0, 0),
    (0, 1),
    (0, 2),
];
const DEBUGSTRING: &str = "A^v<>";
const ARROW_PAD_POSITIONS: [(u8, u8); 5] = [(0, 2), (0, 1), (1, 1), (1, 0), (1, 2)];
const fn num_pad_key_to_int(key: NumPadKey) -> u8 {
    match key {
        NumPadKey::A => 0,
        NumPadKey::K0 => 1,
        NumPadKey::K1 => 2,
        NumPadKey::K2 => 3,
        NumPadKey::K3 => 4,
        NumPadKey::K4 => 5,
        NumPadKey::K5 => 6,
        NumPadKey::K6 => 7,
        NumPadKey::K7 => 8,
        NumPadKey::K8 => 9,
        NumPadKey::K9 => 10,
    }
}
const fn arrow_pad_key_to_int(key: ArrowPadKey) -> u8 {
    match key {
        ArrowPadKey::A => 0,
        ArrowPadKey::UP => 1,
        ArrowPadKey::DOWN => 2,
        ArrowPadKey::LEFT => 3,
        ArrowPadKey::RIGHT => 4,
    }
}

fn lookup_move_numpad() -> [u8; 726] {
    let mut table = [0u8; 11 * 11 * 6];
    for (key_1, pos1) in NUM_PAD_POSITIONS.iter().enumerate() {
        for (key_2, pos2) in NUM_PAD_POSITIONS.iter().enumerate() {
            let mut i = 0;
            if key_1 != key_2 {
                if pos1.1 > pos2.1 {
                    for _ in pos2.1..pos1.1 {
                        table[key_1 * 11 * 6 + key_2 * 6 + i] =
                            arrow_pad_key_to_int(ArrowPadKey::LEFT);
                        i += 1;
                    }
                }
                if pos1.0 < pos2.0 {
                    for _ in pos1.0..pos2.0 {
                        table[key_1 * 11 * 6 + key_2 * 6 + i] =
                            arrow_pad_key_to_int(ArrowPadKey::DOWN);
                        i += 1;
                    }
                }
                if pos1.1 < pos2.1 {
                    for _ in pos1.1..pos2.1 {
                        table[key_1 * 11 * 6 + key_2 * 6 + i] =
                            arrow_pad_key_to_int(ArrowPadKey::RIGHT);
                        i += 1;
                    }
                }
                if pos1.0 > pos2.0 {
                    for _ in pos2.0..pos1.0 {
                        table[key_1 * 11 * 6 + key_2 * 6 + i] =
                            arrow_pad_key_to_int(ArrowPadKey::UP);
                        i += 1;
                    }
                }
            }
        }
    }
    let key_1 = num_pad_key_to_int(NumPadKey::A) as usize;
    let key_2 = num_pad_key_to_int(NumPadKey::K1) as usize;
    table[key_1 * 11 * 6 + key_2 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 2] = arrow_pad_key_to_int(ArrowPadKey::LEFT);

    table[key_2 * 11 * 6 + key_1 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::RIGHT);
    table[key_2 * 11 * 6 + key_1 * 6 + 2] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    //###################################################
    let key_1 = num_pad_key_to_int(NumPadKey::K0) as usize;
    let key_2 = num_pad_key_to_int(NumPadKey::K1) as usize;
    table[key_1 * 11 * 6 + key_2 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 1] = arrow_pad_key_to_int(ArrowPadKey::LEFT);

    table[key_2 * 11 * 6 + key_1 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::RIGHT);
    table[key_2 * 11 * 6 + key_1 * 6 + 1] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    //###################################################
    let key_1 = num_pad_key_to_int(NumPadKey::A) as usize;
    let key_2 = num_pad_key_to_int(NumPadKey::K4) as usize;
    table[key_1 * 11 * 6 + key_2 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 1] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 2] = arrow_pad_key_to_int(ArrowPadKey::LEFT);
    table[key_1 * 11 * 6 + key_2 * 6 + 3] = arrow_pad_key_to_int(ArrowPadKey::LEFT);

    table[key_2 * 11 * 6 + key_1 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::RIGHT);
    table[key_2 * 11 * 6 + key_1 * 6 + 1] = arrow_pad_key_to_int(ArrowPadKey::RIGHT);
    table[key_2 * 11 * 6 + key_1 * 6 + 2] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    table[key_2 * 11 * 6 + key_1 * 6 + 3] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    //###################################################
    let key_1 = num_pad_key_to_int(NumPadKey::K0) as usize;
    let key_2 = num_pad_key_to_int(NumPadKey::K4) as usize;
    table[key_1 * 11 * 6 + key_2 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 1] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 2] = arrow_pad_key_to_int(ArrowPadKey::LEFT);

    table[key_2 * 11 * 6 + key_1 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::RIGHT);
    table[key_2 * 11 * 6 + key_1 * 6 + 1] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    table[key_2 * 11 * 6 + key_1 * 6 + 2] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    //###################################################
    let key_1 = num_pad_key_to_int(NumPadKey::A) as usize;
    let key_2 = num_pad_key_to_int(NumPadKey::K7) as usize;
    table[key_1 * 11 * 6 + key_2 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 1] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 2] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 3] = arrow_pad_key_to_int(ArrowPadKey::LEFT);
    table[key_1 * 11 * 6 + key_2 * 6 + 4] = arrow_pad_key_to_int(ArrowPadKey::LEFT);

    table[key_2 * 11 * 6 + key_1 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::RIGHT);
    table[key_2 * 11 * 6 + key_1 * 6 + 1] = arrow_pad_key_to_int(ArrowPadKey::RIGHT);
    table[key_2 * 11 * 6 + key_1 * 6 + 2] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    table[key_2 * 11 * 6 + key_1 * 6 + 3] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    table[key_2 * 11 * 6 + key_1 * 6 + 4] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    //###################################################
    let key_1 = num_pad_key_to_int(NumPadKey::K0) as usize;
    let key_2 = num_pad_key_to_int(NumPadKey::K7) as usize;
    table[key_1 * 11 * 6 + key_2 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 1] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 2] = arrow_pad_key_to_int(ArrowPadKey::UP);
    table[key_1 * 11 * 6 + key_2 * 6 + 3] = arrow_pad_key_to_int(ArrowPadKey::LEFT);

    table[key_2 * 11 * 6 + key_1 * 6 + 0] = arrow_pad_key_to_int(ArrowPadKey::RIGHT);
    table[key_2 * 11 * 6 + key_1 * 6 + 1] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    table[key_2 * 11 * 6 + key_1 * 6 + 2] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    table[key_2 * 11 * 6 + key_1 * 6 + 3] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    table
}
fn lookup_move_arrow_pad() -> [u8; 100] {
    let mut table = [0u8; 5 * 5 * 4];
    for (key_1, pos1) in ARROW_PAD_POSITIONS.iter().enumerate() {
        for (key_2, pos2) in ARROW_PAD_POSITIONS.iter().enumerate() {
            let mut i = 0;
            if key_1 != key_2 {
                if pos1.1 > pos2.1 {
                    for _ in pos2.1..pos1.1 {
                        table[key_1 * 5 * 4 + key_2 * 4 + i] =
                            arrow_pad_key_to_int(ArrowPadKey::LEFT);
                        i += 1;
                    }
                }
                if pos1.0 < pos2.0 {
                    for _ in pos1.0..pos2.0 {
                        table[key_1 * 5 * 4 + key_2 * 4 + i] =
                            arrow_pad_key_to_int(ArrowPadKey::DOWN);
                        i += 1;
                    }
                }
                if pos1.1 < pos2.1 {
                    for _ in pos1.1..pos2.1 {
                        table[key_1 * 5 * 4 + key_2 * 4 + i] =
                            arrow_pad_key_to_int(ArrowPadKey::RIGHT);
                        i += 1;
                    }
                }
                if pos1.0 > pos2.0 {
                    for _ in pos2.0..pos1.0 {
                        table[key_1 * 5 * 4 + key_2 * 4 + i] =
                            arrow_pad_key_to_int(ArrowPadKey::UP);
                        i += 1;
                    }
                }
            }
        }
    }
    let key_1 = arrow_pad_key_to_int(ArrowPadKey::A) as usize;
    let key_2 = arrow_pad_key_to_int(ArrowPadKey::LEFT) as usize;
    table[key_1 * 5 * 4 + key_2 * 4 + 0] = arrow_pad_key_to_int(ArrowPadKey::DOWN);
    table[key_1 * 5 * 4 + key_2 * 4 + 2] = arrow_pad_key_to_int(ArrowPadKey::LEFT);
    table
}
fn digit_sequence(input: &str, lookup_numpad: &[u8], lookup_arrow_pad: &[u8]) -> u32 {
    let bytes = input.as_bytes();
    let mut sequence: Vec<(u8, u8)> = Vec::new();
    let mut a = 0;
    for b in bytes {
        let b = match *b {
            b'A' => 0,
            x if x >= b'0' && x <= b'9' => (x + 1) - b'0',
            _ => {
                unreachable!()
            }
        };
        //print!("a:{} b:{} ", a, b);
        if a == b {
            let len = sequence.len();
            sequence[len - 1].1 += 1;
            continue;
        }
        let index = (a as usize * 11 * 6) + (b as usize * 6);
        let mut last = 0;
        a = b;
        for i in 0..6 {
            let next = lookup_numpad[index + i];
            if next == last {
                let len = sequence.len();
                sequence[len - 1].1 += 1;
                continue;
            }
            sequence.push((next, 1));
            if next == 0 {
                break;
            }
            last = next;
        }
    }
    for _ in 0..2 {
        let mut new_sequence: Vec<(u8, u8)> = Vec::new();
        let mut a = 0;
        for (b, amount) in sequence.iter() {
            let b = *b;
            //v<A<AA>>^AvAA<^A>Av<<A>>^AvA^Av<A>^Av<<A>^A>AAvA^Av<A<A>>^AAAvA<^A>A
            //<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
            if a == b {
                let len = new_sequence.len();
                new_sequence[len - 1].1 += 1;
                continue;
            }
            let index = a * 5 * 4 + b * 4;
            let mut last = 0;
            //let mut consecutives = 1;
            a = b;
            for i in 0..4 {
                let next = lookup_arrow_pad[index as usize + i];
                if next == last {
                    let len = new_sequence.len();
                    new_sequence[len - 1].1 += 1;
                    continue;
                }
                new_sequence.push((next, 1));
                if next == 0 {
                    //new_sequence.push((0, 1));
                    break;
                }
                last = next;
                //consecutives = 1;
            }
            let len = new_sequence.len();
            new_sequence[len - 1].1 += amount - 1;
        }
        sequence = new_sequence;
    }
    /*for (b, amount) in sequence.iter() {
        for _ in 0..*amount {
            print!("{}", DEBUGSTRING.as_bytes()[*b as usize] as char);
        }
    }*/
    sequence
        .iter()
        .fold(0, |acc, (_, amount)| acc + *amount as u32)
}
pub fn part1(input: &str) -> u32 {
    let lookup_numpad = lookup_move_numpad();
    let lookup_arrowpad = lookup_move_arrow_pad();
    let mut out = 0;
    for l in input.lines() {
        let numval = u32::from_str_radix(&l[0..3], 10).unwrap();
        let len = digit_sequence(l, &lookup_numpad, &lookup_arrowpad);
        out += numval * len;
        //println!("{}", numval * len)
    }
    out
}
pub fn part2(input: &str) -> u32 {
    let lookup_numpad = lookup_move_numpad();
    let lookup_arrowpad = lookup_move_arrow_pad();
    let mut out = 0;
    for l in input.lines() {
        let numval = u32::from_str_radix(&l[0..3], 10).unwrap();
        let len = digit_sequence2(l, &lookup_numpad, &lookup_arrowpad);
        out += numval * len;
        //println!("{}", numval * len)
    }
    out
}
fn digit_sequence(input: &str, lookup_numpad: &[u8], lookup_arrow_pad: &[u8]) -> u32 {
    let bytes = input.as_bytes();
    let mut sequence: Vec<(u8, u8)> = Vec::new();
    let mut a = 0;
    for b in bytes {
        let b = match *b {
            b'A' => 0,
            x if x >= b'0' && x <= b'9' => (x + 1) - b'0',
            _ => {
                unreachable!()
            }
        };
        //print!("a:{} b:{} ", a, b);
        if a == b {
            let len = sequence.len();
            sequence[len - 1].1 += 1;
            continue;
        }
        let index = (a as usize * 11 * 6) + (b as usize * 6);
        let mut last = 0;
        a = b;
        for i in 0..6 {
            let next = lookup_numpad[index + i];
            if next == last {
                let len = sequence.len();
                sequence[len - 1].1 += 1;
                continue;
            }
            sequence.push((next, 1));
            if next == 0 {
                break;
            }
            last = next;
        }
    }
    for _ in 0..25 {
        let mut new_sequence: Vec<(u8, u8)> = Vec::new();
        let mut a = 0;
        for (b, amount) in sequence.iter() {
            let b = *b;
            //v<A<AA>>^AvAA<^A>Av<<A>>^AvA^Av<A>^Av<<A>^A>AAvA^Av<A<A>>^AAAvA<^A>A
            //<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
            if a == b {
                let len = new_sequence.len();
                new_sequence[len - 1].1 += 1;
                continue;
            }
            let index = a * 5 * 4 + b * 4;
            let mut last = 0;
            //let mut consecutives = 1;
            a = b;
            for i in 0..4 {
                let next = lookup_arrow_pad[index as usize + i];
                if next == last {
                    let len = new_sequence.len();
                    new_sequence[len - 1].1 += 1;
                    continue;
                }
                new_sequence.push((next, 1));
                if next == 0 {
                    //new_sequence.push((0, 1));
                    break;
                }
                last = next;
                //consecutives = 1;
            }
            let len = new_sequence.len();
            new_sequence[len - 1].1 += amount - 1;
        }
        sequence = new_sequence;
    }
    /*for (b, amount) in sequence.iter() {
        for _ in 0..*amount {
            print!("{}", DEBUGSTRING.as_bytes()[*b as usize] as char);
        }
    }*/
    //println!();
    sequence
        .iter()
        .fold(0, |acc, (_, amount)| acc + *amount as u32)
} /*


    029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
          <vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A
          v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A
    980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
          v<<A>>^Av<<A>A>^AAvAA<^A>Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A
    179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
          v<<A>>^AAv<<A>A>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A
    456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A

    379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
          v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A


    v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A
  v<<A>>^Av<<A>A>^AAvAA<^A>Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A
  v<<A>>^AAv<<A>A>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A
  v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A
    */
