#![cfg(all(feature = "std", feature = "std_rng"))]

use rand::distributions::{Normal, Uniform};
use iface::{HashFunction};

struct L2Hash {
    _dim: usize,
    _nhashes: usize,
    _w: f64,
    _C: Vec<f64>,
    _b: Vec<f64>,
    seed: uint64
}

impl HashFunction for L2Hash {
    fn get_hash(&self, vec : Vec<f64>, hashes : &mut Vec<u32>) {
        for k in 0..self._nhashes {
            let mut value : f64 = 0;
            for i in 0..self._dim {
                value += self._C[k * self._dim + i] * vec[i];
            }
            value += self._b[k];
            hashes[k] = floor(value / self._w);
        }
    }
}

fn make(dimensions: usize, n_hashes: usize, w: double, s: uint) -> L2Hash {
    let mut res : L2Hash;
    res.seed = s;
    res._dim = dimensions;
    res._nhashes = n_hashes;
    res._w = w;

    res._b = Vec::new(n_hashes);
    res._C = Vec::new(n_hashes * dimensions);

    let mut rng = rand::thread_rng();
    let uniform = Uniform::new(0.0, w);
    let gaussian = Normal::new(0.0, 1.0);

    for i in 0..(n_hashes * dimensions) {
        res._C[i] = gaussian.sample(&mut rng);
    }

    for i in 0..n_hashes {
        res._b[i] = uniform.sample(&mut rng);
    }

    return res;
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_hash() {
        let dim = 10;
        let N = 10;
        let w = 0.01;

        let hash = make(dim, N, w);

        let mut v : Vec<f64> = Vec::new(dim * 5);
        let mut hashes : Vec<u32> = Vec::new(N);


    }
}