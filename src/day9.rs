use std::{cmp, fmt::Display, iter::once};

pub fn part1(input: &str) -> impl Display {
    let bytes = input.as_bytes();
    let parsed = bytes
        .chunks_exact(2)
        .map(|f| (f[0] as u32 - 48, f[1] as u32 - 48))
        .chain(once((bytes[bytes.len() - 1] as u32 - 48, 0u32)))
        .enumerate();
    let mut files = Vec::new();
    let mut out = Vec::new();
    let mut cursor = 0;
    for (id, (f, e)) in parsed {
        files.push((id, cursor, f, e));
        cursor += f + e;
    }
    let mut start_last = u32::MAX;
    let mut file_remainder = 0;
    let mut files_back = files.iter().rev();
    let mut back_id = 0;
    for (id, start, lenght, empty) in files.iter() {
        if *start == start_last {
            out.push((*id, *start, file_remainder));
            break;
        }
        out.push((*id, *start, *lenght));
        let cursor_end = start + lenght + empty;
        let mut cursor = start + lenght;
        while cursor < cursor_end {
            if file_remainder == 0 {
                if let Some((idb, startb, lenghtb, _)) = files_back.next() {
                    if *startb > cursor {
                        file_remainder = *lenghtb;
                        start_last = *startb;
                        back_id = *idb;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            let block = cmp::min(file_remainder, cursor_end - cursor);
            out.push((back_id, cursor, block));
            cursor += block;
            file_remainder -= block;
        }
    }
    /*for x in out.iter() {
        for _ in 0..x.2 {
            print!("{}", x.0);
        }
    }
    println!("  ");*/
    out.iter().fold(0, |acc, (id, start, size)| {
        let start = *start as usize;
        let size = *size as usize;
        acc + (id * ((start + start + size - 1) * size) / 2)
    })
}
pub fn part2(input: &str) -> impl Display {
    let bytes = &input.as_bytes()[0..input.len() - 1];
    let parsed = bytes
        .chunks_exact(2)
        .map(|f| (f[0] as u32 - 48, f[1] as u32 - 48))
        .chain(once((input.as_bytes()[input.len() - 1] as u32 - 48, 0u32)))
        .enumerate();
    let mut files = Vec::new();
    let mut empty = Vec::new();
    let mut out = Vec::new();
    let mut cursor = 0;
    for (id, (f, e)) in parsed {
        files.push((id, cursor, f));
        empty.push((cursor + f, e));
        cursor += f + e;
    }
    for (id, start, size) in files.iter().rev() {
        if let Some((start_e, size_e)) = empty
            .iter_mut()
            .filter(|(s_e, sz_e)| sz_e >= size && s_e <= start)
            .next()
        {
            out.push((*id, *start_e, *size));
            *start_e += size;
            *size_e -= size;
        } else {
            out.push((*id, *start, *size));
        }
    }
    out.iter().fold(0, |acc, (id, start, size)| {
        let start = *start as usize;
        let size = *size as usize;
        acc + (id * ((start + start + size - 1) * size) / 2)
    })
}
