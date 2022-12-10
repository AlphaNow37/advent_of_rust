
fn trad(v: &str) -> &str {
    match v {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        _ => panic!("Unexpected value: {}", v),
    }
}

fn get_vs_list(input: String) -> Vec<(String, String)> {
    let mut vs_list = Vec::new();
    for line in input.lines() {
        let mut vs = line.split(" ");
        let v1 = trad(vs.next().unwrap()).to_string();
        let v2 = vs.next().unwrap().to_string();
        vs_list.push((v1, v2));
    };
    vs_list
}

fn count_points_a(v1: &String, v2: &String) -> u8 {
    (match v2.as_str() {
        "X" => 1, "Y" => 2, "Z" => 3, _ => panic!("Invalid character {}", v2)
    }) + (
        match (v1.as_str(), v2.as_str()) {
            ("X", "X") | ("Y", "Y") | ("Z", "Z") => 3,
            ("X", "Y") | ("Y", "Z") | ("Z", "X") => 6,
            _ => 0
        }
    )
}

const NEXTS: [(&str, &str); 3] = [("X", "Y"), ("Y", "Z"), ("Z", "X")];

fn count_points_b(v1: &String, v2: &String) -> u8 {
    let my_choice = match v2.as_str() {
        "X" => NEXTS.iter().find(|(_, v)| v == v1).unwrap().0,
        "Y" => v1,
        "Z" => NEXTS.iter().find(|(v, _)| v == v1).unwrap().1,
        _ => panic!("Invalid character {}", v2)
    };
    count_points_a(v1, &my_choice.to_string())
}

pub fn run(input: String) {
    let vs_list = get_vs_list(input);
    let sum: usize = vs_list.iter()
        .map(|(v1, v2)| count_points_b(v1, v2) as usize)
        .sum();
    println!("Sum: {}", sum);
}
