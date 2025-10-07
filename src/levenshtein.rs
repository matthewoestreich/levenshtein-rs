const IGNORE: &str = "I";
const ADD: &str = "A";
const REMOVE: &str = "R";
const SUBSTITUTE: &str = "S";

pub fn levenshtein(s1: String, s2: String) -> i32 {
    let mut cache: Vec<Vec<i32>> = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    let mut actions: Vec<Vec<String>> = vec![vec![String::new(); s2.len() + 1]; s1.len() + 1];

    for row in &mut cache {
        for value in row {
            *value = 0;
        }
    }

    for row in &mut actions {
        for value in row {
            *value = "-".to_string();
        }
    }

    cache[0][0] = 0;
    actions[0][0] = IGNORE.to_string();
    trace_cache(&cache, &actions);

    for n2 in 1..s2.len() + 1 {
        let n1 = 0;
        cache[n1][n2] = n2 as i32;
        actions[n1][n2] = ADD.to_string();
        trace_cache(&cache, &actions);
    }

    for n1 in 1..s1.len() + 1 {
        let n2 = 0;
        cache[n1][n2] = n1 as i32;
        actions[n1][n2] = REMOVE.to_string();
        trace_cache(&cache, &actions);
    }

    for n1 in 1..s1.len() + 1 {
        for n2 in 1..s2.len() + 1 {
            if let Some(s1c) = s1.chars().nth(n1 - 1) {
                if let Some(s2c) = s2.chars().nth(n2 - 1) {
                    if s1c == s2c {
                        cache[n1][n2] = cache[n1 - 1][n2 - 1];
                        actions[n1][n2] = IGNORE.to_string();
                        trace_cache(&cache, &actions);
                        continue;
                    }
                }
            }

            let remove = cache[n1 - 1][n2];
            let add = cache[n1][n2 - 1];
            let subs = cache[n1 - 1][n2 - 1];

            cache[n1][n2] = remove;
            actions[n1][n2] = REMOVE.to_string();

            if cache[n1][n2] > add {
                cache[n1][n2] = add;
                actions[n1][n2] = ADD.to_string();
            }

            if cache[n1][n2] > subs {
                cache[n1][n2] = subs;
                actions[n1][n2] = SUBSTITUTE.to_string();
            }

            cache[n1][n2] += 1;

            trace_cache(&cache, &actions);
        }
    }

    //for row in cache {
    //    println!("{row:?}");
    //}
    //for row in actions {
    //    println!("{row:?}");
    //}

    let mut trace: Vec<(String, String, Option<String>)> = vec![];
    let mut n1 = s1.len();
    let mut n2 = s2.len();
    let s1s: Vec<char> = s1.chars().collect();
    let s2s: Vec<char> = s2.chars().collect();

    while n1 > 0 || n2 > 0 {
        let action = actions[n1][n2].clone();

        if action == ADD {
            n2 -= 1;
            trace.push((ADD.to_string(), s2s[n2].to_string(), None));
        } else if action == REMOVE {
            n1 -= 1;
            trace.push((REMOVE.to_string(), s1s[n1].to_string(), None));
        } else if action == IGNORE {
            n1 -= 1;
            n2 -= 1;
            trace.push((IGNORE.to_string(), s1s[n1].to_string(), None));
        } else if action == SUBSTITUTE {
            n1 -= 1;
            n2 -= 1;
            trace.push((
                SUBSTITUTE.to_string(),
                s1s[n1].to_string(),
                Some(s2s[n2].to_string()),
            ));
        }
    }

    trace.reverse();
    println!("{trace:?}");
    return cache[n1][n2];
}

fn trace_cache(cache: &[Vec<i32>], actions: &[Vec<String>]) {
    for row in 0..cache.len() {
        for col in 0..cache[row].len() {
            let item = cache[row][col];
            let action = actions[row][col].clone();
            print!("{:<6}", format!("{item} ({action})"));
        }
        println!();
    }
    println!();
}
