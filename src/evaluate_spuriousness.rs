use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
// We want to check if some positions are spurious breakpoints.
// A spurious breakpoint is a position where the node before has a single outgoing edge and the node after has a single incoming edge.
// We can check this by computing the number of preceding and following neigbors of each node.
// Here, we will list nodes ID per path that has spuriousness issues
// We can then iterate in the next step over the path and check if nodes identified as spurious breakpoints are in the path.
// If they are in, we can get the position in the path, which will gives us a series of spurious breakpoints per path

pub fn spurious_breakpoints(file_path: &str) -> io::Result<Vec<String>> {
    /*
    Given a file path, this function reads the GFA file and returns a HashMap:
    - spurious_nodes: a vector of spurious node IDs as values
    */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut seq_successors: HashMap<String, Vec<String>> = HashMap::new();

    let mut line: String = String::new();
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'E' {
                // In the case of an E-line, we store the predecessor and successor nodes
                let successor: String = String::from(columns[3]);

                if seq_successors.contains_key(&successor) {
                    seq_successors
                        .get_mut(&successor)
                        .unwrap()
                        .push(successor.clone());
                } else {
                    seq_successors.insert(successor.clone(), vec![successor.clone()]);
                }
            }
            line.clear(); // Clear the line buffer for the next read
        }
    }
    // We then search for nodes that have only one predecessor and we return them
    let mut spurious_nodes: Vec<String> = Vec::new();
    for (node, successors) in seq_successors.iter() {
        if successors.len() == 1 {
            spurious_nodes.push(node.clone());
        }
    }
    Ok(spurious_nodes)
}
