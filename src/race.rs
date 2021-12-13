use crate::hashes::iface::HashFunction;

pub struct RACE<H : HashFunction> {
    _R : usize,
    _range : usize,
    _sketch : Vec<u32>,
    hash_scratch: Vec<u32>,
    hasher: H
}

impl<H : HashFunction> RACE<H> {
    pub fn new(h: H, R: usize, range: usize) -> RACE<H> {
        let n_dims = h.get_dims();
        return RACE {
            _R: R,
            _range: range,
            _sketch: Vec::with_capacity(R * range),
            hash_scratch: Vec::with_capacity(n_dims),
            hasher: h
        };
    }

    fn hash(&mut self, vec: Vec<f64>) {
        self.hash_scratch.fill(0);
        self.hasher.get_hash(vec, &mut self.hash_scratch);
    }

    pub fn add(&mut self, vec: Vec<f64>) {
        self.hash(vec);
        for r in 0..self._R {
            let index = self.hash_scratch[r] % (self._range as u32);
            self._sketch[r * self._range + (index as usize)] += 1;
        }
    }

    pub fn subtract(&mut self, vec: Vec<f64>) {
        self.hash(vec);
        for r in 0..self._R {
            let index = self.hash_scratch[r] % (self._range as u32);
            self._sketch[r * self._range + (index as usize)] -= 1;
        }
    }

    pub fn query(&mut self, vec: Vec<f64>) -> f64 {
        self.hash(vec);
        let mut mean = 0.;
        for r in 0..self._R {
            let index = self.hash_scratch[r] % (self._range as u32);
            mean += self._sketch[r * self._range + (index as usize)] as f64;
        }
        return mean / (self._R as f64);
    }

}

