pub fn distance(path_name: &String, path_a: Vec<usize>, path_b: Vec<usize>) {
    /*
    Takes two paths and compares them to find the merge, split and equivalence points.
     */
    if path_a.last() != path_b.last() {
        eprintln!("Both paths must have the same length.");
        std::process::exit(1);
    }
    let mut counter_a = 0;
    let mut counter_b = 0;

    while counter_a < path_a.len() && counter_b < path_b.len() {
        if path_a[counter_a] == path_b[counter_b] {
            counter_a += 1;
            counter_b += 1;
        } else if path_a[counter_a] < path_b[counter_b] {
            println!("{}\tM\t{}", path_name, path_a[counter_a]);
            counter_a += 1;
        } else {
            println!("{}\tS\t{}", path_name, path_b[counter_b]);
            counter_b += 1;
        }
    }
}
