// example of iterator

use itertools::Itertools;

fn main() {
    let v1 = vec![1,2,3];
	let v1_iter = v1.iter();
	
	for val in v1_iter {
		println!("Got {}", val);
	}

    let mut v2_iter = Counter::new().zip(Counter::new().skip(1));
    for val in v2_iter {
        println!("{:?}", val);
    }

    let pack = pack_with_base(&vec![0,1,0,0,0], 8);
    println!("pack={:?}", pack);
    chain()
}

/// use of chain
fn chain() {
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    let chained = array1.iter().chain(array2.iter());

    for num in chained {
        println!("{}", num);
    }
}

/// Pack bits in the range [0,BIT_SIZE[ into a sparse keccak word with the
/// specified bit base
fn pack_with_base(bits: &[usize], base: usize) -> usize {
    bits.iter()
        .rev()
        .fold(0, |acc, &bit| acc * base + bit)
}

pub(crate) const NUM_BITS_PER_BYTE: usize = 8;
pub(crate) const NUM_BYTES_PER_WORD: usize = 8;
pub(crate) const RATE: usize = NUM_WORDS_TO_ABSORB * NUM_BYTES_PER_WORD;
pub(crate) const RATE_IN_BITS: usize = RATE * NUM_BITS_PER_BYTE;
pub(crate) const NUM_WORDS_TO_ABSORB: usize = 17;

fn chunks(){
    let mut bits = vec![];
    // Padding
    bits.push(1);
    while (bits.len() + 1) % RATE_IN_BITS != 0 {
        bits.push(0);
    }
    bits.push(1);
    println!("{:?} with len {:?}", bits, bits.len())
}

fn take_use(){
    let pre_s = [   [1,2,3,4,5],
                    [6,7,8,9,10],
                    [11,12,13,14,15],
                    [16,17,18,19,20],
                    [21,22,23,24,25] ];
    let hash_words: Vec<_> = pre_s
        .into_iter()
        .take(4)
        .map(|a| a[0].clone())
        .take(4)
        .collect();
    let hash_le_bytes = pre_s
        .into_iter()
        .take(4)
        .map(|a| a[0].clone())
        .rev()
        .collect::<Vec<_>>();
    println!("hash_words: {:?}, hash_le_bytes: {:?}", hash_words, hash_le_bytes);
}

fn multi_catersian_prod(){
    let part_size = 3;
    let range = 3;

    for (offset, perm) in (0..part_size)
        .map(|_| 0u64..range)
        .multi_cartesian_product()
        .enumerate()    
    {
        println!("offset: {:?}, perm: {:?}", offset, perm);
    };
}


#[derive(PartialEq, Debug)]
struct Shoe {
	size: i32,
	style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
	shoes.into_iter()
		 .filter(|s| s.size == shoe_size)
		 .collect()
}


#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}




#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn produce_other_iterator() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn filters_by_size() {
	let shoes = vec![
		Shoe {size: 10, style: String::from("sneaker")}, 
		Shoe {size: 13, style: String::from("sandal")},
		Shoe {size: 10, style: String::from("boot")},
		];
	
	let in_my_size = shoes_in_my_size(shoes, 10);
	
	assert_eq!(
		in_my_size,
		vec![
			Shoe {size: 10, style: String::from("sneaker")},
			Shoe {size: 10, style: String::from("boot")},
			]
		);
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a,b)| a*b)
                                 .filter(|x| x%3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}



