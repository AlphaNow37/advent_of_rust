use std::collections::HashSet;

type Action = (isize, isize);
type Position = (isize, isize);

fn parse_input(input: String) -> Vec<Action> {
    input.lines().map(
        |line| {
            let mut parts = line.split(" ");
            let dir = parts.next().unwrap();
            let (coefx, coefy) = match dir {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => panic!("Unknown direction {}", dir)
            };
            let length = parts.next().unwrap().parse::<usize>().unwrap();
            (0..length).map(|_| (coefx, coefy)).collect::<Vec<Action>>()
        }
    )
        .flatten()
        .collect()
}
fn move_point(point: &mut (isize, isize), action: &Action) {
    point.0 += action.0;
    point.1 += action.1;
}
fn apply_constraints(point: &mut Position, follow: &Position) {
    if (follow.0 - point.0).abs() > 1 || (follow.1 - point.1).abs() > 1 {
        point.0 += (follow.0 > point.0) as isize - (follow.0 < point.0) as isize;
        point.1 += (follow.1 > point.1) as isize - (follow.1 < point.1) as isize;
    }
}

const NB_NODES: usize = 9;

fn moves(actions: &Vec<Action>) -> Vec<Position> {
    let mut head = (0, 0);
    let mut nodes = [(0, 0); NB_NODES];

    let mut all_pos = vec![(0, 0)];

    for action in actions {
        move_point(&mut head, action);
        let mut last = head;
        for i in 0..NB_NODES {
            apply_constraints(&mut nodes[i], &last);
            last = nodes[i].clone();
        }
        all_pos.push(nodes[NB_NODES-1].clone());
    };
    all_pos
}

fn dbg_map(set: &HashSet<&Position>) {
    let mut minx = 0;
    let mut maxx = 0;
    let mut miny = 0;
    let mut maxy = 0;
    for (x, y) in set.iter() {
        if *x < minx { minx = *x; }
        if *x > maxx { maxx = *x; }
        if *y < miny { miny = *y; }
        if *y > maxy { maxy = *y; }
    }
    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            if set.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn run(input: String) -> usize {
    let action = parse_input(input);
    let moves = moves(&action);
    println!("Moves: {:?}", moves);
    let set: HashSet<&Position> = HashSet::from_iter(moves.iter());
    println!("set: {:?}", set);
    dbg_map(&set);
    set.len()
}
