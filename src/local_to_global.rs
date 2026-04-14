use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn local_to_global(
    graph_a_file: &str,
    graph_b_file: &str,
    distance_file: &str,
) -> io::Result<()> {
    /*
    Given two file paths, this function reads the GFA file and the TSV file
    It writes a new file in stdio with unique breakpoints
    Merges and splits must be handled differently: each category should be reported on a specific graph:
    Splits on A, Merges on B
    */

    // Graph A
    // We init the edges collection
    let file = File::open(graph_a_file)?;
    let mut reader = BufReader::new(file);
    let mut edges_a_collection: HashMap<[String; 2],Vec<String>> = HashMap::new();
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'L' {
                // In the case of an L-line, we store the two node names and init an empty vec
                let node_x_name = String::from(columns[1].to_owned()+columns[2]);
                let node_y_name = String::from(columns[3].to_owned()+columns[4]);
                edges_a_collection.insert([node_x_name,node_y_name], Vec::new());
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }

    // Graph B
    // We init the edges collection
    let file = File::open(graph_b_file)?;
    let mut reader = BufReader::new(file);
    let mut edges_b_collection: HashMap<[String; 2],Vec<String>> = HashMap::new();

    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'L' {
                // In the case of an L-line, we store the two node names and init an empty vec
                let node_x_name = String::from(columns[1].to_owned()+columns[2]);
                let node_y_name = String::from(columns[3].to_owned()+columns[4]);
                edges_b_collection.insert([node_x_name,node_y_name], Vec::new());
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }

    let file = File::open(distance_file)?;
    let mut reader = BufReader::new(file);

    // We fill the according collection with info about breakpoints paths
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if  let Some(first_char) = line.chars().next() {
            if first_char != '#' {
                // We skip comment lines
                let path_name = String::from(columns[0]);
                let edit_type = String::from(columns[2]);
                let node_a = String::from(columns[3]);
                let node_b = String::from(columns[4]);
                if edit_type == "S" {
                    let positive_node_a = node_a.clone() + "+";
                    let negative_node_a = node_a.clone() + "-";
                    // We need to find all edges that uses the node_a as predecessor in edges_a_collection
                    for ([x,y], vec) in edges_a_collection.iter_mut() {
                        if positive_node_a == *x || negative_node_a == *y {
                            vec.push(path_name.clone());
                        }
                    }
                } else if  edit_type == "M" {
                    let positive_node_b = node_b.clone() + "+";
                    let negative_node_b = node_b.clone() + "-";
                    // We need to find all edges that uses the node_b as predecessor in edges_b_collection
                    for ([x,y], vec) in edges_b_collection.iter_mut() {
                        if positive_node_b == *x || negative_node_b == *y {
                            vec.push(path_name.clone());
                        }
                    }
                    
                }
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }

    // We print to stdio individual breakpoints
    println!("# Graph\tx\ty\tPaths");
    for ([x,y], vec) in edges_a_collection.iter() {
        if !vec.is_empty() {
            println!("A\t{}\t{}\t{:?}",x,y,vec);
        }
    }
    for ([x,y], vec) in edges_b_collection.iter() {
        if !vec.is_empty() {
            println!("B\t{}\t{}\t{:?}",x,y,vec);
        }
    }
    Ok(())
    
}