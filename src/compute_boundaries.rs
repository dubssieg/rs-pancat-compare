use std::collections::HashMap;

pub fn compute_cumulative_sum(
    strings: &Vec<String>,
    hashmap: &HashMap<String, usize>,
) -> Vec<usize> {
    /*
    Compute the cumulative sum of the node lengths in the path
     */
    let mut cumulative_sum = 0;
    let mut result = Vec::new();

    for string in strings {
        if let Some(value) = hashmap.get(string) {
            cumulative_sum += value;
        }
        result.push(cumulative_sum);
    }

    result
}
