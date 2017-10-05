/**
 * File: src/lib.rs
 * Author: Anicka Burova <anicka.burova@gmail.com>
 * Date: 05.10.2017
 * Last Modified Date: 05.10.2017
 * Last Modified By: Anicka Burova <anicka.burova@gmail.com>
 */

pub fn find_exact<'a,'b>(pattern: &'a[u8], data: &'b[u8]) -> FindExact<'a,'b> {
    FindExact::create(pattern, data)
}



pub struct FindExact<'a, 'b> {
    pattern: &'a[u8],
    data: &'b[u8],
    index: usize,
}

impl<'a, 'b> FindExact<'a, 'b> {
    fn create(pattern: &'a[u8], data: &'b[u8]) -> Self {
        //println!("{}", data.tails());
        Self {
            pattern,
            data,
            index: 0,
        }
    }
}

use std::iter::Iterator;

impl<'a, 'b> Iterator for FindExact<'a, 'b> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            None
        } else {
            let pat_size = self.pattern.len();
            for i in self.index .. self.data.len() - pat_size {
                if self.data[i .. i + pat_size] == *self.pattern {
                    self.index = i+1;
                    return Some(i);
                }
            }
            None
        }
    }
}


#[test]
fn find_exact_test() {
    let pattern = vec![4,5,6];
    let data = vec![1,2,3,4,5,6,7,8,4,5,6,9,10];
    for i in find_exact(&pattern[..], &data[..]) {
        println!("{}", i);
    }
}
