use std::str::FromStr;
use regex::Regex;

type ItemMove = (usize, usize);

#[derive(Debug)]
enum Op {
    ADD, SUB, MUL, DIV
}

impl Op {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Op::ADD => a + b,
            Op::SUB => a - b,
            Op::MUL => a * b,
            Op::DIV => a / b,
        }
    }
}

#[derive(Debug)]
enum OpValue {
    Nb(usize),
    OLD
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    op_value: OpValue,
    div_test: usize,
    target_monkeys: [usize; 2],
    nb_inspected: usize,
}

impl Monkey {
    fn inspect_items(&mut self) -> Vec<ItemMove> {
        let moves = self.items.iter().map(
            |&item| {
                self.nb_inspected += 1;
                let op_value = match self.op_value {
                    OpValue::Nb(nb) => nb,
                    OpValue::OLD => item,
                };
                let mut item = self.op.apply(item, op_value);
                // item /= 3;
                let is_divisible = item % self.div_test == 0;
                (item, self.target_monkeys[is_divisible as usize])
            }
        ).collect();
        self.items = vec![];
        moves
    }
}

fn parse_monkeys(input: String) -> Vec<Monkey> {
    let re = Regex::new(
"Monkey [0-9]:
 +Starting items: (?P<items>([0-9]+, )*([0-9]+))
 +Operation: new = old (?P<op>[+/*-]) (?P<mulby>([0-9]+|(old)))
 +Test: divisible by (?P<div_test>[0-9]+)
 +If true: throw to monkey (?P<targ_true>[0-9])
 +If false: throw to monkey (?P<targ_false>[0-9])"
    ).unwrap();
    re.captures_iter(&input.replace("\r", ""))
        .map(
            |cap| {
                let items = &cap["items"]
                    .split(", ")
                    .map(|s| usize::from_str(s).unwrap())
                    .collect::<Vec<usize>>();
                let op = match &cap["op"] {
                    "+" => Op::ADD,
                    "-" => Op::SUB,
                    "*" => Op::MUL,
                    "/" => Op::DIV,
                    _ => panic!("Unknown op"),
                };
                let mulby = &cap["mulby"];
                let op_value = if mulby == "old" {
                    OpValue::OLD
                } else {
                    OpValue::Nb(usize::from_str(mulby).unwrap())
                };
                let div_test: usize = *(&cap["div_test"].parse().unwrap());
                let targ_true: &usize = &cap["targ_true"].parse().unwrap();
                let targ_false: &usize = &cap["targ_false"].parse().unwrap();

                Monkey {
                    items: items.clone(),
                    op,
                    op_value,
                    div_test,
                    target_monkeys: [*targ_false, *targ_true],
                    nb_inspected: 0,
                }
            }
        ).collect()
}

fn rounds(monkeys: &mut Vec<Monkey>) {
    let test_product = monkeys.iter().map(|m| m.div_test).product::<usize>();
    println!("{}", &test_product);
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let moves = monkeys[i].inspect_items();
            for (item, target) in moves {
                monkeys[target].items.push(item % test_product);
            }
        }
    }
}

pub fn run(input: String) -> usize {
    let mut monkeys = parse_monkeys(input);
    rounds(&mut monkeys);
    let mut nb_inspecteds = monkeys.iter().map(|m| m.nb_inspected).collect::<Vec<usize>>();
    nb_inspecteds.sort();
    let ln = nb_inspecteds.len();
    nb_inspecteds[ln-1] * nb_inspecteds[ln-2]
}
