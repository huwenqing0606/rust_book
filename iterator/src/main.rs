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



