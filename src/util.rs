use rand::{distr::StandardUniform, prelude::*};

use rand_mt::Mt64;
pub type MTRng = Mt64;

#[inline]
pub fn gen_rng(seed: u64) -> MTRng {
    MTRng::new(seed)
}

pub fn fill_rand<'a, I, T: 'a, R>(a: I, rng: &mut R)
where
    I: IntoIterator<Item = &'a mut T>,
    R: rand::Rng,
    StandardUniform: rand::distr::Distribution<T>,
{
    for v in a.into_iter() {
        *v = rng.sample(StandardUniform);
    }
}

pub fn myrand<R: Rng>(n: usize, rng: &mut R) -> Vec<f64> {
    let mut d: Vec<f64> = vec![0.; n];
    fill_rand(&mut d, rng);
    d
}
