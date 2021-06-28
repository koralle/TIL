use proconio::{fastout, input};

fn solve(a: &Vec<isize>, l: isize, k: isize, mid: &isize) -> bool {
    let mut cnt: isize = 0;
    let mut cutted: isize = 0;

    for pos in 0..a.len() {
        if a[pos] - cutted >= *mid && l - a[pos] >= *mid {
            cnt += 1;
            cutted = a[pos];
        }
    }

    if cnt >= k {
        return true;
    } else {
        return false;
    }
}

#[fastout]
fn main() {
    input! {
      n: isize,
      l: isize,
      k: isize,
      a: [isize; n],
    }

    let mut left = -1;
    let mut right = l+1;

    while right - left > 1 {
        let mid = (right + left) / 2;

        if solve(&a, l, k, &mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
