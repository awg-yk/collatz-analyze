const JUMP_BITS: u32 = 20;
const TABLE_SIZE: usize = 1 << JUMP_BITS;
const MASK: u128 = (1u128 << JUMP_BITS) - 1;
const STEP_GUARD: u32 = 2_000_000;

static mut JUMP_A: [u32; TABLE_SIZE] = [0; TABLE_SIZE];
static mut JUMP_C: [u32; TABLE_SIZE] = [0; TABLE_SIZE];

#[no_mangle]
pub extern "C" fn table_a_ptr() -> *mut u32 {
    unsafe { JUMP_A.as_mut_ptr() }
}

#[no_mangle]
pub extern "C" fn table_c_ptr() -> *mut u32 {
    unsafe { JUMP_C.as_mut_ptr() }
}

#[no_mangle]
pub extern "C" fn table_len() -> u32 {
    TABLE_SIZE as u32
}

/// n の軌道を検証する。
/// 戻り値: 0以上 = 成功、進んだステップ数
///         -1 = u128の範囲を超える可能性があり安全に計算できない(JS側でBigIntにフォールバック)
///         -2 = STEP_GUARD超過(反例の可能性、要確認)
#[no_mangle]
pub extern "C" fn test_one(n_hi: u64, n_lo: u64) -> i64 {
    let n: u128 = ((n_hi as u128) << 64) | (n_lo as u128);
    let mut v: u128 = n;
    let mut steps: u32 = 0;

    while v >= n {
        let r = (v & MASK) as usize;
        let a = unsafe { JUMP_A[r] } as u128;
        let c = unsafe { JUMP_C[r] } as u128;
        let m = v >> JUMP_BITS;

        let next = match a.checked_mul(m).and_then(|am| am.checked_add(c)) {
            Some(x) => x,
            None => return -1,
        };
        v = next;
        steps += JUMP_BITS;
        if steps > STEP_GUARD {
            return -2;
        }
    }
    steps as i64
}
