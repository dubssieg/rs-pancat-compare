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
    path_lengths1: HashMap<String, u64>,
    path_lengths2: HashMap<String, u64>,
    spurious_breakpoints1: Vec<String>,
    spurious_breakpoints2: Vec<String>,
) -> io::Result<()> {
    /*
    Given two GFA files and their associated node sizes and path positions, this function computes the distance between the two graphs.
    - file_path1: the path to the first GFA file
    - file_path2: the path to the second GFA file
    - node_sizes1: a HashMap with the node names as keys and the node sizes as values for the first GFA file
    - node_sizes2: a HashMap with the node names as keys and the node sizes as values for the second GFA file
    - path_positions1: a HashMap with the path names as keys and the offset of the path description for the first GFA file
    - path_positions2: a HashMap with the path names as keys and the offset of the path description for the second GFA file
    - path_lengths1: a HashMap with the path names as keys and the number of nodes in the path for the first GFA file
    - path_lengths2: a HashMap with the path names as keys and the number of nodes in the path for the second GFA file
    Writes to standard output the operations (merges and splits) needed to transform the first graph into the second graph
    */
    let intersection: Vec<&String> = path_positions1
        .keys()
        .filter(|&k| path_positions2.contains_key(k))
        .collect();

    println!("# Intersection of paths: {:?}", intersection);

    // Print the length of every path in the intersection
    for path_name in intersection.iter() {
        println!("## {}\t{}", path_name, path_lengths1[path_name.as_str()]);
    }

    let mut equivalences_count: i32 = 0;
    let mut merges_count: i32 = 0;
    let mut splits_count: i32 = 0;
    let mut spurious_count: i32 = 0;

    // We need to duplicate the spurius vectors to keep the original ones
    let mut sp1: Vec<String> = spurious_breakpoints1.clone();
    let mut sp2: Vec<String> = spurious_breakpoints2.clone();

    println!("# Path name\tPosition\tOperation\tNodeA\tNodeB\tBreakpointA\tBreakpointB");
    for path_name in intersection {
        // We get the positions of the path in the two files
        let pos1: u64 = path_positions1[path_name];
        let pos2: u64 = path_positions2[path_name];

        // We open the two files
        let mut file1: BufReader<File> = BufReader::new(File::open(file_path1)?);
        let mut file2: BufReader<File> = BufReader::new(File::open(file_path2)?);

        // We seek to the position of the path in the two files
        file1.seek(SeekFrom::Start(pos1))?;
        file2.seek(SeekFrom::Start(pos2))?;

        // Buffer to read the two files
        let mut buffer1: [u8; 1] = [0; 1];
        let mut buffer2: [u8; 1] = [0; 1];

        // Node names
        let mut node1: String = String::new();
        let mut node2: String = String::new();

        let mut breakpoint_a: usize = 0;
        let mut breakpoint_b: usize = 0;

        let mut position: usize = 0;

        let max_length1: usize = path_lengths1[path_name.as_str()] as usize;
        let max_length2: usize = path_lengths2[path_name.as_str()] as usize;

        if max_length1 != max_length2 {
            // The two paths have different lengths, we cannot compare them
            println!(
                "# Error: the two paths representing {} have different lengths: {} and {}.",
                path_name, max_length1, max_length2
            );
            continue;
        } else {
            while position < max_length1 {
                if breakpoint_a == breakpoint_b {
                    // The two positions in the two paths are aligned
                    equivalences_count += 1;
                    // No edition operation is needed
                    // We must read the two next nodes in the two files
                    node1 = read_next_node(&mut file1, &mut buffer1);
                    node2 = read_next_node(&mut file2, &mut buffer2);
                    // Store their associated sizes in breakpoint_a and breakpoint_b
                    breakpoint_a += node_sizes1.get(&node1).unwrap().clone();
                    breakpoint_b += node_sizes2.get(&node2).unwrap().clone();
                } else if breakpoint_a < breakpoint_b {
                    // The node in the first path is missing in the second path
                    // The two positions in the two paths are not aligned
                    if sp2.contains(&node2) {
                        // Remove the spurious breakpoint from the vector
                        sp2.retain(|x| x != &node2);
                        spurious_count += 1;
                    } else {
                        // It is a split operation
                        splits_count += 1;
                        println!(
                            "{}\t{}\tS\t{}\t{}\t{}\t{}",
                            path_name, position, node1, node2, breakpoint_a, breakpoint_b
                        );
                    }
                    node1 = read_next_node(&mut file1, &mut buffer1);
                    breakpoint_a += node_sizes1.get(&node1).unwrap().clone();
                } else if breakpoint_a > breakpoint_b {
                    // The node in the second path is missing in the first path
                    // The two positions in the two paths are not aligned
                    if sp1.contains(&node1) {
                        // Remove the spurious breakpoint from the vector
                        sp1.retain(|x| x != &node1);
                        spurious_count += 1;
                    } else {
                        // It is a merge operation
                        merges_count += 1;
                        println!(
                            "{}\t{}\tM\t{}\t{}\t{}\t{}",
                            path_name, position, node1, node2, breakpoint_a, breakpoint_b
                        );
                    }
                    node2 = read_next_node(&mut file2, &mut buffer2);
                    breakpoint_b += node_sizes2.get(&node2).unwrap().clone();
                }

                // We update the position in the two paths
                position = min(breakpoint_a, breakpoint_b);
            }
        }
    }
    println!(
        "# Distance: {} (E={}, S={}, M={}, SP={}).",
        splits_count + merges_count,
        equivalences_count,
        splits_count,
        merges_count,
        spurious_count
    );
    Ok(())
}

fn read_next_node(file: &mut BufReader<File>, buffer: &mut [u8; 1]) -> String {
    /*
     * Read the next node in the file, until a comma or a newline is found
     * Return the node name and a boolean indicating if the end of the line has been reached
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
