use std::{char, cmp, fmt::Display, str::from_utf8};
struct Scan {
    simble: u8,
    checked: bool,
    column: u32,
    end: u32,
    area_id: u32,
}
const COLUMS: usize = 140;
const ROWS: usize = 140;
const ROWSIZE: usize = 141;
pub fn part1(input: &str) -> impl Display {
    let mut scans: Vec<Vec<Scan>> = Vec::new();
    let map = input.as_bytes();
    let mut area_id = 0;
    for y in 0..ROWS {
        let mut rowscans = Vec::new();
        let rowstart = y * ROWSIZE;
        let mut char = map[rowstart];
        let mut start = 0;
        for x in 1..COLUMS {
            let current = map[rowstart + x];
            if current != char {
                rowscans.push(Scan {
                    simble: char,
                    column: start,
                    end: x as u32,
                    area_id: area_id,
                    checked: false,
                });
                char = current;
                start = x as u32;
                area_id += 1;
            }
        }
        rowscans.push(Scan {
            simble: char,
            column: start,
            end: COLUMS as u32,
            area_id: area_id,
            checked: false,
        });
        area_id += 1;
        scans.push(rowscans);
    }
    let mut merging = 0;
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut out = 0;
    for r in 0..ROWS {
        for index in 0..scans[r].len() {
            if scans[r][index].area_id < merging {
                continue;
            } else {
                /* println!(
                    "{} merging:{} currrent{}",
                    scans[r][index].simble, merging, scans[r][index].area_id
                ); */
                merging = scans[r][index].area_id;
                let sible = scans[r][index].simble;
                let mut area = 0;
                let mut perimeter = 0;
                let mut total_overlap = 0;
                scans[r][index].checked = true;
                queue.push((r, index));
                while let Some((row, index)) = queue.pop() {
                    let start = scans[row][index].column;
                    let end = scans[row][index].end;
                    let size = end - start;
                    area += size;
                    perimeter += size * 2 + 2;
                    if row > 0 {
                        for (index, scan) in scans[row - 1]
                            .iter_mut()
                            .enumerate()
                            .filter(|(_, s)| s.area_id >= merging && s.simble == sible)
                        {
                            let overlap = cmp::min(scan.end, end)
                                .saturating_sub(cmp::max(start, scan.column));
                            if overlap > 0 {
                                scan.area_id = merging;
                                total_overlap += overlap;
                                if !scan.checked {
                                    scan.checked = true;
                                    queue.push((row - 1, index));
                                }
                            }
                        }
                    }
                    if row < (ROWS - 1) {
                        for (index, scan) in scans[row + 1]
                            .iter_mut()
                            .enumerate()
                            .filter(|(_, s)| s.area_id >= merging && s.simble == sible)
                        {
                            let overlap = cmp::min(scan.end, end)
                                .saturating_sub(cmp::max(start, scan.column));
                            if overlap > 0 {
                                scan.area_id = merging;
                                total_overlap += overlap;
                                if !scan.checked {
                                    scan.checked = true;
                                    queue.push((row + 1, index));
                                }
                            }
                        }
                    }
                }
                merging += 1;
                //println!("{} {} {}", sible, area, (perimeter - (total_overlap)));
                out += area * (perimeter - (total_overlap));
            }
        }
    }
    out
}
pub fn part2(input: &str) -> impl Display {
    let mut scans: Vec<Vec<Scan>> = Vec::new();
    let map = input.as_bytes();
    let mut area_id = 0;
    for y in 0..ROWS {
        let mut rowscans = Vec::new();
        let rowstart = y * ROWSIZE;
        let mut char = map[rowstart];
        let mut start = 0;
        for x in 1..COLUMS {
            let current = map[rowstart + x];
            if current != char {
                rowscans.push(Scan {
                    simble: char,
                    column: start,
                    end: x as u32,
                    area_id: area_id,
                    checked: false,
                });
                char = current;
                start = x as u32;
                area_id += 1;
            }
        }
        rowscans.push(Scan {
            simble: char,
            column: start,
            end: COLUMS as u32,
            area_id: area_id,
            checked: false,
        });
        area_id += 1;
        scans.push(rowscans);
    }
    let mut merging = 0;
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut out = 0;
    for r in 0..ROWS {
        for index in 0..scans[r].len() {
            if scans[r][index].area_id < merging {
                continue;
            } else {
                /* println!(
                    "{} merging:{} currrent{}",
                    scans[r][index].simble, merging, scans[r][index].area_id
                ); */
                merging = scans[r][index].area_id;
                let sible = scans[r][index].simble;
                let mut area = 0;
                let mut corners = 0;
                let mut corner_overrlap = 0;
                scans[r][index].checked = true;
                queue.push((r, index));
                while let Some((row, index)) = queue.pop() {
                    let start = scans[row][index].column;
                    let end = scans[row][index].end;
                    let size = end - start;
                    area += size;
                    corners += 4;
                    if row > 0 {
                        for (index, scan) in scans[row - 1]
                            .iter_mut()
                            .enumerate()
                            .filter(|(_, s)| s.area_id >= merging && s.simble == sible)
                        {
                            let overlap = cmp::min(scan.end, end)
                                .saturating_sub(cmp::max(start, scan.column));
                            if overlap > 0 {
                                scan.area_id = merging;
                                if start == scan.column {
                                    corner_overrlap += 1;
                                }
                                if end == scan.end {
                                    corner_overrlap += 1;
                                }
                                if !scan.checked {
                                    scan.checked = true;
                                    queue.push((row - 1, index));
                                }
                            }
                        }
                    }
                    if row < (ROWS - 1) {
                        for (index, scan) in scans[row + 1]
                            .iter_mut()
                            .enumerate()
                            .filter(|(_, s)| s.area_id >= merging && s.simble == sible)
                        {
                            let overlap = cmp::min(scan.end, end)
                                .saturating_sub(cmp::max(start, scan.column));
                            if overlap > 0 {
                                if start == scan.column {
                                    corner_overrlap += 1;
                                }
                                if end == scan.end {
                                    corner_overrlap += 1;
                                }
                                scan.area_id = merging;
                                if !scan.checked {
                                    scan.checked = true;
                                    queue.push((row + 1, index));
                                }
                            }
                        }
                    }
                }
                merging += 1;
                //println!("{} {} {}", sible, area, (corners - (corner_overrlap)));
                out += area * (corners - (corner_overrlap));
            }
        }
    }
    out
}
