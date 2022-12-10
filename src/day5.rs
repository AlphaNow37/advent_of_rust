type Stack = Vec<char>;
type Stacks = Vec<Stack>;

type Ins = [u8; 3];
const SPACE_CHAR: char = ' ';

fn parse_input(input: String) -> (Stacks, Vec<Ins>) {
    let mut blocks = input.split("\r\n\r\n");
    let stacks_str: String = blocks.next().unwrap().to_string();
    let mut groups = stacks_str
        .split("\n")
        .map(|s: &str| {
            let mut chars = s.chars();
            chars.next().unwrap();
            chars.step_by(4)
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    groups.pop();

    let mut stacks = Vec::new();
    for group in groups.iter().rev() {
        if group.len() > stacks.len() {
            for _ in 0..group.len() - stacks.len() {
                stacks.push(Vec::new());
            }
        }
        for (i, c) in group.iter().enumerate() {
            if *c != SPACE_CHAR {
                stacks[i].push(*c);
            }
        }
    }

    let instructions_str = blocks.next().unwrap().to_string();
    let instructions = instructions_str
        .lines()
        .map(|s: &str| {
            let mut worlds = s.split(" ");
            worlds.next();
            worlds
                .step_by(2)
                .map(|s: &str| s.parse::<u8>().unwrap()-1)
                .collect::<Vec<u8>>()
                .try_into().unwrap()
        }).collect::<Vec<Ins>>();
    (stacks, instructions)
}

fn execute_instructions(stacks: &mut Stacks, instructions: &Vec<Ins>) {
    for instruction in instructions {
        let [nb, from, to] = *instruction;
        let length = &stacks[from as usize].len();
        let mut tail: Vec<char> = stacks[from as usize].split_off(length - nb as usize - 1);
        stacks[to as usize].append(&mut tail);
    }
}

pub fn run(input: String) -> String {
    let (mut stacks, instructions) = parse_input(input);
    execute_instructions(&mut stacks, &instructions);
    stacks.iter()
        .map(|s| s[s.len()-1].to_string())
        .collect::<Vec<String>>()
        .join("")
}
