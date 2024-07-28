use wasm_bindgen::prelude::*;

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
