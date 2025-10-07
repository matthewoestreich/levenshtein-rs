mod levenshtein;

use levenshtein::*;

fn main() {
    let s1 = "add";
    let s2 = "dady";
    levenshtein(s1.to_string(), s2.to_string());
}
