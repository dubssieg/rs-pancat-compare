use std::collections::HashMap;

fn print_seqlengths(hashmap: HashMap<String, usize>) {
    for (name, length) in hashmap {
        println!("Node name: {}, Length: {}", name, length);
    }
}

fn print_nodelists(hashmap: HashMap<String, Vec<String>>) {
    for (name, nodes) in hashmap {
        println!("Path name: {}, Nodes: {:?}", name, nodes);
    }
}
