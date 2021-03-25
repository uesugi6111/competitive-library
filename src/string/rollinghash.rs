pub fn rolling_hash(s: &[char], t: &[char]) -> bool {
    let base: u128 = 2u128.pow(61) - 1;
    let l = s.len();

    let pow_b = base.wrapping_pow(l as u32);

    let mut target_hash: u128 = 0;
    let mut base_hash: u128 = 0;
    for k in 0..l {
        base_hash = base_hash.wrapping_mul(base) + s[k] as u128;
        target_hash = target_hash.wrapping_mul(base) + t[k] as u128;
    }
    if target_hash == base_hash {
        return true;
    }
    for k in 0..t.len() - l {
        target_hash = target_hash
            .wrapping_mul(base)
            .wrapping_add(t[l + k] as u128)
            .wrapping_sub((t[k] as u128).wrapping_mul(pow_b));
        if target_hash == base_hash {
            return true;
        }
    }
    false
}

#[test]
fn test_rolling_hash() {
    let mut mached = false;
    let s = "hhggggghhhhhgghhhhhghgggggghggggghgghhggghhggghhhhhgggghhggggghhghghghhhhhhhgggghhhgghgggghhhggghhhgghhghhhghhghhhhhghghhghghggghhhhhghhgghghghhhhhghhhhgghhhhhghghgggggghhgghgghhghhgghghghghhghhhhhggggghggggghggggghhhhgggggghghgghhhhhhggghhhggghhghghhghhhhhhghghghhhghhhhgghhghghghgggghhghhhhgghhghghgghggghgghggggghggghhggghggghghhhgghhgggghhghhhhhghgghhhhghghhggghggghggghhhghhhgggghhhhghggghggghhggggghhghhhhhhhhggghgghhghhhhhgghhhhghhgghgghghgghghgghhgghggghghgghgghghggghghhghghhhhghhhhgghhghghhhhhghghhghghghgghhhghgghhhhhgggghhhhghghghgghhhghhhhhhhhhhhhghghghgghhhggghhhgghhhgghgghghggghhhgghhgggghhggghhghhghhhhghgghhgggghhghghhgghggggghgggghhghghhgghggggggggghhggghhhhghhhhghhggggggghhhhghhgghhhggghhghhghgghhhgghhhghgghhhhhhgghhhhgggghgghhhhhghghhhhgghhggghggghhgggghghhgghghhhgghhhhghghghggghhgggghhhhgghghhhhhhhhgghhhhgggghhgghhhghggghhghghggghhhghghhghgghggghhghhghgghghghhhgggggghghggghhhhgghhhhghghhgggghghhghgghghhhgghghhhhhghghghhgghhhhghhhgghghggggghghhhgghhhhghghhgghhghhhgghgghhhghghhgghggghhghghhgghgggghghhhhghhggghhhhhhggghgghhhghggggghhgggghhghhgghhgghhhhhghhhhhhgghhghhhhhghghghghghhggggggghghgghggghhhhhgghghhhghgghgghhhghghhhhggggghhghghghhgghghgghhgggggggghhhhhghghhhghghhhhhgghhgghhghgghghggghhgghghghhghghhghhhhghhhgggghghhghhhghhhhghhgghggghghghhghghhhggghhhgggghhggghhghgggghhhghghhhghhhhhhhghhgggghhhhgghggghhgghhhhgggghgggghhhhghhhggghhhgggggghggghgghghhhghggghghgghgggghhghhhghhgghghhhhghhgghhgghhhhhggghggghhghhghhgggghghgggghgghggghhhhhhghhghgghghhhhhghghhhggghgghghghgggghghhgghhhgghhgggggghggghhggghhhghhghghggghhggghghhhhhgghhgghhhhggghhhhgggghhhgghhggghghhghghghghhgghghggghgghgghghhgghhhghgghghhggghghhghhhhgghhghghhhghghhghghghhgghhhgghghhghggggghhhhhggggggggghhhhgghhhgghhghghggghghghhgggghghghhhgghgghhghhgghggggggghggghggghghgggghghhggghgghhhhhhgghhghggghgghhhhghhghghhhggggghhghhgghhgghhgghggghggghhhhhgghgggggghghhhhhghhhghhhghghghhhhhghggghhgghhhhhgggghhghhhhhghhgghhhgghgggghghgghggghhhhhhhhggghghghhhghhghhgghhghghhhgghghgghhhhggghhhggghhgghghgghhhhhggghhghghghghhgghghghhghghghhgggggghghhhghhhhhggghgghghhgghghhhhghhhhgghhghghhggggghghghghghhghgghggghhghhggghggghhggghhgghghhghggghgggghgghhhghgghgghhhghghghhghhhggghhhhgghhhhgghhhghghhghggggggghhhhghghhhgghhggghhghgghgghhhghhhghggghhggghghhgghhhghgghhhghhgghghhhghgghggghgghhghhggghhghhhhghgghgghggggghgghhhhhhghhgggggggghggggghgggggggghghhhhghhghggghgggghghghhghgghhgghhgghghhhghhghhghhgghgghhhgghghhhhgghghgghhgggghggggggghghghhghgggghghhgggghgghghhhhhgghhghgghghhggghghhghhghgghhggghhhhgghhhhhgggggghhggghhghhggghgghhghhhghhhhhgghhghgggggghgghggghhghgghghhgggggghgghhhghhhhghgghhhhhhghghhgghgghhgghhgghhhhhggggghhghhgghhghgghghghhhghgghggghghghgggghgghhggghghgghghhhhhhgggggghghghhhghhghhhgghghghgghhhghhggghggggggggghhhgghghhhhhhghgghgghgghghgghhhhhhgggghghhhhgggghgggggghhhghghghgghhghghhhhghgghhhhhgggggggggghghghggggggghgghhgghghhghhhhggghhghgggghghhghgghgghggghhhgghhhghgghhhhhghggghhhhghhghhhhghhggggghhhhhhhhghhhghghhggghhhhgggggghghhhgggghghhhgghhggghhghhghhgghggggggghhhgghgghghhhghgghhhghhhhhgghgghhhhgghhhhhhghhggghhhgghggggghghghghgghghgghhhhhhhhhhgghhhgghgghghhghhghgghgghggghghggghhhgghgghhhghghgghghghhhgghhggghhggggggghgghghhghghghhhhghhgghhhgghghhhghhghhhhhghhhgghgghhhghhhhghhhghghgghhghhgggggghgghghghhhghgghhhhhhhhhghghhhhhggggghgggghhhghgghhhghhhghhgghghhghggghggghhghgghhhghghhhhhggghhhghghhhgghhhhgghgggghhhghgghhggghhhggggghghhhgggghghgghhggghgg".chars().collect::<Vec<_>>();

    for i in 0..s.len() {
        if i + 21 * 2 > s.len() {
            break;
        }
        if rolling_hash(&s[i..i + 21], &s) {
            mached = true;
            break;
        }
    }
    assert_eq!(mached, true);
}
