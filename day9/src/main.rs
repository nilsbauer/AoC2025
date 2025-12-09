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
            let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            if !polygon.contains_square(x1, x2, y1, y2) {
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
}

impl Polygon {
    fn from_point_vec(points: Vec<(u64, u64)>) -> Polygon {
        Polygon { points }
    }

    fn contains_square(&self, x1: u64, x2: u64, y1: u64, y2: u64) -> bool {
        if x1 > x2 || y1 > y2 {
            panic!("square is not ordered!");
        }

        for (idx, (x, y)) in self.points.iter().enumerate() {
            let (x, y) = (*x, *y);
            if x1 < x && x < x2 && y1 < y && y < y2 {
                return false;
            }
            if y1 < y && y < y2 {
                if x == x1 {
                    let idx_before = if idx > 0 { idx-1 } else { self.points.len() - 1 };
                    let idx_after = if idx < self.points.len() - 1 { idx+1 } else { 0 };
                    let (x_before, _) = self.points[idx_before];
                    let (x_after, _) = self.points[idx_after];
                    if x_before > x || x_after > x {
                        return false;
                    }
                }
                if x == x2 {
                    let idx_before = if idx > 0 { idx-1 } else { self.points.len() - 1 };
                    let idx_after = if idx < self.points.len() - 1 { idx+1 } else { 0 };
                    let (x_before, _) = self.points[idx_before];
                    let (x_after, _) = self.points[idx_after];
                    if x_before < x || x_after < x {
                        return false;
                    }
                }
            }
            if x1 < x && x < x2 {
                if y == y1 {
                    let idx_before = if idx > 0 { idx-1 } else { self.points.len() - 1 };
                    let idx_after = if idx < self.points.len() - 1 { idx+1 } else { 0 };
                    let (_, y_before) = self.points[idx_before];
                    let (_, y_after) = self.points[idx_after];
                    if y_before > y || y_after > y {
                        return false;
                    }
                }
                if y == y2 {
                    let idx_before = if idx > 0 { idx-1 } else { self.points.len() - 1 };
                    let idx_after = if idx < self.points.len() - 1 { idx+1 } else { 0 };
                    let (_, y_before) = self.points[idx_before];
                    let (_, y_after) = self.points[idx_after];
                    if y_before < y || y_after < y {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_square() {
        let polygon = Polygon::from_point_vec(vec![(7, 1), (11, 1), (11, 7), (9, 7), (9, 5), (2, 5), (2, 3), (7, 3)]);
        assert!(polygon.contains_square(2, 9, 3, 5));
    }
}
