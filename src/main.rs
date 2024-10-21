mod compute_distance;
mod index_gfa_file;
use std::env;

fn main() {
    /*
    Compare two GFA files and compute the distance between them
    Two arguments must be given in the command line:
    - The path to the first GFA file
    - The path to the second GFA file
    It will print to standard output the differences between the two graphs
    */
    // Get the file path from command line arguments
    let args: Vec<String> = env::args().collect();
    let file_path_a = &args[1];
    let file_path_b = &args[2];

    // Parse first graph
    let (seq_lengths_a, path_descriptors_a, path_lengths_a) =
        match index_gfa_file::index_gfa(file_path_a) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Failed to read GFA file: {}", error);
                return;
            }
        };

    // Parse second graph
    let (seq_lengths_b, path_descriptors_b, path_lengths_b) =
        match index_gfa_file::index_gfa(file_path_b) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Failed to read GFA file: {}", error);
                return;
            }
        };

    // Compute the distance between the two graphs
    compute_distance::distance(
        file_path_a,
        file_path_b,
        seq_lengths_a,
        seq_lengths_b,
        path_descriptors_a,
        path_descriptors_b,
        path_lengths_a,
        path_lengths_b,
    )
    .unwrap();
}
