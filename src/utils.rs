pub fn int_to_base(mut num: u128, base: u128) -> Vec<u128> {
    if num == 0 {
        return vec![0];
    }

    let mut digits =
        Vec::with_capacity(((num as f64).log2() / (base as f64).log2()).floor() as usize + 1);
    while num != 0 {
        digits.push(num % base);
        num /= base;
    }

    digits
}
