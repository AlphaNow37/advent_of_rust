fn find_shared(groups: Vec<Vec<char>>) -> char {
    let a = groups[0].clone();
    let others = groups[1..].to_vec();
    for c in &a {
        if others.iter().all(|g| g.contains(c)) {
            return *c;
        }
    }
    panic!("No shared character found\nin  {:?}\nand {:?}\n", a, others);
}

fn get_priority(c: char) -> u8 {
    let ord = c as u8;
    if ord < 96 { ord - 64 + 26 } else { ord - 96 }
}

fn get_line_priority(line: &str) -> u8 {
    let chars = line.chars().collect::<Vec<char>>();
    let half = (chars.len() / 2) as usize;
    let shared_char = find_shared(Vec::from(chars.chunks(half).map(|x| x.to_vec()).collect::<Vec<Vec<char>>>()));
    get_priority(shared_char)
}

fn day3_a(input: String) {
    let prio_sum: usize = input.lines()
        .map(|line| get_line_priority(line) as usize)
        .sum();
    println!("Sum: {}", prio_sum);
}

fn day3_b(input: String) -> usize {
    input.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| {
            let arr = (*lines).to_vec();
            let as_chars = arr.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
            get_priority(find_shared(as_chars)) as usize
        })
        .sum()
}


pub fn run(input: String) -> usize {
    day3_b(input)
}
