mod annotate_edit_lengths;
mod compute_distance;
mod evaluate_spuriousness;
mod index_gfa_file;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = "v0.1.3",
    about = "GFA graph comparison tool",
    long_about = "Compares pangenome graphs by calculating the segmentation distance between two GFA (Graphical Fragment Assembly) files."
)]
struct Cli {
    /// The path to the first GFA file
    file_path_a: String,
    /// The path to the second GFA file
    file_path_b: String,
    /// Checks for spurious breakpoints in graphs
    #[clap(long = "spurious", short = 's', action)]
    spurious: bool,
    /// Enables hard match for path names (case and block sensitive)
    #[clap(long = "hard", short = 'H', action)]
    hard: bool,
    /// Annotate editions with their lengths (on an already computed edit file)
    #[clap(long = "annot", short = 'a', action)]
    annotate: Option<String>,
    /// Filter the paths to be annotated
    #[clap(long = "filter", short = 'f', action)]
    filter: Option<String>,
}

fn main() {
    /*
    Compare two GFA files and compute the distance between them
    Two arguments must be given in the command line:
    - The path to the first GFA file
    - The path to the second GFA file
    It will print to standard output the differences between the two graphs
    */
    // Get the file path from command line arguments
    let args: Cli = Cli::parse();

    // Parse first graph
    let (seq_lengths_a, path_descriptors_a, path_lengths_a, path_types_a) =
        match index_gfa_file::index_gfa(&args.file_path_a, args.hard) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Failed to read GFA file: {}", error);
                return;
            }
        };
    // If path_descriptors_a is empty, the GFA file is not in GFA1.0 format
    if path_descriptors_a.is_empty() {
        eprintln!("Error: No paths found in graph.");
        std::process::exit(1);
    }

    // Parse second graph
    let (seq_lengths_b, path_descriptors_b, path_lengths_b, path_types_b) =
        match index_gfa_file::index_gfa(&args.file_path_b, args.hard) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Failed to read GFA file: {}", error);
                return;
            }
        };

    // If path_descriptors_b is empty, the GFA file is not in GFA1.0 format
    if path_descriptors_b.is_empty() {
        eprintln!("Error: No paths found in graph.");
        std::process::exit(1);
    }

    let spurious_nodes_a: Vec<String>;
    let spurious_nodes_b: Vec<String>;

    if args.spurious {
        // Check for spurious breakpoints in the first graph
        spurious_nodes_a = evaluate_spuriousness::spurious_breakpoints(&args.file_path_a).unwrap();

        // Check for spurious breakpoints in the second graph
        spurious_nodes_b = evaluate_spuriousness::spurious_breakpoints(&args.file_path_b).unwrap();
    } else {
        // If the spurious option is not given, do not check for spurious breakpoints
        // Init empty vectors
        spurious_nodes_a = Vec::new();
        spurious_nodes_b = Vec::new();
    }

    // If the annotate option is given, annotate the editions with their lengths
    if args.annotate.is_some() {
        // Create empty vector
        let intersection: Vec<String>;
        if args.filter.is_some() {
            // Create a vector containing the string as sole element
            intersection = vec![args.filter.as_deref().unwrap().to_string()];
        } else {
            //  Compute path intersection
            intersection = path_descriptors_a
                .keys()
                .filter(|&k| path_descriptors_b.contains_key(k))
                .map(|s| s.to_string())
                .collect();
        }
        annotate_edit_lengths::annotate_editions(
            &args.file_path_a,
            &args.file_path_b,
            args.annotate.as_deref().unwrap(),
            seq_lengths_a,
            seq_lengths_b,
            path_descriptors_a,
            path_descriptors_b,
            path_lengths_a,
            path_lengths_b,
            path_types_a,
            path_types_b,
            intersection,
        )
        .unwrap();
    // Else, compute the distance between the two graphs
    } else {
        compute_distance::distance(
            &args.file_path_a,
            &args.file_path_b,
            seq_lengths_a,
            seq_lengths_b,
            path_descriptors_a,
            path_descriptors_b,
            path_lengths_a,
            path_lengths_b,
            path_types_a,
            path_types_b,
            spurious_nodes_a,
            spurious_nodes_b,
        )
        .unwrap();
    }
}
