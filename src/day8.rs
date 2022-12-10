
type Grid = Vec<Vec<usize>>;

fn parse_grid(input: String) -> Grid {
    input.lines().map(
        |line| line.chars().map(
            |c| c.to_digit(10).unwrap() as usize
        ).collect()
    ).collect()
}

// fn set_row_sumeach(grid: &mut Grid, reverse_iter: bool, rev_xy: bool) {
//
//     let i = (reverse_iter as usize) * 2 + (rev_xy as usize);
//
//     let mut width = grid[0].len();
//     let mut height = grid.len();
//
//     if rev_xy {
//         (width, height) = (height, width);
//     };
//     let y_iter = 0..height;
//     for y in y_iter {
//         let x_iter = if reverse_iter {
//             (0..width).rev().collect::<Vec<usize>>()
//         } else {
//             (0..width).collect::<Vec<usize>>()
//         };
//         let mut nb_visible = 0;
//         let mut min_size = 10;  // > each digit
//         for x in &x_iter {
//             let (x, y) = if rev_xy { (y, *x) } else { (*x, y) };
//             let mut cell = grid[y][x];
//             cell.1[i] = nb_visible;
//             if cell.0 < min_size {
//                 nb_visible = 0;
//             }
//             min_size = cell.0;
//             nb_visible += 1;
//         }
//     }
// }
//
// fn set_total_sumeach(grid: &mut Grid) {
//     for rev_xy in [true, false] {
//         for reverse_iter in [true, false] {
//             set_row_sumeach(grid, reverse_iter, rev_xy);
//         }
//     };
// }

fn get_row_vue(grid: &Grid, x: usize, y: usize, it: Vec<(usize, usize)>) -> usize {
    let mut nb_visible = 0;
    let maxi = grid[y][x];
    for (x, y) in it {
        let cell = grid[y][x];
        if cell < maxi {
            nb_visible += 1;
        } else {
            nb_visible += 1;
            break;
        }
    };
    nb_visible
}


fn get_vue(grid: &Grid, x: usize, y: usize) -> usize {
    [
        (0..x).rev().collect::<Vec<usize>>().iter().map(|x| (*x, y)).collect::<Vec<(usize, usize)>>(),
        (x+1..grid[0].len()).collect::<Vec<usize>>().iter().map(|x| (*x, y)).collect::<Vec<(usize, usize)>>(),
        (0..y).rev().collect::<Vec<usize>>().iter().map(|y| (x, *y)).collect::<Vec<(usize, usize)>>(),
        (y+1..grid.len()).collect::<Vec<usize>>().iter().map(|y| (x, *y)).collect::<Vec<(usize, usize)>>(),
    ].map(|it| get_row_vue(grid, x, y, it)).to_vec().iter().product()
}

fn get_max_vue(grid: &Grid) -> usize {
    (0..grid.len()).map(
        |y| (0..grid[0].len()).map(
            |x| get_vue(grid, x, y)
        ).max().unwrap()
    ).max().unwrap()
}

pub fn run(input: String) -> usize {
    let mut grid = parse_grid(input);
    get_max_vue(&grid)
}
