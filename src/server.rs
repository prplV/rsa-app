//mods 
mod algs;

use crate::rand::Rng;
use crate::BigUint;
use crate::num::bigint::ToBigUint;
use crate::num::ToPrimitive;
use crate::server::algs::{is_simple, big_pow};

pub struct Server{
    private_key: [usize; 2],
    public_key: [usize; 2],
}

impl Server{
    pub fn new() -> Self{
        Self{
            private_key: [0, 0],
            public_key: [0, 0],
        }
    }

    pub fn gen_key_pair(&mut self){
    
        // elemnets initialization 
        let mut p = 0;
        let mut q = 0;
        let mut e = 0;
        let mut d = 0;
        let mut n = 0;
        
        let mut generator = rand::thread_rng();
    
        'loop_for_check: loop{
    
            // getting p and q elements => calculating n element
            'p: loop {
                let ver = is_simple(generator.gen_range(2..=50));
                match ver {
                    Some(value) => {
                        p = value;
                        break 'p;
                    },
                    None => continue,
                }
            }
            'q: loop {
                let ver = is_simple(generator.gen_range(2..=50));
                match ver {
                    Some(value) => {
                        if p != value{
                            q = value;
                            break 'q;
                        }
                        else  { 
                            continue;
                        }
                    },
                    None => continue,
                }
            }
            n = p * q;
        
            // getting e element
            let sigma = (p - 1)*(q - 1);
            'e: loop {
                match is_simple(generator.gen_range(2..sigma))
                {
                    Some(value) => {
                        if sigma % value == 1{
                            e = value;
                            break 'e;
                        } else{
                            continue;
                        }
                    },
                    None => continue,
                }
            }
            
            // calculating d element
            
            let difference: usize = sigma / e;
            d = sigma - difference;
            
    
            if d != e {
                println!("public key is ({}, {})", e, n);
                println!("private key is ({}, {})", d, n);
                break 'loop_for_check;
            } else{
                continue;
            }
        }
        self.public_key = [e, n];
        self.private_key = [d, n];
    }
    pub fn calculating_initial_kc_value(&self, r_value: BigUint) -> usize{
        // to u32 converting 
        let d_value = self.private_key[0] as u32;
        let n_value = self.private_key[1] as u32;
        //let r_value = r_value as u32;
    
        //to BigUint converting
        let d_value_bi: BigUint = d_value.to_biguint().unwrap();
        let n_value_bi: BigUint = n_value.to_biguint().unwrap();
        //let r_value_bi: BigUint = r_value.to_biguint().unwrap();
    
        // calculations
        let f: BigUint = big_pow(&r_value.into(), &d_value.into());
        let res: BigUint = f % n_value;
    
        //reverse converting
        let res: usize = res.to_usize().unwrap();
        println!("initial k (decrypted) = {}", res);
        res
    }
    pub fn get_public_key(&self) -> [usize; 2]{
        self.public_key
    }
}