fn parse_interval(interval: &str) -> [usize; 2] {
    let mut parts = interval.split('-');
    parts.map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>().try_into().unwrap()
}

fn extract_intervals(line: &str) -> [[usize; 2]; 2] {
    line.split(",")
        .map(|s| parse_interval(s))
        .collect::<Vec<[usize; 2]>>()
        .try_into().unwrap()
}

fn do_intervals_overloap(i1: [usize; 2], i2: [usize; 2]) -> bool {
    i1[0] <= i2[1] && i1[0] >= i2[0]
}
fn is_overlapping_intervals(line: &str) -> bool {
    let intervals = extract_intervals(line);
    do_intervals_overloap(intervals[0], intervals[1]) || do_intervals_overloap(intervals[1], intervals[0])
}

pub fn run(input: String) -> usize {
    input.lines().filter(|line| is_overlapping_intervals(line)).count()
}
