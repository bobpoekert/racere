use rand::distributions::{Uniform};
use rand_distr::{Normal, Distribution};
use super::iface::{HashFunction};



pub struct L2Hash {
    _dim: usize,
    _nhashes: usize,
    _w: f64,
    _C: Vec<f64>,
    _b: Vec<f64>
}

impl HashFunction for L2Hash {
    fn get_hash(&self, vec : Vec<f64>, hashes : &mut Vec<u32>) {
        for k in 0..self._nhashes {
            let mut value : f64 = 0.;
            for i in 0..self._dim {
                value += self._C[k * self._dim + i] * vec[i];
            }
            value += self._b[k];
            hashes[k] = (value / self._w).floor() as u32;
        }
    }

    fn get_dims(&self) -> usize {
        return self._dim;
    }
}

pub fn make(dimensions: usize, n_hashes: usize, w: f64) -> Result<L2Hash, ()> {
    let mut res = L2Hash { 
        _dim: dimensions, 
        _nhashes: n_hashes, 
        _w: w, 
        _C: Vec::with_capacity(n_hashes * dimensions), 
        _b: Vec::with_capacity(n_hashes)
    };

    let mut rng = rand::thread_rng();
    let uniform = Uniform::new(0.0, w);
    match Normal::new(0.0, 1.0) {
        Err(x) => {
            return Err(())
        },
        Ok(gaussian) => {
            for i in 0..(n_hashes * dimensions) {
                res._C[i] = gaussian.sample(&mut rng);
            }

            for i in 0..n_hashes {
                res._b[i] = uniform.sample(&mut rng);
            }
            return Ok(res);
        }
    }

}
