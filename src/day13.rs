use std::f32::consts::E;
use json;
use json::JsonValue::{self, Number, Array};
use json::number::Number as JsonNumber;

fn parse(line: &str) -> JsonValue {
    json::parse(line).expect(format!("Failed to parse {}", &line).as_str())
}

#[allow(dead_code)]
fn dbg_value(value: &JsonValue, indent: usize) {
    let indent_str = " ".repeat(indent);
    match value {
        Number(n) => println!("{}{}, ", indent_str, n),
        Array(a) => {
            println!("{} Array :", indent_str);
            for v in a {
                dbg_value(v, indent + 2);
            }
        }
        _ => println!("{}Other: {:?}", indent, value),
    }
}


fn get_pairs(input: String) -> Vec<(JsonValue, JsonValue)> {
    input
        .split("\r\n\r\n")
        .map(|line| {
            let mut parts = line.split("\r\n");
            (parse(parts.next().unwrap()), parse(parts.next().unwrap()))
        })
        .collect()
}

fn is_ordered_pair(values: (JsonValue, JsonValue)) -> Result<(), bool> {
    let (a, b);
    match values {
        (Number(a1), Number(b1)) => {
            let a = a1.as_fixed_point_u64(0);
            let b = b1.as_fixed_point_u64(0);
            return if a < b {
                Err(true)
            } else if a == b {
                Ok(())
            } else {
                Err(false)
            }
        }

        (Array(v), n @ Number(_)) => {
            a = v;
            b = vec![n];
        },

        (n @ Number(_), Array(v)) => {
            a = vec![n];
            b = v;
        },

        (Array(a_), Array(b_)) => {
            a = a_;
            b = b_;
        },

        (a, b) => panic!("Unexpected value: {:?}", (a, b)),
    };

    let mut i = 0;
    loop {
        if i == a.len() && i == b.len() {return Ok(());}
        if i == a.len() {return Err(true);}
        if i == b.len() {return Err(false);}
        is_ordered_pair((a[i].clone(), b[i].clone()))?;
        i += 1;
    }
}

// fn get_rightorder_pairs(input: String) -> Vec<bool> {
//     let pairs = get_pairs(input);
//     pairs.iter().map(|pair| {
//         let ordered = is_ordered_pair(pair.clone()).expect_err("Expected ordered pair");
//         // dbg_value(&pair.0, 0);
//         // println!();
//         // dbg_value(&pair.1, 0);
//         // dbg!(ordered);
//         ordered
//     }).collect()
// }

fn get_values(input: String) -> Vec<JsonValue> {
    let mut values = vec![];
    values.push(Array(vec![Array(vec![Number(JsonNumber::from(2))])]));
    values.push(Array(vec![Array(vec![Number(JsonNumber::from(6))])]));
    for (a, b) in get_pairs(input) {
        values.push(a);
        values.push(b);
    }
    values.sort_by(|a, b| {
        let ordered = is_ordered_pair((a.clone(), b.clone())).expect_err("Expected ordered pair");
        if ordered {std::cmp::Ordering::Less} else {std::cmp::Ordering::Greater}
    });
    // dbg!(&values);
    values
}

pub fn run(input: String) -> usize {
    let values = get_values(input);
    let index_of_2 = values.iter().position(|v| v == &Array(vec![Array(vec![Number(JsonNumber::from(2))])])).unwrap()+1;
    let index_of_6 = values.iter().position(|v| v == &Array(vec![Array(vec![Number(JsonNumber::from(6))])])).unwrap()+1;
    index_of_2 * index_of_6
    // get_rightorder_pairs(input)
    //     .iter()
    //     .enumerate()
    //     .filter(|(_, x)| **x)
    //     .map(|(x, _)| x+1)
    //     .sum()
}
