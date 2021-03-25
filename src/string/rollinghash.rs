//const BASE: u128 = 2_305_843_009_213_693_951; // 2^61-1
const BASE: u128 = 1000000007;
pub fn rolling_hash(s: &[char], t: &[char]) -> bool {
    let l = s.len();

    let pow_b = BASE.wrapping_pow(l as u32);

    let mut target_hash: u128 = 0;
    let mut base_hash: u128 = 0;
    for k in 0..l {
        base_hash = base_hash.wrapping_mul(BASE) + unsafe { *s.get_unchecked(k) } as u128;
        target_hash = target_hash.wrapping_mul(BASE) + unsafe { *t.get_unchecked(k) } as u128;
    }
    if target_hash == base_hash {
        return true;
    }
    for k in 0..t.len() - l {
        target_hash = target_hash
            .wrapping_mul(BASE)
            .wrapping_add(unsafe { *t.get_unchecked(l + k) } as u128)
            .wrapping_sub((unsafe { *t.get_unchecked(k) } as u128).wrapping_mul(pow_b));
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
