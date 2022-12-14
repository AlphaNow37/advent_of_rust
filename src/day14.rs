use std::collections::HashSet;

type Coord = i16;
type Pos = (Coord, Coord);
type BlockSet = HashSet<Pos>;

fn trace_line(from: Pos, to: Pos, set: &mut BlockSet) {
    let dist_x = to.0 - from.0;
    let dist_y = to.1 - from.1;
    let steps = dist_x.abs().max(dist_y.abs());
    let step_x = dist_x.signum();
    let step_y = dist_y.signum();
    for i in 0..=steps {
        let x = from.0 + i * step_x;
        let y = from.1 + i * step_y;
        set.insert((x, y));
    }
}

fn parse_pos(string: &str) -> Pos {
    let mut parts = string.split(',');
    let x = parts.next().unwrap().parse::<Coord>().unwrap();
    let y = parts.next().unwrap().parse::<Coord>().unwrap();
    (x, y)
}

fn get_initial_blockset(input: String) -> BlockSet {
    let mut set = BlockSet::new();
    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let mut from = parse_pos(parts.next().unwrap());
        for part in parts {
            let to = parse_pos(part);
            trace_line(from, to, &mut set);
            from = to;
        }
    }
    set
}

fn get_max_y(set: &BlockSet) -> Coord {
    set.iter().map(|(_, y)| *y).max().unwrap() + 2
}

fn simulate_one_fall(set: &BlockSet, maxy: Coord) -> Pos {
    let mut x = 500;
    let mut y = 0;
    while y < maxy {
        if set.contains(&(x, y+1)) {
            if !set.contains(&(x-1, y+1)) {
                x -= 1
            } else if !set.contains(&(x+1, y+1)) {
                x += 1
            } else {
                return (x, y);
            }
        };
        y += 1;
    };
    (x, y-1)
}

fn simulation(mut blockset: BlockSet, maxy: Coord) -> usize {
    let mut rest_number = 0;

    loop {
        let (x, y) = simulate_one_fall(&blockset, maxy);
        rest_number += 1;
        if y == 0 {
            return rest_number;
        }
        blockset.insert((x, y));
    }
}


pub fn run(input: String) -> usize {
    let set = get_initial_blockset(input);
    let maxy = get_max_y(&set);
    simulation(set, maxy)
}
