use crate::BigUint;
use crate::num::FromPrimitive;

// help functions
pub fn is_simple(wit: usize) -> Option<usize>{
    for i in 2..wit/2{
        if wit % i == 0{
            return None;
        }
    }
    Some(wit)
}
pub fn big_pow(base: &BigUint, exp: &BigUint) -> BigUint
{
    if *exp == BigUint::from_u32(0).unwrap()
    {
        return BigUint::from_u32(1).unwrap();
    }
    let mut answer = base.clone();
    let mut i = BigUint::from_u32(1).unwrap();
    while i < *exp
    {
        i = i + BigUint::from_u32(1).unwrap();
        answer = answer * base.clone();
    }
    return answer;
}