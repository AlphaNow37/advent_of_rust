use std::collections::vec_deque::VecDeque;

type HeightMap = Vec<Vec<u8>>;
type Position = (usize, usize);

const MINIMAL_HEIGHT: u8 = 1;
const MINIMAL_CHAR: u8 = 'a' as u8 - MINIMAL_HEIGHT;

fn get_height_map(input: String) -> (HeightMap, Position, Position) {
    let mut start= (0, 0);
    let mut end = (0, 0);
    (input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .map(|(x, mut char)| {
                    if char == 'S' {
                        start = (x, y);
                        char = 'a';
                    } else if char == 'E' {
                        end = (x, y);
                        char = 'z';
                    }
                    char as u8 - MINIMAL_CHAR
                })
                .collect()
        })
        .collect(), start, end)
}

fn path_find(height_map: &HeightMap, _start: Position, end: Position) -> usize {
    /// part 1 algorithm
    let width = height_map[0].len();
    let height = height_map.len();

    let mut queue = VecDeque::from([end]);
    let mut visited = vec![vec![false; width]; height];

    let mut mustbreak = false;
    let mut result = 0;
    while !mustbreak {
        if queue.is_empty() {
            panic!("No path found!");
        }
        result += 1;
        queue = queue
            .iter()
            .flat_map(|(x, y)| {
                let mut values = Vec::new();
                let act_height = height_map[*y][*x];
                for (x_, y_) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                    let new_x = *x as isize + x_;
                    let new_y = *y as isize + y_;
                    if new_x >= 0 && new_y >= 0 && new_x < width as isize && new_y < height as isize {
                        let new_x = new_x as usize;
                        let new_y = new_y as usize;
                        if !visited[new_y][new_x] && height_map[new_y][new_x] >= act_height-1 {
                        // if !visited[new_y][new_x] && height_map[new_y][new_x] <= act_height+1 {
                            visited[new_y][new_x] = true;
                            if height_map[new_y][new_x] == MINIMAL_HEIGHT {
                            // if (new_x, new_y) == end {
                                mustbreak = true;
                            } else {
                                values.push((new_x, new_y));
                            }
                        }
                    }
                }
                values
            }).collect();
    }
    result
}

pub fn run(input: String) -> usize {
    let (height_map, start, end) = get_height_map(input);
    path_find(&height_map, start, end)
}
