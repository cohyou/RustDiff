fn main() {
    let old = "fn main() { println!(\"{:?}\", \"wowow\"); let old = 1; }";
    let new = "fn main() { let n = 1; println!(\"{:?}\", n); }";
    let n = string_compare(old, new, old.as_bytes().len()-1, new.as_bytes().len()-1);
    println!("{:?}", n);
}

#[allow(dead_code)]
enum Op {
    Match, Insert, Delete
}

fn string_compare(s: &str, t: &str, i: usize, j: usize) -> i32 {
    if i == 0 { return j as i32 * indel(0) }
    if j == 0 { return i as i32 * indel(0) }

    let target_s = s.as_bytes()[i];
    let target_t = t.as_bytes()[j];
    let opt = [
    string_compare(s, t, i-1, j-1) + match_string(target_s, target_t),
    string_compare(s, t, i  , j-1) + indel(target_t),
    string_compare(s, t, i-1, j  ) + indel(target_s)
    ];

    let mut lowest_cost;
    {
        lowest_cost = opt[0];
        for o in opt.iter() {
            if *o < lowest_cost {
                lowest_cost = *o;
            }
        }
    }
    return lowest_cost
}

#[allow(unused_variables)]
fn indel(s: u8) -> i32 {
    return 1
}

#[allow(unused_variables)]
fn match_string(c: u8, d: u8) -> i32 {
    return 1
}
