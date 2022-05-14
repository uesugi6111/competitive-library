#![feature(const_eval_limit)]
#![const_eval_limit = "0"]
const fn e() -> [usize; 78498] {
    let mut a = [true; 1_000_001];
    let mut ret_index = 0;
    let mut ret = [0; 78_498];
    let mut i = 1;

    while i < 1_000_000 {
        i += 1;
        if !a[i] {
            continue;
        }
        ret[ret_index] = i;
        ret_index += 1;
        let mut index = i;
        while index < 1_000_000 {
            a[index] = false;
            index += i;
        }
    }
    ret
}

const E: [usize; 78498] = e();
#[test]
fn test() {
    println!("{}", E.len());
}
