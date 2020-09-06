use std::cmp::min;

static mut TOTAL: i32 = 0;

fn set_partition(c: &[i32], k: i32, d: i32) -> i32 {
    unsafe {
        TOTAL = TOTAL + 1;
    }

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
    let set = [3, 1, 4, 2, 2, 1];

    println!("Set Partition Min: {}", set_partition(&set, 0, 0));

    unsafe {
        println!("{}", TOTAL + 1);
    }
}
