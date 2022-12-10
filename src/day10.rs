
const WIDTH: usize = 40;
const HEIGHT: usize = 6;

struct Program {
    x: isize,
    actual_cycle: usize,
    board: [[char; WIDTH]; HEIGHT],
    line: usize,
    column: usize,
}

impl Program {
    fn new() -> Program {
        Program {
            x: 1,
            actual_cycle: 0,
            board: [['?'; WIDTH]; HEIGHT],
            line: 0,
            column: 0,
        }
    }
    fn next_cycle(&mut self) {
        let chr = if (self.x - (self.column as isize)).abs() <= 1 {'#'} else {'.'};
        self.board[self.line][self.column] = chr;
        self.actual_cycle += 1;
        self.column += 1;
        if self.column >= WIDTH {
            self.column = 0;
            self.line += 1;
        }
    }
    fn run(&mut self, input: String) {
        for line in input.lines() {
            self.next_cycle();
            if line == "noop" {
                continue
            }
            self.next_cycle();
            let to_add: isize = line.split(" ")
                .nth(1)
                .expect("Expected two numbers")
                .parse()
                .expect("can't parse a number");
            self.x += to_add;
        };
    }
    fn draw(&self) {
        let mut to_draw = self.board.map(
            |row| row.map(|c| c.to_string()).join("")
        ).join("\n");
        to_draw.insert(0, '\n');
        println!("{}", to_draw);
    }
}

pub fn run(input: String) {
    let mut prog = Program::new();
    prog.run(input);
    prog.draw();
}
