use std::collections::HashMap;
use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let (node, connections) = line.split_once(':').unwrap();
        let connections: Vec<_> = connections.split_ascii_whitespace().map(|s| s.to_string()).collect();
        nodes.insert(node.to_string(), connections);
    }
    let ret = find_out(&nodes, "you");
    println!("{ret}");
}

fn find_out(nodes: &HashMap<String, Vec<String>>, current_node: &str) -> u32 {
    if current_node == "out" {
        return 1;
    }
    let mut ret = 0;
    for connection in &nodes[current_node] {
        ret += find_out(nodes, &connection);
    }
    ret
}
