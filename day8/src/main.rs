use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut boxes = BoxList::new();

    for line in input.lines() {
        let jb : JunctionBox = line.parse().unwrap();
        boxes.add(jb);
    }

    let mut last_connection = None;
    while boxes.list[0].circuit == None || boxes.list.iter().any(|b| b.circuit != boxes.list[0].circuit) {
        last_connection = Some(boxes.connect_closest());
    }

    if let Some((idx1, idx2)) = last_connection {
        println!("last boxes:");
        println!("{:?}", boxes.list[idx1]);
        println!("{:?}", boxes.list[idx2]);
        let res = boxes.list[idx1].x * boxes.list[idx2].x;
        println!("{res}");
    }
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

    fn connect_closest(&mut self) -> (usize, usize) {
        let ((idx1, idx2), _dist) = self.distances.iter().min_by_key(|(_, dist)| **dist).expect("no closest boxes found");
        let idx1 = idx1.clone();
        let idx2 = idx2.clone();
        self.distances.remove(&(idx1, idx2));
        if self.list[idx1].circuit.is_some_and(|c1| self.list[idx2].circuit.is_some_and(|c2| c1 == c2)) {
            return (idx1, idx2);
        }

        let new_circuit_num = self.list[idx1].circuit.unwrap_or_else(|| {
            self.next_circuit += 1;
            self.next_circuit
        });
        self.list[idx1].circuit = Some(new_circuit_num);
        if let Some(connecting_circuit) = self.list[idx2].circuit {
            for bx in self.list.iter_mut() {
                if bx.circuit == Some(connecting_circuit) {
                    bx.circuit = Some(new_circuit_num);
                }
            }
        } else {
            self.list[idx2].circuit = Some(new_circuit_num);
        }

        (idx1, idx2)
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
