use std::fmt;

#[derive(Clone)]
pub enum Action {
    Ignore,
    Add,
    Remove,
    Substitute,
    None,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Add => write!(f, "A"),
            Action::Ignore => write!(f, "I"),
            Action::Remove => write!(f, "R"),
            Action::Substitute => write!(f, "S"),
            Action::None => write!(f, "-"),
        }
    }
}

pub fn levenshtein(s1: String, s2: String) -> i32 {
    let mut cache: Vec<Vec<i32>> = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    let mut actions: Vec<Vec<Action>> = vec![vec![Action::None; s2.len() + 1]; s1.len() + 1];

    for row in &mut cache {
        for value in row {
            *value = 0;
        }
    }

    for row in &mut actions {
        for value in row {
            *value = Action::None
        }
    }

    cache[0][0] = 0;
    actions[0][0] = Action::Ignore;
    trace_cache(&cache, &actions);

    for n2 in 1..s2.len() + 1 {
        let n1 = 0;
        cache[n1][n2] = n2 as i32;
        actions[n1][n2] = Action::Add;
        trace_cache(&cache, &actions);
    }

    for n1 in 1..s1.len() + 1 {
        let n2 = 0;
        cache[n1][n2] = n1 as i32;
        actions[n1][n2] = Action::Remove;
        trace_cache(&cache, &actions);
    }

    for n1 in 1..s1.len() + 1 {
        for n2 in 1..s2.len() + 1 {
            let s1c: Vec<char> = s1.chars().collect();
            let s2c: Vec<char> = s2.chars().collect();

            if s1c[n1 - 1] == s2c[n2 - 1] {
                cache[n1][n2] = cache[n1 - 1][n2 - 1];
                actions[n1][n2] = Action::Ignore;
                trace_cache(&cache, &actions);
                continue;
            }

            let remove = cache[n1 - 1][n2];
            let add = cache[n1][n2 - 1];
            let subs = cache[n1 - 1][n2 - 1];

            cache[n1][n2] = remove;
            actions[n1][n2] = Action::Remove;

            if cache[n1][n2] > add {
                cache[n1][n2] = add;
                actions[n1][n2] = Action::Add;
            }

            if cache[n1][n2] > subs {
                cache[n1][n2] = subs;
                actions[n1][n2] = Action::Substitute;
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

    let mut trace: Vec<Vec<String>> = vec![];
    let mut n1 = s1.len();
    let mut n2 = s2.len();
    let s1c: Vec<char> = s1.chars().collect();
    let s2c: Vec<char> = s2.chars().collect();

    while n1 > 0 || n2 > 0 {
        match actions[n1][n2] {
            Action::Ignore => {
                n1 -= 1;
                n2 -= 1;
                trace.push(vec![Action::Ignore.to_string(), s1c[n1].to_string()]);
            }
            Action::Add => {
                n2 -= 1;
                trace.push(vec![Action::Add.to_string(), s2c[n2].to_string()]);
            }
            Action::Remove => {
                n1 -= 1;
                trace.push(vec![Action::Remove.to_string(), s1c[n1].to_string()]);
            }
            Action::Substitute => {
                n1 -= 1;
                n2 -= 1;
                trace.push(vec![
                    Action::Substitute.to_string(),
                    s1c[n1].to_string(),
                    s2c[n2].to_string(),
                ]);
            }
            Action::None => { /* No op */ }
        }
    }

    trace.reverse();
    println!("{trace:?}");
    return cache[n1][n2];
}

fn trace_cache(cache: &[Vec<i32>], actions: &[Vec<Action>]) {
    for row in 0..cache.len() {
        for col in 0..cache[row].len() {
            let item = cache[row][col];
            let action = actions[row][col].clone();
            print!("{:<8}", format!("{item} ({action})"));
        }
        println!();
    }
    println!();
}
