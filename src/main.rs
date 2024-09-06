mod calculate_distance;
mod compute_boundaries;
mod parse_gfa_file;
use std::env;

fn main() {
    // Get the file path from command line arguments
    let args: Vec<String> = env::args().collect();
    let file_path_a = &args[1];
    let file_path_b = &args[2];

    // Parse first graph
    let (seq_lengths_a, node_lists_a) = match parse_gfa_file::read_gfa_file(file_path_a) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Failed to read GFA file: {}", error);
            return;
        }
    };

    // Parse second graph
    let (seq_lengths_b, node_lists_b) = match parse_gfa_file::read_gfa_file(file_path_b) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Failed to read GFA file: {}", error);
            return;
        }
    };

    // Compute the intersection of node_list_a and node_list_b keys
    let common_keys: Vec<&String> = node_lists_a
        .keys()
        .filter(|key| node_lists_b.contains_key(*key))
        .collect();
    // Iterate on common_keys
    for key in common_keys {
        let path_a_descriptor = compute_boundaries::compute_cumulative_sum(
            &node_lists_a.get(key).unwrap(),
            &seq_lengths_a,
        );
        let path_b_descriptor = compute_boundaries::compute_cumulative_sum(
            &node_lists_b.get(key).unwrap(),
            &seq_lengths_b,
        );
        calculate_distance::distance(key, path_a_descriptor, path_b_descriptor);
    }
}
