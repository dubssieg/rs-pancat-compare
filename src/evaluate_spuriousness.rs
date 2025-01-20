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
    let mut seq_predecessors: HashMap<String, Vec<String>> = HashMap::new();
    let mut seq_successors: HashMap<String, Vec<String>> = HashMap::new();

    let mut line: String = String::new();
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'E' {
                // In the case of an E-line, we store the predecessor and successor nodes
                let predecessor: String = String::from(columns[1]);
                let successor: String = String::from(columns[3]);

                add_relation(
                    &mut seq_predecessors,
                    successor.clone(),
                    predecessor.clone(),
                );
                add_relation(&mut seq_successors, predecessor, successor);
            }
            line.clear(); // Clear the line buffer for the next read
        }
    }
    Ok(filter_spurious(seq_predecessors, seq_successors))
}

fn filter_spurious(
    seq_predecessors: HashMap<String, Vec<String>>,
    seq_successors: HashMap<String, Vec<String>>,
) -> Vec<String> {
    // We then search for predecessors nodes that have only one successor and where the sole successor is the predecessor
    let mut spurious_nodes: Vec<String> = Vec::new();
    for (node, successors) in seq_successors.iter() {
        if successors.len() == 1 {
            let succ: &String = &successors[0];
            if seq_predecessors.contains_key(succ) {
                if seq_predecessors.get(succ).unwrap().len() == 1
                    && seq_predecessors.get(succ).unwrap()[0] == *node
                {
                    spurious_nodes.push(succ.clone());
                }
            }
        }
    }
    spurious_nodes
}

fn add_relation(links: &mut HashMap<String, Vec<String>>, value: String, key: String) {
    if links.contains_key(&key) {
        if links.get_mut(&key).unwrap().contains(&value) {
            return;
        } else {
            links.get_mut(&key).unwrap().push(value.clone());
        }
    } else {
        links.insert(key.clone(), vec![value.clone()]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_single() {
        let mut predecessors: HashMap<String, Vec<String>> = HashMap::new();
        let mut successors: HashMap<String, Vec<String>> = HashMap::new();
        predecessors.insert("1".to_string(), vec!["2".to_string()]);
        successors.insert("2".to_string(), vec!["1".to_string()]);
        let spurious_nodes: Vec<String> = filter_spurious(predecessors, successors);
        assert_eq!(spurious_nodes, vec!["1".to_string()]);
    }

    #[test]
    fn test_filter_none() {
        let predecessors: HashMap<String, Vec<String>> = HashMap::new();
        let successors: HashMap<String, Vec<String>> = HashMap::new();
        let spurious_nodes: Vec<String> = filter_spurious(predecessors, successors);
        assert_eq!(spurious_nodes, Vec::<String>::new());
    }

    #[test]
    fn test_filter_empty_succ() {
        let mut predecessors: HashMap<String, Vec<String>> = HashMap::new();
        let mut successors: HashMap<String, Vec<String>> = HashMap::new();
        predecessors.insert("1".to_string(), vec!["2".to_string()]);
        predecessors.insert("3".to_string(), vec!["2".to_string()]);
        successors.insert("2".to_string(), vec!["1".to_string(), "3".to_string()]);
        let spurious_nodes: Vec<String> = filter_spurious(predecessors, successors);
        assert_eq!(spurious_nodes, Vec::<String>::new());
    }

    #[test]
    fn test_filter_empty_preds() {
        let mut predecessors: HashMap<String, Vec<String>> = HashMap::new();
        let mut successors: HashMap<String, Vec<String>> = HashMap::new();
        predecessors.insert("1".to_string(), vec!["2".to_string(), "3".to_string()]);
        successors.insert("2".to_string(), vec!["1".to_string()]);
        successors.insert("3".to_string(), vec!["1".to_string()]);
        let spurious_nodes: Vec<String> = filter_spurious(predecessors, successors);
        assert_eq!(spurious_nodes, Vec::<String>::new());
    }

    #[test]
    fn test_filter_preds() {
        let mut predecessors: HashMap<String, Vec<String>> = HashMap::new();
        let mut successors: HashMap<String, Vec<String>> = HashMap::new();
        predecessors.insert("1".to_string(), vec!["2".to_string()]);
        predecessors.insert("2".to_string(), vec!["3".to_string(), "4".to_string()]);
        successors.insert("2".to_string(), vec!["1".to_string()]);
        successors.insert("3".to_string(), vec!["2".to_string()]);
        successors.insert("4".to_string(), vec!["2".to_string()]);
        let spurious_nodes: Vec<String> = filter_spurious(predecessors, successors);
        assert_eq!(spurious_nodes, vec!["1".to_string()]);
    }

    #[test]
    fn test_filter_succs() {
        let mut predecessors: HashMap<String, Vec<String>> = HashMap::new();
        let mut successors: HashMap<String, Vec<String>> = HashMap::new();
        predecessors.insert("1".to_string(), vec!["3".to_string()]);
        predecessors.insert("2".to_string(), vec!["3".to_string()]);
        predecessors.insert("3".to_string(), vec!["4".to_string()]);
        successors.insert("4".to_string(), vec!["3".to_string()]);
        successors.insert("3".to_string(), vec!["1".to_string(), "2".to_string()]);
        let spurious_nodes: Vec<String> = filter_spurious(predecessors, successors);
        assert_eq!(spurious_nodes, vec!["3".to_string()]);
    }

    #[test]
    fn test_add_relation_forward() {
        let mut linkage: HashMap<String, Vec<String>> = HashMap::new();
        add_relation(&mut linkage, "1".to_string(), "2".to_string());
        assert_eq!(linkage.get("2").unwrap(), &vec!["1".to_string()]);
    }

    #[test]
    fn test_add_relation_reverse() {
        let mut linkage: HashMap<String, Vec<String>> = HashMap::new();
        add_relation(&mut linkage, "2".to_string(), "1".to_string());
        assert_eq!(linkage.get("1").unwrap(), &vec!["2".to_string()]);
    }

    #[test]
    fn test_add_relation_multiple() {
        let mut linkage: HashMap<String, Vec<String>> = HashMap::new();
        add_relation(&mut linkage, "2".to_string(), "1".to_string());
        add_relation(&mut linkage, "3".to_string(), "1".to_string());
        assert_eq!(
            linkage.get("1").unwrap(),
            &vec!["2".to_string(), "3".to_string()]
        );
    }

    #[test]
    fn test_add_and_spurious() {
        let mut predecessors: HashMap<String, Vec<String>> = HashMap::new();
        let mut successors: HashMap<String, Vec<String>> = HashMap::new();
        add_relation(&mut predecessors, "2".to_string(), "1".to_string());
        add_relation(&mut successors, "1".to_string(), "2".to_string());
        let spurious_nodes: Vec<String> = filter_spurious(predecessors, successors);
        assert_eq!(spurious_nodes, vec!["1".to_string()]);
    }
}
