use std::cmp::min;

fn set_partition(c: &[i32], k: i32, d: i32) -> i32 {
    let n = c.len() as i32;

    if k == n {
        d.abs()
    } else {
        let s1 = set_partition(c, k + 1, d - c[k as usize]);
        let s2 = set_partition(c, k + 1, d + c[k as usize]);

        min(s1, s2)
    }
}

fn main() {
    let set = [17, 20, 10, 3];

    println!("Set Partition Min: {}", set_partition(&set, 0, 0));
}
