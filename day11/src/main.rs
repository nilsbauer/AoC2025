use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;


fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let (node, connections) = line.split_once(':').unwrap();
        let connections: Vec<_> = connections.split_ascii_whitespace().map(|s| s.to_string()).collect();
        nodes.insert(node.to_string(), Node::new(connections));
    }
    let ret = find_out(&mut nodes, "svr", false, false);
    println!("{ret}");
}

fn find_out(nodes: &HashMap<String, Node>, current_node: &str, visited_dac: bool, visited_fft: bool) -> u32 {
    let mut visited_dac = visited_dac;
    let mut visited_fft = visited_fft;

    if current_node == "out" {
        if visited_dac && visited_fft {
            return 1;
            println!("found out");
        }
        println!("found out invalid");
        return 0;
    }
    if let Some(ret) = *nodes[current_node].value.borrow() {
        println!("cache hit in {current_node}: {ret}");
        return ret;
    }
    if current_node == "dac" {
        visited_dac = true;
    }
    if current_node == "fft" {
        visited_fft = true;
    }
    let mut ret = 0;
    for connection in &nodes[current_node].connections {
        ret += find_out(nodes, &connection, visited_dac, visited_fft);
    }
    if ret > 0 {
        *nodes[current_node].value.borrow_mut() = Some(ret);
        println!("cached for {current_node}: {ret}");
    }
    ret
}

struct Node {
    connections: Vec<String>,
    value: RefCell<Option<u32>>,
}

impl Node {
    fn new(connections: Vec<String>) -> Self {
        Node { connections, value: RefCell::new(None) }
    }
}
