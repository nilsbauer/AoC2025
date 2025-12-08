use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

const CONNECTION_NUM : u32 = 1000;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut boxes = BoxList::new();

    for line in input.lines() {
        let jb : JunctionBox = line.parse().unwrap();
        boxes.add(jb);
    }

    for _i in 0..CONNECTION_NUM {
        boxes.connect_closest();
    }

    let mut connection_map = HashMap::new();
    for bx in boxes.list {
        if let Some(circuit) = bx.circuit {
            let count = connection_map.entry(circuit).or_insert(0);
            *count += 1;
        }
    }
    let mut res = 1;
    for i in 1..=3 {
        if let Some((circuit, connections)) = connection_map.iter().max_by_key(|(_k, v)| **v) {
            let circuit = circuit.clone();
            println!("{i}. circuit: {circuit} with {connections} connections");
            res *= connections;
            connection_map.remove(&circuit);
        }
    }
    println!("{res}");
}

struct BoxList {
    list: Vec<JunctionBox>,
    distances: HashMap<(usize, usize), u128>,
    next_circuit: u32,
}

impl BoxList {
    fn new () -> Self {
        BoxList {
            list: Vec::new(),
            distances: HashMap::new(),
            next_circuit: 0,
        }
    }

    fn add(&mut self, bx: JunctionBox) {
        for (ex_idx, existing) in self.list.iter().enumerate() {
            self.distances.insert((ex_idx, self.list.len()), existing.distance(&bx));
        }
        self.list.push(bx);
    }

    fn connect_closest(&mut self) {
        let ((idx1, idx2), _dist) = self.distances.iter().min_by_key(|(_, dist)| **dist).expect("no closest boxes found");
        let new_circuit_num = self.list[*idx1].circuit.unwrap_or_else(|| {
            self.next_circuit += 1;
            self.next_circuit
        });
        self.list[*idx1].circuit = Some(new_circuit_num);
        if let Some(connecting_circuit) = self.list[*idx2].circuit {
            for bx in self.list.iter_mut() {
                if bx.circuit == Some(connecting_circuit) {
                    bx.circuit = Some(new_circuit_num);
                }
            }
        } else {
            self.list[*idx2].circuit = Some(new_circuit_num);
        }

        self.distances.remove(&(*idx1, *idx2));
    }
}

#[derive(Debug, PartialEq, Eq)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
    circuit: Option<u32>,
}

impl JunctionBox {
    /// calculate the distance² so we don't have to deal with fractions or irrational numbers
    fn distance(&self, other: &JunctionBox) -> u128 {
        let diff_x = i128::from(self.x) - i128::from(other.x);
        let diff_y = i128::from(self.y) - i128::from(other.y);
        let diff_z = i128::from(self.z) - i128::from(other.z);

        u128::try_from(diff_x * diff_x +
            diff_y * diff_y +
            diff_z * diff_z).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseJunctionBoxError {
    CommasNotFound,
    CoordinateParseError(ParseIntError),
}

impl FromStr for JunctionBox {
    type Err = ParseJunctionBoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s.split_once(',').and_then(|(x, yz)| yz.split_once(',').map(|(y, z)| (x, y, z)))
            .ok_or(ParseJunctionBoxError::CommasNotFound)?;
        let x = x.parse().map_err(|e| ParseJunctionBoxError::CoordinateParseError(e))?;
        let y = y.parse().map_err(|e| ParseJunctionBoxError::CoordinateParseError(e))?;
        let z = z.parse().map_err(|e| ParseJunctionBoxError::CoordinateParseError(e))?;

        Ok(JunctionBox { x, y, z, circuit: None })
    }
}
