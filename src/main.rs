mod levenshtein;

const SHOULD_PRINT_TRACE: bool = true;

fn main() {
    let s1 = "prefer";
    let s2 = "pretense";
    let r = levenshtein::distance(s1, s2, SHOULD_PRINT_TRACE);
    println!(
        "distance = {}\nactions  = {:?}",
        r.distance, r.actions_taken
    );
}
