//mods 
mod algs;

use crate::num::bigint::ToBigUint;
use crate::rand::Rng;
use crate::client::algs::big_pow;
use crate::BigUint;

pub struct Client{
    k: usize,
}
impl Client{
    pub fn new() -> Self{
        Self{
            k: 0,
        }
    }
    pub fn calculating_r_value(&mut self, pub_key: [usize; 2]) -> BigUint{
        // getting random k element (k is in {1, ... , n-1})
        let e = pub_key[0] as u32;
        let n = pub_key[1] as u32;
        let mut generator = rand::thread_rng();
        let k = generator.gen_range(2..n) / 3 + 1;
        println!("k = {}", k);
        self.k = k as usize;
    
        //converting to BigUint
        let e: BigUint = e.to_biguint().unwrap();
        let k: BigUint = k.to_biguint().unwrap();
    
        // calculating r and returning 
        let r: BigUint = big_pow(&k, &e);
        r
    }
    
    pub fn validation(&self, to_check: usize) -> bool{
        if self.k == to_check{
            return true;
        }
        false
    }
}