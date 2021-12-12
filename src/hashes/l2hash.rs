use rand::distributions::{Uniform};
use rand_distr::{Normal, Distribution};
use super::iface::{HashFunction};



struct L2Hash {
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
}

fn make(dimensions: usize, n_hashes: usize, w: f64) -> Result<L2Hash, ()> {
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

#[cfg(test)]
mod tests {
    use super::super::iface::HashFunction;

    #[test]
    fn get_hash() {
        let target_map_hashes = [-1137,-786,1346,-1393,3351,-1608,2101,-2002,1030,-988];

        let dim = 10;
        let N = 10;
        let w = 0.01;

        match super::make(dim, N, w) {
            Err(_) => assert!(false),
            Ok(hash) => {
                let mut v : Vec<f64> = Vec::with_capacity(dim * 5);
                let mut hashes : Vec<u32> = Vec::with_capacity(N);

                let mut map : Vec<f64> = Vec::with_capacity(dim);
                map.fill(5.0);

                let mut maphashes = Vec::with_capacity(N);

                hash.get_hash(map, &mut maphashes);

                assert_eq!(target_map_hashes.len(), maphashes.len());
                for i in 0..maphashes.len() {
                    assert_eq!(target_map_hashes[i], maphashes[i]);
                }
            }

        }


    }
}