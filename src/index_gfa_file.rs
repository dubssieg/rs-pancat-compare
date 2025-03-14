use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek};

pub fn index_gfa(
    file_path: &str,
    hard_match: bool,
) -> io::Result<(
    HashMap<String, usize>,
    HashMap<String, u64>,
    HashMap<String, u64>,
    HashMap<String, char>,
)> {
    /*
    Given a file path, this function reads the GFA file and returns two HashMaps:
    - seq_lengths: a HashMap with the node names as keys and the sequence lengths as values
    - path_positions: a HashMap with the path names as keys and the offset of the path description as values
    - path_lengths: a HashMap with the path names as keys and the number of nodes in the path as values
    */
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut seq_lengths = HashMap::new();
    let mut path_positions = HashMap::new();
    let mut path_lengths = HashMap::new();
    let mut path_types = HashMap::new();

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // In the case of an S-line, we store the node name and the sequence length
                let node_name = String::from(columns[1]);
                let sequence_length = columns[2].trim().len();
                seq_lengths.insert(node_name, sequence_length);
            }
            if first_char == 'W' {
                path_types.insert(columns[1].to_string(), 'W');
                let mut path_length = 0;
                // In the case of a W-line, we store the path name and the offset of the path description
                // When processing paths, we can match paths in the path_positions HashMap
                // Then start reading the file from there and go with a buffer to read node by node the path
                let path_name = if hard_match {
                    columns[1].to_string() + "#" + columns[2] + "#" + columns[3]
                } else {
                    // Capitalize path name and remove trailing '#0'
                    (columns[1].to_string() + "#" + columns[2] + "#" + columns[3])
                        .to_ascii_uppercase()
                        .trim_end_matches("#0")
                        .to_string()
                };
                let offset = reader.seek(io::SeekFrom::Current(0))?
                    - (line.len() as u64
                        - columns[0].len() as u64
                        - columns[1].len() as u64
                        - columns[2].len() as u64
                        - columns[3].len() as u64
                        - columns[4].len() as u64
                        - columns[5].len() as u64
                        - 6);
                path_positions.insert(path_name.clone(), offset + 1);

                let node_list: Vec<String> = columns[6][1..]
                    .trim()
                    .split(&['<', '>'][..]) // Split by '<' and '>'
                    .map(|s| s[..s.len()].to_string())
                    .collect();
                for node in node_list {
                    let sequence_length = seq_lengths.get(&node).unwrap();
                    path_length += sequence_length;
                }

                path_lengths.insert(path_name.clone(), path_length as u64);
            }
            if first_char == 'P' {
                path_types.insert(columns[1].to_string(), 'P');
                let mut path_length = 0;
                // In the case of a P-line, we store the path name and the offset of the path description
                // When processing paths, we can match paths in the path_positions HashMap
                // Then start reading the file from there and go with a buffer to read node by node the path
                let path_name = if hard_match {
                    String::from(columns[1])
                } else {
                    // Capitalize path name and remove trailing '#0'
                    String::from(columns[1])
                        .to_ascii_uppercase()
                        .trim_end_matches("#0")
                        .to_string()
                };
                let offset = reader.seek(io::SeekFrom::Current(0))?
                    - (line.len() as u64 - columns[0].len() as u64 - columns[1].len() as u64 - 2);
                path_positions.insert(path_name.clone(), offset);

                let node_list: Vec<String> = columns[2]
                    .trim()
                    .split(',')
                    .map(|s| s[..s.len() - 1].to_string())
                    .collect();
                for node in node_list {
                    let sequence_length = seq_lengths.get(&node).unwrap();
                    path_length += sequence_length;
                }

                path_lengths.insert(path_name.clone(), path_length as u64);
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }

    Ok((seq_lengths, path_positions, path_lengths, path_types))
}
