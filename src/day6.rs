use std::ops::Index;

fn get_signal_start(line: &str) -> usize {
    let mut dont_take = 3;
    let mut lasts: Vec<char> = Vec::from(['_'; 13]);
    for (x, char) in line.chars().enumerate() {
        match lasts.iter().enumerate().rfind(|(_, c)| **c == char) {
            None => if dont_take == 0 {
                return x + 1
            },
            Some((i, _)) => if i >= dont_take {dont_take = i+1},
        }
        dont_take -= 1;
        lasts.remove(0);
        lasts.push(char);
    };
    panic!("... !")
}

pub fn run(input: String) -> Vec<usize> {
    input.lines().map(get_signal_start).collect()
}
