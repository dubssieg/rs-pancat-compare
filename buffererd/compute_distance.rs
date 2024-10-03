use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};

pub fn distance(
    file_path1: &str,
    file_path2: &str,
    node_sizes1: HashMap<String, usize>,
    node_sizes2: HashMap<String, usize>,
    path_positions1: HashMap<String, u64>,
    path_positions2: HashMap<String, u64>,
) -> io::Result<()> {
    /*
    Given two GFA files and their associated node sizes and path positions, this function computes the distance between the two graphs.
    - file_path1: the path to the first GFA file
    - file_path2: the path to the second GFA file
    - node_sizes1: a HashMap with the node names as keys and the node sizes as values for the first GFA file
    - node_sizes2: a HashMap with the node names as keys and the node sizes as values for the second GFA file
    - path_positions1: a HashMap with the path names as keys and the offset of the path description for the first GFA file
    - path_positions2: a HashMap with the path names as keys and the offset of the path description for the second GFA file
    Writes to standard output the operations (merges and splits) needed to transform the first graph into the second graph
    */
    let intersection: Vec<&String> = path_positions1
        .keys()
        .filter(|&k| path_positions2.contains_key(k))
        .collect();

    println!("Intersection of paths: {:?}", intersection);

    println!("# Path name\tPosition\tOperation\tNodeA\tNodeB");
    for path_name in intersection {
        // We get the positions of the path in the two files
        let pos1 = path_positions1[path_name];
        let pos2 = path_positions2[path_name];

        // We open the two files
        let mut file1 = BufReader::new(File::open(file_path1)?);
        let mut file2 = BufReader::new(File::open(file_path2)?);

        // We seek to the position of the path in the two files
        file1.seek(SeekFrom::Start(pos1))?;
        file2.seek(SeekFrom::Start(pos2))?;

        // Buffer to read the two files
        let mut buffer1 = [0; 1];
        let mut buffer2 = [0; 1];

        // Positions in the two paths
        let mut breakpoint_a = 0;
        let mut breakpoint_b = 0;
        let mut position = 0;

        // Node names
        let mut node1 = String::new();
        let mut node2 = String::new();

        let mut end_a = false;
        let mut end_b = false;

        while !end_a || !end_b {
            if breakpoint_a == breakpoint_b {
                // The two positions in the two paths are aligned
                let mut node1 = String::new();
                let mut node2 = String::new();

                // No edition operation is needed
                // We must read the two next nodes in the two files
                while file1.read(&mut buffer1)? > 0 {
                    if buffer1[0] == b',' || buffer1[0] == b'\n' {
                        if buffer1[0] == b'\n' {
                            end_a = true;
                        }
                        break;
                    }
                    node1.push(buffer1[0] as char);
                }
                node1.retain(|c| c != '+' && c != '-');

                while file2.read(&mut buffer2)? > 0 {
                    if buffer2[0] == b',' || buffer1[0] == b'\n' {
                        if buffer2[0] == b'\n' {
                            end_b = true;
                        }
                        break;
                    }
                    node2.push(buffer2[0] as char);
                }
                node2.retain(|c| c != '+' && c != '-');

                // Store their associated sizes in breakpoint_a and breakpoint_b
                breakpoint_a += *node_sizes1.get(&node1).unwrap_or(&0);
                breakpoint_b += *node_sizes2.get(&node2).unwrap_or(&0);

                // We update the position in the two paths
            } else if breakpoint_a < breakpoint_b && !end_a {
                // The node in the first path is missing in the second path
                // It is a merge operation
                println!("{}\t{}\tM\t{}\t{}", path_name, position, node1, node2);

                // The two positions in the two paths are not aligned
                let mut node1 = String::new();

                // We must read the next node in the first file
                while file1.read(&mut buffer1)? > 0 {
                    if buffer1[0] == b',' || buffer1[0] == b'\n' {
                        if buffer1[0] == b'\n' {
                            end_a = true;
                        }
                        break;
                    }
                    node1.push(buffer1[0] as char);
                }
                if !end_a {
                    node1.retain(|c| c != '+' && c != '-');
                    // Store its associated size in breakpoint_a
                    breakpoint_a += *node_sizes1.get(&node1).unwrap_or(&0);
                }
            } else if breakpoint_a > breakpoint_b && !end_b {
                // The node in the second path is missing in the first path
                // It is a split operation
                println!("{}\t{}\tS\t{}\t{}", path_name, position, node1, node2);

                // The two positions in the two paths are not aligned
                let mut node2 = String::new();

                // We must read the next node in the second file
                while file2.read(&mut buffer2)? > 0 {
                    if buffer2[0] == b',' || buffer2[0] == b'\n' {
                        if buffer2[0] == b'\n' {
                            end_b = true;
                        }
                        break;
                    }
                    node2.push(buffer2[0] as char);
                }
                if !end_b {
                    node2.retain(|c| c != '+' && c != '-');
                    // Store its associated size in breakpoint_b
                    breakpoint_b += *node_sizes2.get(&node2).unwrap_or(&0);
                }
            } else {
                break;
            }
            // We update the position in the two paths
            position = min(breakpoint_a, breakpoint_b);
        }
    }

    Ok(())
}
