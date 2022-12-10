
#[derive(Debug)]
enum Cmd {
    ChangeDir,
    BackDir,
    BaseDir,
    Ls,
}
impl Cmd {
    fn from_str(s: &str) -> Cmd {
        let mut parts = s.split_whitespace();
        let cmd = parts.next().unwrap();
        match cmd {
            "cd" => {
                let dir = parts.next().unwrap();
                match dir {
                    ".." => Cmd::BackDir,
                    "/" => Cmd::BaseDir,
                    _ => Cmd::ChangeDir,
                }
            },
            "ls" => Cmd::Ls,
            _ => panic!("Unknown command: {}", cmd),
        }
    }
}

#[derive(Debug)]
struct Dir {
    size: usize,
    childs: Vec<usize>
}

fn count_size(dirs: &Vec<Dir>, dir_idx: usize, total_sizes: &mut Vec<usize>) -> usize {
    let dir = &dirs[dir_idx];
    let mut total_size = dir.size;
    for child in &dir.childs {
        let total_size_ = count_size(dirs, *child, total_sizes);
        total_size += total_size_;
    };
    // dbg!(counting_size, &dir);
    total_sizes[dir_idx] = total_size;
    total_size
}

fn get_total_size(input: String) -> usize {
    let mut lines = input.lines();
    let mut dirs = vec![Dir { size: 0, childs: Vec::new() }];
    let mut current_dir_idx: usize = 0;
    let mut path = Vec::new();

    loop {
        let line_opt = lines.next();
        if let None = line_opt {
            break;
        }
        let line = line_opt.unwrap();
        if line.starts_with("$ ") {
            let cmd = Cmd::from_str(&line[2..]);
            match cmd {
                Cmd::BaseDir => {
                    current_dir_idx = 0;
                    path = Vec::new();
                },
                Cmd::BackDir => {
                    current_dir_idx = path[path.len()-1]; path.pop();
                },
                Cmd::ChangeDir => {
                    path.push(current_dir_idx);
                    let mut newdir = Dir { size: 0, childs: Vec::new() };
                    let idx  = dirs.len();
                    dirs[current_dir_idx].childs.push(idx);
                    current_dir_idx = idx;
                    dirs.push(newdir);
                }
                Cmd::Ls => (),
            }
        } else {
            let part = line.split_whitespace().next().unwrap();
            if part == "dir" {continue};
            let size = part.parse::<usize>().unwrap();
            dirs[current_dir_idx].size += size;
        }
    };

    let mut total_sizes = vec![0; dirs.len()];

    // dbg!(&dirs);
    let total_size = count_size(&dirs, 0, &mut total_sizes);
    // dbg!(&total_sizes);
    let unused_space = 70_000_000 - total_size;
    let must_be_deleted = 30_000_000 - unused_space;
    println!("Must be deleted: {}", must_be_deleted);
    *total_sizes.iter().filter(
        |&&size| size >= must_be_deleted
    ).min().unwrap()
}

pub fn run(input: String) -> usize {
    get_total_size(input)
}
