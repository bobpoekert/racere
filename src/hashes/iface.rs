
pub trait HashFunction {
    fn get_hash(&self, vec : Vec<f64>, hashes : &mut Vec<u32>);
    fn get_dims(&self) -> usize;
}