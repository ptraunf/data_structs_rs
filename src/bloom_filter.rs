use std::fmt::Display;
use fasthash::{murmur3, spooky};
use crate::set::Set;

pub struct BloomFilter {
    // Max number of elements
    capacity: usize,
    // Probability of false positives
    p: f64,
    // Number of bits needed to achieve p
    num_bits: usize,
    // Number of indices per element
    k: usize,
    // The filter
    bytes: Box<[u8]>,
    // Functions that compute the k'th index of an input
    hash_fns: Vec<Box<dyn Fn(String) -> usize>>,
}

impl BloomFilter {
    pub fn new(capacity: usize, p: f64) -> Self {
        // let ln2 = 2_f64.ln();

        let m: usize = ((p.ln() * -1.0 * (capacity as f64)) / 2_f64.ln().powi(2)) as usize;

        let k = m / capacity;

        let num_bytes = m.div_ceil(8);

        let mut hash_fns = Vec::new();
        for i in 0..k {
            hash_fns.push(Self::gen_hash_fn(i as u32, m as u32));
        }

        BloomFilter {
            capacity,
            p,
            num_bits: m,
            k,
            bytes: vec![0x00; num_bytes].into_boxed_slice(),
            hash_fns,
        }
    }
    fn bit_coordinates(index: usize) -> (usize, usize) {
        let byte_index = index / 8;
        let bit_offset = index % 8;
        (byte_index, bit_offset)
    }
    fn read_bit(&self, index: usize) -> u8 {
        let (byte_index, bit_offset) = BloomFilter::bit_coordinates(index);
        let bitmask: u8 = 1 << bit_offset;
        (self.bytes[byte_index] & bitmask) >> bit_offset
    }
    fn write_bit(&mut self, index: usize) {
        let (byte_index, bit_offset) = BloomFilter::bit_coordinates(index);
        self.bytes[byte_index] |= 1 << bit_offset;
    }
    fn key_to_bit_indicies(&self, key: String) -> Vec<usize> {
        let mut indicies: Vec<usize> = Vec::new();
        for i in 0..self.k {
            indicies.push(self.hash_fns[i](key.clone()));
        }
        indicies
    }
    fn gen_hash_fn(i: u32, num_bits: u32) -> Box<dyn Fn(String) -> usize> {
        let hash_fn: Box<dyn Fn(String) -> usize> = Box::new(move |s: String| {
            let s = &s;
            let murmur = murmur3::hash32(s);
            let spooky: u32 = spooky::hash32(s);
            // XOR of Murmur + i and Spooky + i squared, fit to the
            let h: u32 = ((murmur + i) ^ (spooky + i * i)) % num_bits;
            h as usize
        });
        hash_fn
    }
    pub fn insert(&mut self, value: String) {
        let indicies = self.key_to_bit_indicies(value);
        for i in indicies {
            self.write_bit(i);
        }
    }
    pub fn contains(&self, value: String) -> bool {
        let indicies = self.key_to_bit_indicies(value);
        let mut set_bits = 0;
        for i in indicies {
            set_bits += self.read_bit(i);
        }
        set_bits as usize == self.k
    }
    pub fn false_positive_probability(&self) -> f64 {
        self.p.clone()
    }
}

impl Set for BloomFilter {
    type T = String;

    fn insert(&mut self, e: String) {
        self.insert(e)
    }
    fn contains(&self, e: String) -> bool {
        self.contains(e)
    }
}

impl Display for BloomFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "n:\t{}\np:\t{:.5}\nm:\t{}\nk:\t{}\n",
            self.capacity,
            self.p,
            self.num_bits,
            self.k,
        )
    }
}

#[cfg(test)]
mod test {
    use super::BloomFilter;

    #[test]
    fn test_init_bloom_filter() {
        let capacity = 32;
        let p_of_false_positive = 0.5;
        let bf: BloomFilter = BloomFilter::new(capacity, p_of_false_positive);
        println!("BF:\n{bf}");
    }

    #[test]
    fn test_bloom_filter_insert_contains() {
        let capacity = 128;
        let p_of_false_positive = 0.01;
        let mut bf: BloomFilter = BloomFilter::new(capacity, p_of_false_positive);
        println!("BF:\n{bf}");
        let s = String::from("Blah blah blah");

        bf.insert(s.clone());
        let contains_s = bf.contains(s.clone());
        println!("BF contains {s}? {contains_s}");
        assert!(contains_s);
        println!("BF:\n{bf}");
    }

    #[test]
    fn test_bloom_filter_does_not_contain() {

        let capacity = 64;
        let p_of_false_positive = 0.01;
        let mut bf: BloomFilter = BloomFilter::new(capacity, p_of_false_positive);
        println!("BF:\n{bf}");
        let s = String::from("Blah blah blah");
        let contains_s = bf.contains(s.clone());
        assert!(!contains_s);

        bf.insert(String::from("A different string"));

        assert!(!contains_s);
        println!("BF contains {s}? {contains_s}");

        println!("BF:\n{bf}");
    }
}