use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};

pub fn annotate_editions(
    file_path1: &str,
    file_path2: &str,
    edition_results_file: &str,
    node_sizes1: HashMap<String, usize>,
    node_sizes2: HashMap<String, usize>,
    path_positions1: HashMap<String, u64>,
    path_positions2: HashMap<String, u64>,
    path_lengths1: HashMap<String, u64>,
    path_lengths2: HashMap<String, u64>,
    path_types1: HashMap<String, char>,
    path_types2: HashMap<String, char>,
    intersection: Vec<String>,
) -> io::Result<()> {
    /*
    Given an edition file and two graphs, annotate the length of the edition operations
    Adds two fields to the edition file: global_length and local_length
    global_length: length of operation without taking other edits into account
    local_length: length of operation taking other edits into account
     */

    // Create empty vector hashmap to store edit positions
    let mut edit_positions: HashMap<String, Vec<u64>> = HashMap::new();
    for path_name in intersection.iter() {
        edit_positions.insert(path_name.to_string(), Vec::new());
    }

    // Create vectors that contains both edit positions + node positions * 2 (one for each graph)
    // Sort the vector and seek the index of the current edit position
    // Look left and right, find the closest position, the difference is the length of the edit
    for path_name in edit_positions.keys().cloned().collect::<Vec<String>>() {
        let pos1: u64 = path_positions1[&path_name];
        let mut file1: BufReader<File> = BufReader::new(File::open(file_path1)?);
        file1.seek(SeekFrom::Start(pos1))?;
        let mut buffer1: [u8; 1] = [0; 1];
        let mut cum_length = 0;
        let max_length1 = path_lengths1[&path_name];
        if let Some(positions) = edit_positions.get_mut(&path_name) {
            positions.push(0);
        }

        while cum_length < max_length1 {
            // We read the next node
            let node_a = if path_types1[&path_name] == 'W' {
                read_next_w_node(&mut file1, &mut buffer1)
            } else {
                read_next_p_node(&mut file1, &mut buffer1)
            };
            cum_length += node_sizes1[&node_a] as u64;
            if let Some(positions) = edit_positions.get_mut(&path_name) {
                positions.push(cum_length);
            }
        }
    }

    for path_name in edit_positions.keys().cloned().collect::<Vec<String>>() {
        let pos2: u64 = path_positions2[&path_name];
        let mut file2: BufReader<File> = BufReader::new(File::open(file_path2)?);
        file2.seek(SeekFrom::Start(pos2))?;
        let mut buffer2: [u8; 1] = [0; 1];
        let mut cum_length = 0;
        let max_length2 = path_lengths2[&path_name];
        if let Some(positions) = edit_positions.get_mut(&path_name) {
            positions.push(0);
        }

        while cum_length < max_length2 {
            // We read the next node
            let node_b = if path_types2[&path_name] == 'W' {
                read_next_w_node(&mut file2, &mut buffer2)
            } else {
                read_next_p_node(&mut file2, &mut buffer2)
            };
            cum_length += node_sizes2[&node_b] as u64;
            if let Some(positions) = edit_positions.get_mut(&path_name) {
                positions.push(cum_length);
            }
        }
    }

    // We sort each and every vector
    for (_, positions) in edit_positions.iter_mut() {
        positions.sort();
    }

    // Now we can iterate on the edit file and compute the length of each edit
    let file = File::open(edition_results_file)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if line.starts_with('#') {
            print!("{}", line);
            line.clear();
            continue;
        }
        let columns: Vec<&str> = line.split('\t').collect();
        let path_name = columns[0].to_string();
        if !intersection.contains(&path_name) {
            line.clear();
            continue;
        }
        let edit: u64 = columns[1].parse::<u64>().unwrap();
        // We search the index of the position in the vector
        let index = edit_positions[&path_name]
            .iter()
            .position(|r| r == &edit)
            .unwrap();
        let length = min(
            edit_positions[&path_name][index + 1] - edit,
            edit - edit_positions[&path_name][index - 1],
        );
        // We get the node where the operation is : 3rd column if operation == S, 4th column if operation == M
        let operation = columns[2];
        let node_length = if operation == "S" {
            node_sizes1[columns[3]]
        } else {
            node_sizes2[columns[4]]
        };

        println!("{}\t{}\t{}", line.trim_end(), length, node_length);
        line.clear();
    }
    Ok(())
}

fn read_next_p_node(file: &mut BufReader<File>, buffer: &mut [u8; 1]) -> String {
    /*
     * Read the next node in the file, until a comma is found
     * file: the file to read
     * buffer: a buffer to read the file
     */
    let mut node = String::new();
    while file.read(buffer).unwrap() > 0 {
        if buffer[0] == b'+' || buffer[0] == b'-' || buffer[0] == b'\t' {
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
