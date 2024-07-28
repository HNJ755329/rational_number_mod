use wasm_bindgen::prelude::*;

fn main() {
    let u = 399297744;
    let modulus = 998244353;
    if let Some((a, b)) = qmod(u, modulus, true) {
        println!("{}/{}", a, b);
    }
    println!("done");
    test();
}

fn test() {
    //let modulus = 998244353;
    //let modulus = 1000000009;
    let modulus = 1000000007;
    let n = ((modulus / 2) as f64).sqrt() as i64;
    println!("n={}", n);
    for a in 1..10 {
        for b in 1..1000 {
            // u = a/b (mod M);
            testab(a, b, modulus);
        }
    }
    testab(n - 2, n - 1, modulus);
    return;
}

fn testab(a: i64, b: i64, modulus: i64) {
    let u = (a * modpow(b, modulus - 2, modulus)) % modulus;
    if let Some((num, div)) = qmod(u, modulus, false) {
        if num * b != div * a {
            println!("err {}/{}={},{}/{}", a, b, u, num, div);
        }
    } else {
        println!("non {}/{}={}", a, b, u);
    }
}

fn modpow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut res = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            res *= base;
            res %= m;
        }
        base = base * base;
        base %= m;
        exp /= 2;
    }
    res
}

#[wasm_bindgen]
pub struct Quotient(pub i64, pub i64);

#[wasm_bindgen]
pub fn qmod_wasm(u: i64, m: i64, debug: bool) -> Quotient {
    let n = ((m / 2) as f64).sqrt() as i64;
    let mut a1: i64 = m;
    let mut a2: i64 = u;
    let mut b1: i64 = 0;
    let mut b2: i64 = 1;
    let mut i = 0;
    if debug {
        println!("a{}/b{}={}/{}", i, i, a1, b1);
        i += 1;
        println!("a{}/b{}={}/{}", i, i, a2, b2);
    }
    while b2.abs() < n {
        if a2.abs() < n {
            //i += 1;
            //if debug {
            //    println!("{}:a/b={}/{}", i, b2.signum() * a2, b2.abs());
            //}
            return Quotient(b2.signum() * a2, b2.abs());
        }
        let q = a1 / a2;
        let tmpa2 = a2;
        a2 = a1 - q * a2;
        a1 = tmpa2;
        let tmpb2 = b2;
        b2 = b1 - q * b2;
        b1 = tmpb2;
        i += 1;
        if debug {
            println!("a{}/b{}={}/{}", i, i, a2, b2);
        }
    }
    return Quotient(-1, -1);
}

pub fn qmod(u: i64, m: i64, debug: bool) -> Option<(i64, i64)> {
    let n = ((m / 2) as f64).sqrt() as i64;
    let mut a1: i64 = m;
    let mut a2: i64 = u;
    let mut b1: i64 = 0;
    let mut b2: i64 = 1;
    let mut i = 0;
    if debug {
        println!("a{}/b{}={}/{}", i, i, a1, b1);
        i += 1;
        println!("a{}/b{}={}/{}", i, i, a2, b2);
    }
    while b2.abs() < n {
        if a2.abs() < n {
            //i += 1;
            //if debug {
            //    println!("{}:a/b={}/{}", i, b2.signum() * a2, b2.abs());
            //}
            return Some((b2.signum() * a2, b2.abs()));
        }
        let q = a1 / a2;
        let tmpa2 = a2;
        a2 = a1 - q * a2;
        a1 = tmpa2;
        let tmpb2 = b2;
        b2 = b1 - q * b2;
        b1 = tmpb2;
        i += 1;
        if debug {
            println!("a{}/b{}={}/{}", i, i, a2, b2);
        }
    }
    return None;
}
