use rand::distributions::Uniform;

use rand::{thread_rng, Rng};
use std::cmp::{max, min, Ord};
use std::fmt::Debug;
use std::fmt::Display;
use std::time::Instant;

pub struct PriorityQueue<T: Ord + Copy + Display + Debug> {
    heap: Vec<T>,
}
impl<T: Ord + Copy + Display + Debug> From<Vec<T>> for PriorityQueue<T> {
    fn from(value: Vec<T>) -> Self {
        PriorityQueue { heap: value }
    }
}
impl<T: Ord + Copy + Display + Debug> Display for PriorityQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let p = 0;
        let size = self.heap.len();
        let max_level = max(1, (size as f64).log2().floor() as usize);
        let mut display_string = String::new();
        for l in 0..=max_level {
            display_string.push_str(&format!("{}", l));
            let num_indents = max(0, (2i32.pow(max_level as u32 - l as u32) / 2) - 1) as usize;
            writeln!(f, "Num Indents: {num_indents}")?;
            for _ in 0..num_indents {
                display_string.push_str("    ");
            }
            // Start and End of row determined by level
            let start: usize = (u32::pow(2, l as u32) - 1) as usize;
            let end: usize = (u32::pow(2, (l + 1) as u32) - 2) as usize;
            // TODO: Variable-width space-padding
            for i in start..=min(end, size - 1) {
                // display_string.push_str(&format!("[{}]{} ", i, self.heap[i].to_string().as_str()));
                display_string.push_str(&format!("  {}  ", self.heap[i].to_string().as_str()))
            }
            display_string.push('\n');
        }
        f.write_str(display_string.as_str())
    }
}
impl<T: Ord + Copy + Display + Debug> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue { heap: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(item);
        self.heapify();
    }
    pub fn append(&mut self, items: &mut Vec<T>) {
        self.heap.append(items);
        self.heapify();
    }
    pub fn pop(&mut self) -> Option<T> {
        let value = self.heap.drain(0..=0).last();
        value
    }
    pub fn peek(&self) -> T {
        self.heap[0]
    }
    pub fn size(&self) -> usize {
        self.heap.len()
    }
    pub fn heapify(&mut self) {
        let size = self.heap.len();
        let max_parent = if size % 2 == 0 {
            (size - 2) / 2
        } else {
            (size - 1) / 2
        };
        for i in (0..=max_parent).rev() {
            // println!("i: {i}");
            let mut largest = i;
            let left = (2 * i) + 1;
            let right = (2 * i) + 2;

            if left < size && self.heap[left] > self.heap[largest] {
                largest = left;
            }
            if right < size && self.heap[right] > self.heap[largest] {
                largest = right
            }
            if largest != i {
                // println!("Swapping [{}]{} with [{}]{}", i, self.heap[i], largest, self.heap[largest]);
                self.heap.swap(i, largest);
            }
        }
    }
}

pub mod test {
    use super::*;

    fn generate_test_vec<T>(min: T, max: T, size: usize) -> Vec<T>
        where
            T: Sized + rand::distributions::uniform::SampleUniform,
    {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(min, max);
        let mut v: Vec<T> = Vec::new();
        for _ in 0..size {
            v.push(rng.sample(&range));
        }
        v
    }
    #[test]
    pub fn test_pq() {
        let size = 31;
        let min: u32 = 1;
        let max: u32 = 96;
        println!("Testing with {size} elements:");
        let now = Instant::now();
        {
            // let mut rng = rand::thread_rng();
            // let range = rng.gen_range(min..max)
            // let mut test_vec: Vec<u32> = Standard.sample_iter(rng).take(size).collect();
            let mut test_vec: Vec<u32> = generate_test_vec::<u32>(min, max, size);
            let mut pq: PriorityQueue<u32> = PriorityQueue::new();
            pq.append(&mut test_vec);
            println!("PQ:\n{}", pq);
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}
