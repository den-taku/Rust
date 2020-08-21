use std::collections::HashMap;

fn main() {
    let mut vote_counts: HashMap<String, usize> = HashMap::new();
    let ballots = vec!["DenTaku".to_string(), "DenTaku".to_string(), "DenTaku".to_string(), "DenTaku".to_string(), "DenTaku".to_string(), "George".to_string()];
    for name in ballots {
        let count = vote_counts.entry(name).or_insert(0);
        *count += 1;
    }
    if let Some(r) = vote_counts.get(&"Dentaku".to_string()) {
        println!("Dentaku got {} votes!", *r);
    }

}
