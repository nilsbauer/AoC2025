use std::fs;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();

    let mut shapes = Vec::new();
    let mut new_shape: Vec<Vec<bool>> = Vec::new();
    let mut regions = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            shapes.push(new_shape);
            new_shape = Vec::new();
        } else if let Some((dimensions, region_shapes)) = line.split_once(':') {
            if let Some((dim_x, dim_y)) = dimensions.split_once('x') {
                let dim_x = dim_x.parse().unwrap();
                let dim_y = dim_y.parse().unwrap();
                let shapes = region_shapes
                    .split_ascii_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect();
                regions.push(Region { dim_x, dim_y, shapes });
            }
        } else {
            let shape_row = line.chars().map(|c| c == '#').collect();
            new_shape.push(shape_row);
        }
    }
    println!("shapes: {shapes:?}");
    println!("regions: {regions:?}");
}

#[derive(Debug)]
struct Region {
    dim_x: u32,
    dim_y: u32,
    shapes: Vec<usize>,
}

#[derive(Debug)]
struct Shape {
    /// list of coordinates for all variants (rotations/flips) of the shape
    variants: Vec<Vec<(usize, usize)>>,
}

impl Shape {
    fn from_vec(shape: Vec<Vec<bool>>) -> Self {
        let mut variants = Vec::new();
        let original = shape.iter()
            .enumerate()
            .map(|(y, row)| row.iter()
                .enumerate()
                .filter(|(x, is)| is)
                .map(|(x, _)| (x, y))
            );
    }
}

/// rotate anti-clockwise (aka the correct way)
fn rotate(coordinates: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let dim_x = coordinates[0].len();
    let dim_y = coordinates.len();

    let ret = Vec::new();
    for (old_x, old_y) in coordinates {
        ret.push((old_y, dim_x - old_x - 1));
    }
    normalize(&ret)
}

// TODO rename to reflect?
fn flip(coordinates: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
}

/// translates the coordinates so that they start at x = 0 and y = 0
fn normalize(coordinates: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let shift_x = coordinates.iter().map(|(x, _y)| x).min().unwrap();
    let shift_y = coordinates.iter().map(|(_x, y)| y).min().unwrap();

    let ret = Vec::new();
    for (x, y) in coordinates {
        ret.push((x - shift_x, y - shift_y));
    }
    ret
}
