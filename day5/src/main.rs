use std::fs;
use std::ops::RangeInclusive;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let mut ranges = Vec::new();
    let mut res = 0;

    for line in input.lines() {
        if let Some((start, end)) = line.split_once('-') {
            let start : u64 = start.parse().unwrap();
            let end : u64 = end.parse().unwrap();
            ranges = add_range(ranges, start ..= end);
        } else {
            break;
        }
    }
    for range in ranges {
        res += *range.end() - *range.start() + 1;
    }
    println!("{res}");
}

fn add_range(mut ranges: Vec<RangeInclusive<u64>>, range: RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
    let mut range_containing_start = None;
    let mut range_containing_end = None;
    let mut remove_ranges = Vec::new();
    let mut insert_idx = None;
    for (idx, existing) in ranges.iter().enumerate() {
        if existing.contains(range.start()) && existing.contains(range.end()) {
            return ranges;
        } else if existing.contains(range.start()) {
            range_containing_start = Some(idx);
        } else if existing.contains(range.end()) {
            range_containing_end = Some(idx);
        } else if range.contains(existing.start()) && range.contains(existing.end()) {
            remove_ranges.push(idx);
        }
        if existing.start() > range.end() {
            insert_idx = Some(idx);
            break;
        }
    }
    if let (Some(start), Some(end)) = (range_containing_start, range_containing_end) {
        let union = *ranges[start].start() ..= *ranges[end].end();
        ranges.remove(end);
        while let Some(overlapped) = remove_ranges.pop() {
            ranges.remove(overlapped);
        }
        ranges.remove(start);
        ranges.insert(start, union);
        return ranges;
    }
    if let Some(start) = range_containing_start {
        let existing = ranges.get_mut(start).unwrap();
        *existing = *existing.start() ..= *range.end();
        while let Some(overlapped) = remove_ranges.pop() {
            ranges.remove(overlapped);
        }
        return ranges;
    }
    if let Some(end) = range_containing_end {
        let existing = ranges.get_mut(end).unwrap();
        *existing = *range.start() ..= *existing.end();
        while let Some(overlapped) = remove_ranges.pop() {
            ranges.remove(overlapped);
        }
        return ranges;
    }
    if let Some(insert_idx) = insert_idx {
        ranges.insert(insert_idx, range);
        while let Some(overlapped) = remove_ranges.pop() {
            ranges.remove(overlapped);
        }
        return ranges;
    }
    ranges.push(range);
    ranges
}
