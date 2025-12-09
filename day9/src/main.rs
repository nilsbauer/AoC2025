use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut points = Vec::new();
    let mut biggest_area = 0;

    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let x : u64 = x.parse().unwrap();
        let y : u64 = y.parse().unwrap();
        points.push((x, y));
    }
    let polygon = Polygon::from_point_vec(points);
    for (idx1, (x1, y1)) in polygon.points.iter().enumerate() {
        for (x2, y2) in polygon.points.iter().skip(idx1+1) {
            let (x1, x2, y1, y2) = (*x1, *x2, *y1, *y2);
            if !polygon.contains_rectangle(x1, x2, y1, y2) {
                continue;
            }
            let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
            if area > biggest_area {
                println!("found new biggest area between points ({x1}|{y1}) and ({x2}|{y2}) -> {area}");
                biggest_area = area;
            }
        }
    }
    println!("{biggest_area}");
}

struct Polygon {
    points: Vec<(u64, u64)>,
    sides: Vec<LineSegment>
}

impl Polygon {
    fn from_point_vec(points: Vec<(u64, u64)>) -> Polygon {
        let mut sides = Vec::new();
        let mut last_point = None;
        for (x, y) in &points {
            if let Some((last_x, last_y)) = last_point {
                sides.push(LineSegment::new(*x, last_x, *y, last_y));
            }
            last_point = Some((*x, *y));
        }
        let (x1, y1) = points[0];
        let (x2, y2) = points[points.len()-1];
        sides.push(LineSegment::new(x1, x2, y1, y2));

        Polygon { points, sides }
    }

    fn contains_rectangle(&self, x1: u64, x2: u64, y1: u64, y2: u64) -> bool {
        self.contains_point(x1, y2) &&
        self.contains_point(x2, y1) &&
        !self.sides.iter().any(|s| s.intersects_rectangle(x1, x2, y1, y2))
    }

    fn contains_point(&self, x: u64, y: u64) -> bool {
        let mut intersections = 0;
        for side in &self.sides {
            if side.direction == Direction::Horizontal {
                if side.y1 < y {
                    if side.x1 <= x && side.x2 > x {
                        intersections += 1;
                    }
                } else if side.y1 == y && side.x1 <= x && side.x2 >= x {
                    // point is on the vertex
                    return true;
                }
            }
        }
        intersections % 2 == 1
    }
}

#[derive(Debug)]
struct LineSegment {
    x1: u64,
    x2: u64,
    y1: u64,
    y2: u64,
    direction: Direction,
}

impl LineSegment {
    fn new(x1: u64, x2: u64, y1: u64, y2: u64) -> Self {
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        let direction = if x1 == x2 {
            Direction::Vertical
        } else {
            Direction::Horizontal
        };

        LineSegment { x1, x2, y1, y2, direction }
    }

    fn intersects_rectangle(&self, x1: u64, x2: u64, y1: u64, y2: u64) -> bool {
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        match self.direction {
            Direction::Vertical => {
                if self.x1 <= x1 || self.x1 >= x2 {
                    false
                } else {
                    !(self.y1 <= y1 && self.y2 <= y1 || self.y1 >= y2 && self.y2 >= y2)
                }
            },
            Direction::Horizontal => {
                if self.y1 <= y1 || self.y1 >= y2 {
                    false
                } else {
                    !(self.x1 <= x1 && self.x2 <= x1 || self.x1 >= x2 && self.x2 >= x2)
                }
            }
        }
    }
}

#[derive(Debug,PartialEq,Eq)]
enum Direction {
    Horizontal,
    Vertical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_square() {
        let polygon = Polygon::from_point_vec(vec![(7, 1), (11, 1), (11, 7), (9, 7), (9, 5), (2, 5), (2, 3), (7, 3)]);
        assert!(polygon.contains_rectangle(2, 9, 3, 5));
    }

    #[test]
    fn test_intersection() {
        let line = LineSegment::new(1, 3, 2, 2);
        assert!(line.intersects_rectangle(1, 3, 1, 3));
    }
}
