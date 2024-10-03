use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek};

pub fn index_gfa(file_path: &str) -> io::Result<(HashMap<String, usize>, HashMap<String, u64>)> {
    /*
    Given a file path, this function reads the GFA file and returns two HashMaps:
    - seq_lengths: a HashMap with the node names as keys and the sequence lengths as values
    - path_positions: a HashMap with the path names as keys and the offset of the path description as values
    */
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut seq_lengths = HashMap::new();
    let mut path_positions = HashMap::new();

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // In the case of an S-line, we store the node name and the sequence length
                let node_name = String::from(columns[1]);
                let sequence_length = columns[2].len();
                seq_lengths.insert(node_name, sequence_length);
            }
            if first_char == 'P' {
                // In the case of a P-line, we store the path name and the offset of the path description
                // When processing paths, we can match paths in the path_positions HashMap
                // Then start reading the file from there and go with a buffer to read node by node the path
                let path_name = String::from(columns[1]);
                let offset = reader.seek(io::SeekFrom::Current(0))?
                    - (line.len() as u64 - columns[0].len() as u64 - columns[1].len() as u64 - 2);
                path_positions.insert(path_name, offset);
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }

    Ok((seq_lengths, path_positions))
}
