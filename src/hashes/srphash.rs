use rand_distr::{Normal, Distribution};
use super::iface::{HashFunction};

pub struct SRPHash {
    _dim: usize,
    _nhashes: usize,
    _C: Vec<f64>
}

impl HashFunction for SRPHash {
    fn get_hash(&self, vec : Vec<f64>, hashes : &mut Vec<u32>) {
        for k in 0..self._nhashes {
            let mut value = 0.;
            for i in 0..self._dim {
                value += self._C[k * self._dim + i] * vec[i];
            }
            if value > 0. {
                hashes[k] = 1;
            } else {
                hashes[k] = 0;
            }
        }
    }

    fn get_dims(&self) -> usize {
        return self._dim;
    }
}

pub fn make(dimensions: usize, n_hashes: usize) -> Result<SRPHash, ()> {
    let capacity = n_hashes * dimensions;
    let mut res = SRPHash {
        _dim: dimensions,
        _nhashes: n_hashes,
        _C: Vec::with_capacity(capacity)
    };

    let mut rng = rand::thread_rng();
    match Normal::new(0., 1.) {
        Err(_) => {
            return Err(());
        },
        Ok(gaussian) => {
            for i in 0..capacity {
                res._C[i] = gaussian.sample(&mut rng);
            }
            return Ok(res);
        }
    }
}