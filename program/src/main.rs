#![no_main]

sp1_zkvm::entrypoint!(main);

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn main() {
    let number = sp1_zkvm::io::read::<u32>();
    let result = is_prime(number);
    sp1_zkvm::io::commit(&result);
}
