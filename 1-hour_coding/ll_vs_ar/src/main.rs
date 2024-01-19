use std::time::Instant;
use std::collections::LinkedList;
use rand::prelude::*;

fn fill_vec(vec: &mut Vec<i32>, count: u64) {
    for i in 0..count {
        vec.push(i as i32);
    }
}

fn fill_ll(ll: &mut LinkedList<i32>, count: u64) {
    for i in 0..count {
        ll.push_back(i as i32);
    }
}

fn random_insert_vec(vec: &mut Vec<i32>, count: u64) {
    for _ in 0..count {
        let idx = rand::thread_rng().gen_range(0..vec.len()) as usize;
        vec.insert(idx, -1 as i32);
    }
}

fn random_insert_ll(ll: &mut LinkedList<i32>, count: u64) {
    for _ in 0..count {
        let idx = rand::thread_rng().gen_range(0..ll.len()) as usize;
        let mut tail = ll.split_off(idx);
        ll.push_back(-1 as i32);
        ll.append(&mut tail);
    }
}

fn test_vec() {
    let mut vec_1: Vec<i32> = Vec::new();
    let mut vec_10: Vec<i32> = Vec::new();
    let mut vec_100: Vec<i32> = Vec::new();

    let start = Instant::now();
    fill_vec(&mut vec_1, 1_000_000);
    let duration = start.elapsed();
    println!("Took {:?} to fill Vector with 1 Million elements", duration);

    let start = Instant::now();
    fill_vec(&mut vec_10, 10_000_000);
    let duration = start.elapsed();
    println!("Took {:?} to fill Vector with 10 Million elements", duration);

    let start = Instant::now();
    fill_vec(&mut vec_100, 100_000_000);
    let duration = start.elapsed();
    println!("Took {:?} to fill Vector with 100 Million elements", duration);

    let start = Instant::now();
    random_insert_vec(&mut vec_1, 10_000);
    let duration = start.elapsed();
    println!("Took {:?} to insert 10 Thousand elements at random indicies into a Vector with 1 Million existing elements", duration);

    let start = Instant::now();
    random_insert_vec(&mut vec_10, 10_000);
    let duration = start.elapsed();
    println!("Took {:?} to insert 10 Thousand elements at random indicies into a Vector with 10 Million existing elements", duration);

    let start = Instant::now();
    random_insert_vec(&mut vec_100, 10_000);
    let duration = start.elapsed();
    println!("Took {:?} to insert 10 Thousand elements at random indicies into a Vector with 100 Million existing elements", duration);

    drop(vec_1);
    drop(vec_10);
    drop(vec_100);

}


fn test_ll()    {
    let mut ll_1: LinkedList<i32> = LinkedList::new();
    let mut ll_10: LinkedList<i32> = LinkedList::new();
    let mut ll_100: LinkedList<i32> = LinkedList::new();

    let start = Instant::now();
    fill_ll(&mut ll_1, 1_000_000);
    let duration = start.elapsed();
    println!("Took {:?} to fill LinkedList with 1 Million elements", duration);

    let start = Instant::now();
    fill_ll(&mut ll_10, 10_000_000);
    let duration = start.elapsed();
    println!("Took {:?} to fill LinkedList with 10 Million elements", duration);

    let start = Instant::now();
    fill_ll(&mut ll_100, 100_000_000);
    let duration = start.elapsed();
    println!("Took {:?} to fill LinkedList with 100 Million elements", duration);

    let start = Instant::now();
    random_insert_ll(&mut ll_1, 10_000);
    let duration = start.elapsed();
    println!("Took {:?} to insert 10 Thousand elements at random indicies into a LinkedList with 1 Million existing elements", duration);

    let start = Instant::now();
    random_insert_ll(&mut ll_10, 10_000);
    let duration = start.elapsed();
    println!("Took {:?} to insert 10 Thousand elements at random indicies into a LinkedList with 10 Million existing elements", duration);

    let start = Instant::now();
    random_insert_ll(&mut ll_100, 10_000);
    let duration = start.elapsed();
    println!("Took {:?} to insert 10 Thousand elements at random indicies into a LinkedList with 100 Million existing elements", duration);

    drop(ll_1);
    drop(ll_10);
    drop(ll_100);

}

fn main() {
    test_vec();
    print!("---------------------------------------------------\n");
    test_ll();
}
