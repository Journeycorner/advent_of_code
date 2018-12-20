use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Type {
    Coordinate,
    DangerousPoint,
}

#[derive(Debug)]
struct Coordinate {
    coordinate_type: Type,
    closest_point: Option<i32>,
    distance_closest_point: Option<i32>,
    x: i32,
    y: i32,
}

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    // fiel size
    let mut max_x = 0;
    let mut max_y = 0;

    let dangerous_points: HashMap<i32, Coordinate> = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| parse_line(line))
        .map(|coordinate| {
            if coordinate.x > max_x {
                max_x = coordinate.x + 1;
            }
            if coordinate.y > max_y {
                max_y = coordinate.y + 1;
            }
            (coordinate.y * max_y + coordinate.x, coordinate)
        })
        .collect();

    let mut field: Vec<Option<Coordinate>> = Vec::with_capacity(max_x as usize * max_y as usize);
    for y in 0..max_y {
        for x in 0..max_x {
            let position = y * max_y + x;
            if let Some(_) = dangerous_points.get(&position) {
                field.push(None);
            } else {
                field.push(Some(Coordinate {
                    coordinate_type: Type::Coordinate,
                    closest_point: Option::None,
                    distance_closest_point: Option::None,
                    x,
                    y,
                }))
            }
        }
    }

    part_one(&mut field, &dangerous_points);

    let mut result: HashMap<i32, i32> = HashMap::new();
    for position in field {
        if let Some(position) = position {
            if let Some(closest_point) = position.closest_point {
                *result.entry(closest_point).or_insert(1) += 1;
            }
        }
    }

    println!("{:#?}", result);
    Ok(())
}

fn part_one(field: &mut Vec<Option<Coordinate>>, dangerous_points: &HashMap<i32, Coordinate>) {
    for coordinate in field {
        if let Some(coordinate) = coordinate {
            for (key, point) in dangerous_points {
                let distance = manhatten_distance(coordinate, point);
                if let Some(distance_closest_point) = coordinate.distance_closest_point {
                    if distance < distance_closest_point {
                        coordinate.distance_closest_point = Some(distance);
                        coordinate.closest_point = Some(*key);
                    } else if distance == distance_closest_point {
                        coordinate.closest_point = None;
                    }
                } else {
                    coordinate.distance_closest_point = Some(distance);
                    coordinate.closest_point = Some(*key);
                }
            }
        }
    }
}

fn parse_line(line: &str) -> Coordinate {
    let mut iter = line.split(", ");
    Coordinate {
        coordinate_type: Type::DangerousPoint,
        closest_point: Option::None,
        distance_closest_point: Option::None,
        x: iter.next().unwrap().parse().unwrap(),
        y: iter.next().unwrap().parse().unwrap(),
    }
}

fn manhatten_distance(a: &Coordinate, b: &Coordinate) -> i32 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}
