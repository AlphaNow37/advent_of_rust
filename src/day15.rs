use regex::Regex;

#[derive(Debug)]
struct Sensor {
    x: isize,
    diameter: isize,
    radius: isize,
}

const WANTED_Y: isize = 2_000_000;

fn parse_input(input: String) -> Vec<Sensor> {
    Regex::new("Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap()
        .captures_iter(&input)
        .filter_map(|group|{
            let (x, y, bx, by) = if let [x, y, bx, by] = group
                .iter()
                .skip(1)
                .map(|g| g.unwrap().as_str().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()[..] { (x, y, bx, by) } else { unreachable!() };
            let radius = ((bx - x).abs() + (by - y).abs()) - (y-WANTED_Y).abs();
            let diameter = radius * 2 + 1;
            if diameter <= 0 { return None;};
            Some(Sensor { x, diameter, radius })
        })
        .collect()
}

fn sort_sensors(sensors: &mut Vec<Sensor>) {
    sensors.sort_by_key(|s| s.x - s.radius as isize);
}

fn find_nb_collides(sensors: &Vec<Sensor>) -> usize {
    let mut x = sensors[0].x - sensors[0].radius + 1;
    let mut nb_collides = 0;
    for sensor in sensors {
        if sensor.x + sensor.radius > x {
            let left = sensor.x - sensor.radius;
            if left > x {
                nb_collides += sensor.diameter as usize;
            } else {
                let to_add = sensor.diameter + left - x;
                nb_collides += to_add as usize;
            }
            x = sensor.x + sensor.radius + 1;
        }
    };
    nb_collides
}


pub fn run(input: String) -> usize {
    let mut sensors = parse_input(input);
    sort_sensors(&mut sensors);
    dbg!(&sensors);
    find_nb_collides(&sensors)
}
