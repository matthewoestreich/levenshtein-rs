mod levenshtein;

use levenshtein::*;

fn main() {
    let s1 = "add".to_string();
    let s2 = "dady".to_string();
    levenshtein(s1, s2);
}
