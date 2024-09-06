use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_gfa_file(
    file_path: &str,
) -> Result<(HashMap<String, usize>, HashMap<String, Vec<String>>), io::Error> {
    /*
    Read a GFA file and return two HashMaps:
    - The first HashMap has the node names as keys and the node lengths as values.
    - The second HashMap has the path names as keys and the node lists as values.
     */
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Failed to open file: {}", error);
            return Err(error);
        }
    };
    let reader = io::BufReader::new(file);
    let mut seq_lengths: HashMap<String, usize> = HashMap::new();
    let mut node_lists: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let columns: Vec<&str> = line.split('\t').collect();
                if let Some(first_char) = line.chars().next() {
                    if first_char == 'S' {
                        if columns.len() >= 3 {
                            let node_name = String::from(columns[1]);
                            let sequence_length = columns[2].len();
                            seq_lengths.insert(node_name, sequence_length);
                        }
                    }
                    if first_char == 'P' {
                        // Split at every comma the third field
                        let path_name = String::from(columns[1]);
                        let node_list: Vec<String> = columns[2]
                            .split(',')
                            .map(|s| s[..s.len() - 1].to_string())
                            .collect();
                        node_lists.insert(path_name, node_list);
                    }
                }
            }
            Err(error) => return Err(error),
        }
    }
    Ok((seq_lengths, node_lists))
}
