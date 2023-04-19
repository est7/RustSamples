#![allow(dead_code)]

static BANNER: &str = "Welcome to RustOS 3.14";
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

pub fn main() {
    //method()
}


fn method() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");

    println!("{BANNER}");
}

