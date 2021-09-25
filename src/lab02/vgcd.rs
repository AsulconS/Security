extern crate num;

pub fn vecGCD<T: Copy + num::Integer>(vec: &Vec<T>) -> T
{
    let mut res: T = num::zero();
    for iref in vec.iter()
    {
        if *iref == num::zero() { continue; }
        else { res = num::integer::gcd(res, *iref); }
    }
    return res;
}
