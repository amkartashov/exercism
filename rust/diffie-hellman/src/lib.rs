///  https://en.wikipedia.org/wiki/Linear_congruential_generator
///  TODO: use mutex
const LCG_SEED: u128 = 1;
const LCG_A: u128 = 16807;
const LCG_M: u128 = 2147483647;
static mut LCG_RANDOM: u128 = LCG_SEED;
fn lcg_random() -> u64 {
    let mut random = unsafe { LCG_RANDOM };
    random = (LCG_A * (random)) % LCG_M;
    unsafe {
        LCG_RANDOM = random;
    }
    return random as u64;
}

pub fn private_key(p: u64) -> u64 {
    let mut res = lcg_random() % p;
    loop {
        if res > 1 {
            return res;
        }
        res = lcg_random() % p;
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    exp_modulo(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    exp_modulo(b_pub, a, p)
}

/// based on https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
fn exp_modulo(mut base: u64, mut exp: u64, modulos: u64) -> u64 {
    if modulos == 1 {
        return 0;
    }

    base = base % modulos;

    let mut result = 1;

    while exp > 0 {
        if (exp % 2) == 1 {
            result = ((result as u128 * base as u128) % modulos as u128) as u64;
        }
        exp = exp / 2;
        base = ((base as u128 * base as u128) % modulos as u128) as u64;
    }

    result
}
