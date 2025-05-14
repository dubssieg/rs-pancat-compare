use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};

pub fn index_gfa(
    file_path: &str,
    hard_match: bool,
) -> io::Result<(
    HashMap<String, u64>,
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

    let mut seq_lengths: HashMap<String, u64> = HashMap::new();
    let mut path_positions: HashMap<String, u64> = HashMap::new();
    let mut path_types: HashMap<String, char> = HashMap::new();

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // In the case of an S-line, we store the node name and the sequence length
                let node_name = String::from(columns[1]);
                let sequence_length = columns[2].trim().len();
                seq_lengths.insert(node_name, sequence_length as u64);
            }
            if first_char == 'W' {
                path_types.insert(columns[1].to_string(), 'W');
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
            }
            if first_char == 'P' {
                path_types.insert(columns[1].to_string(), 'P');
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
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }
    let path_lengths: HashMap<String, u64> = get_paths_lengths(file_path,path_positions.clone(),path_types.clone(),seq_lengths.clone()).unwrap();

    Ok((seq_lengths, path_positions, path_lengths, path_types))
}

fn get_paths_lengths(file_path: &str,path_positions: HashMap<String, u64>,path_types:HashMap<String, char>,seq_lengths:HashMap<String, u64>) -> Result<HashMap<String, u64>,io::Error> {

    let mut path_lengths: HashMap<String, u64> = HashMap::new();
    let mut file: BufReader<File> = BufReader::new(File::open(file_path)?);
    
    // Buffer to read the two files
    let mut buffer: [u8; 1] = [0; 1];
    
    for (path_name,path_pos) in path_positions.into_iter() {
        file.seek(SeekFrom::Start(path_pos))?;
        let mut path_length:u64 = 0;
        loop {
            let node = if path_types[path_name.as_str()] == 'P' {
                read_next_p_node(&mut file, &mut buffer)
            } else {
                read_next_w_node(&mut file, &mut buffer)
            };
            if node.contains("\t") {
                break;
            }
            let sequence_length = seq_lengths.get(&node).unwrap();
            path_length += sequence_length;
        }
        path_lengths.insert(path_name.clone(), path_length as u64);
    }
    Ok(path_lengths)

}


fn read_next_p_node(file: &mut BufReader<File>, buffer: &mut [u8; 1]) -> String {
    /*
     * Read the next node in the file, until a comma is found
     * file: the file to read
     * buffer: a buffer to read the file
     */
    let mut node = String::new();
    while file.read(buffer).unwrap() > 0 {
        if buffer[0] == b'+' || buffer[0] == b'-' {
            break;
        }
        if buffer[0] != b',' {
            node.push(buffer[0] as char);
        }
    }
    node
}

fn read_next_w_node(file: &mut BufReader<File>, buffer: &mut [u8; 1]) -> String {
    /*
     * Read the next node in the file, until a > or < is found
     * file: the file to read
     * buffer: a buffer to read the file
     */
    let mut node = String::new();
    while file.read(buffer).unwrap() > 0 {
        if buffer[0] == b'>' || buffer[0] == b'<' || buffer[0] == b'\t' {
            break;
        } else {
            node.push(buffer[0] as char);
        }
    }
    node
}
