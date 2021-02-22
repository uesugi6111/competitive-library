
pub fn rolling_hash(s: &[char], start: usize, l: usize) -> bool {
    let b: u128 = 1_000_000_007;
    let target_start = start + l;

    let pow_b = b.wrapping_pow(l as u32);

    let mut target_hash: u128 = 0;
    let mut base_hash: u128 = 0;
    for k in 0..l {
        base_hash = base_hash.wrapping_mul(b) + s[start + k] as u128;
        target_hash = target_hash.wrapping_mul(b) + s[target_start + k] as u128;
    }

    for k in 0..s.len() - target_start {
        if target_hash == base_hash {
            return true;
        }
        if target_start + l + k < s.len() {
            target_hash = target_hash
                .wrapping_mul(b)
                .wrapping_add(s[target_start + l + k] as u128)
                .wrapping_sub((s[target_start + k] as u128).wrapping_mul(pow_b));
        }
    }
    false
}

#[test]
fn test_rolling_hash() {
    let mut mached = false;

    for i in 0..3375 {
        if i + 21 * 2 > 3375 {
            break;
        }
        if rolling_hash(&("hhggggghhhhhgghhhhhghgggggghggggghgghhggghhggghhhhhgggghhggggghhghghghhhhhhhgggghhhgghgggghhhggghhhgghhghhhghhghhhhhghghhghghggghhhhhghhgghghghhhhhghhhhgghhhhhghghgggggghhgghgghhghhgghghghghhghhhhhggggghggggghggggghhhhgggggghghgghhhhhhggghhhggghhghghhghhhhhhghghghhhghhhhgghhghghghgggghhghhhhgghhghghgghggghgghggggghggghhggghggghghhhgghhgggghhghhhhhghgghhhhghghhggghggghggghhhghhhgggghhhhghggghggghhggggghhghhhhhhhhggghgghhghhhhhgghhhhghhgghgghghgghghgghhgghggghghgghgghghggghghhghghhhhghhhhgghhghghhhhhghghhghghghgghhhghgghhhhhgggghhhhghghghgghhhghhhhhhhhhhhhghghghgghhhggghhhgghhhgghgghghggghhhgghhgggghhggghhghhghhhhghgghhgggghhghghhgghggggghgggghhghghhgghggggggggghhggghhhhghhhhghhggggggghhhhghhgghhhggghhghhghgghhhgghhhghgghhhhhhgghhhhgggghgghhhhhghghhhhgghhggghggghhgggghghhgghghhhgghhhhghghghggghhgggghhhhgghghhhhhhhhgghhhhgggghhgghhhghggghhghghggghhhghghhghgghggghhghhghgghghghhhgggggghghggghhhhgghhhhghghhgggghghhghgghghhhgghghhhhhghghghhgghhhhghhhgghghggggghghhhgghhhhghghhgghhghhhgghgghhhghghhgghggghhghghhgghgggghghhhhghhggghhhhhhggghgghhhghggggghhgggghhghhgghhgghhhhhghhhhhhgghhghhhhhghghghghghhggggggghghgghggghhhhhgghghhhghgghgghhhghghhhhggggghhghghghhgghghgghhgggggggghhhhhghghhhghghhhhhgghhgghhghgghghggghhgghghghhghghhghhhhghhhgggghghhghhhghhhhghhgghggghghghhghghhhggghhhgggghhggghhghgggghhhghghhhghhhhhhhghhgggghhhhgghggghhgghhhhgggghgggghhhhghhhggghhhgggggghggghgghghhhghggghghgghgggghhghhhghhgghghhhhghhgghhgghhhhhggghggghhghhghhgggghghgggghgghggghhhhhhghhghgghghhhhhghghhhggghgghghghgggghghhgghhhgghhgggggghggghhggghhhghhghghggghhggghghhhhhgghhgghhhhggghhhhgggghhhgghhggghghhghghghghhgghghggghgghgghghhgghhhghgghghhggghghhghhhhgghhghghhhghghhghghghhgghhhgghghhghggggghhhhhggggggggghhhhgghhhgghhghghggghghghhgggghghghhhgghgghhghhgghggggggghggghggghghgggghghhggghgghhhhhhgghhghggghgghhhhghhghghhhggggghhghhgghhgghhgghggghggghhhhhgghgggggghghhhhhghhhghhhghghghhhhhghggghhgghhhhhgggghhghhhhhghhgghhhgghgggghghgghggghhhhhhhhggghghghhhghhghhgghhghghhhgghghgghhhhggghhhggghhgghghgghhhhhggghhghghghghhgghghghhghghghhgggggghghhhghhhhhggghgghghhgghghhhhghhhhgghhghghhggggghghghghghhghgghggghhghhggghggghhggghhgghghhghggghgggghgghhhghgghgghhhghghghhghhhggghhhhgghhhhgghhhghghhghggggggghhhhghghhhgghhggghhghgghgghhhghhhghggghhggghghhgghhhghgghhhghhgghghhhghgghggghgghhghhggghhghhhhghgghgghggggghgghhhhhhghhgggggggghggggghgggggggghghhhhghhghggghgggghghghhghgghhgghhgghghhhghhghhghhgghgghhhgghghhhhgghghgghhgggghggggggghghghhghgggghghhgggghgghghhhhhgghhghgghghhggghghhghhghgghhggghhhhgghhhhhgggggghhggghhghhggghgghhghhhghhhhhgghhghgggggghgghggghhghgghghhgggggghgghhhghhhhghgghhhhhhghghhgghgghhgghhgghhhhhggggghhghhgghhghgghghghhhghgghggghghghgggghgghhggghghgghghhhhhhgggggghghghhhghhghhhgghghghgghhhghhggghggggggggghhhgghghhhhhhghgghgghgghghgghhhhhhgggghghhhhgggghgggggghhhghghghgghhghghhhhghgghhhhhgggggggggghghghggggggghgghhgghghhghhhhggghhghgggghghhghgghgghggghhhgghhhghgghhhhhghggghhhhghhghhhhghhggggghhhhhhhhghhhghghhggghhhhgggggghghhhgggghghhhgghhggghhghhghhgghggggggghhhgghgghghhhghgghhhghhhhhgghgghhhhgghhhhhhghhggghhhgghggggghghghghgghghgghhhhhhhhhhgghhhgghgghghhghhghgghgghggghghggghhhgghgghhhghghgghghghhhgghhggghhggggggghgghghhghghghhhhghhgghhhgghghhhghhghhhhhghhhgghgghhhghhhhghhhghghgghhghhgggggghgghghghhhghgghhhhhhhhhghghhhhhggggghgggghhhghgghhhghhhghhgghghhghggghggghhghgghhhghghhhhhggghhhghghhhgghhhhgghgggghhhghgghhggghhhggggghghhhgggghghgghhggghgg".chars().collect::<Vec<_>>()), i, 21) {
            mached = true;
            break;
        }
    }
    assert_eq!(mached, true);
}
