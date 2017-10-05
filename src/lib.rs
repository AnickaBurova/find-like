/**
 * File: src/lib.rs
 * Author: Anicka Burova <anicka.burova@gmail.com>
 * Date: 05.10.2017
 * Last Modified Date: 05.10.2017
 * Last Modified By: Anicka Burova <anicka.burova@gmail.com>
 */
extern crate rustkell;

pub fn find_exact<'a,'b>(pattern: &'a[u8], data: &'b[u8]) -> FindExact<'a,'b> {
    FindExact::create(pattern, data)
}



pub struct FindExact<'a, 'b> {
    pattern: &'a[u8],
    data: &'b[u8],
}

impl<'a, 'b> FindExact<'a, 'b> {
    fn create(pattern: &'a[u8], data: &'b[u8]) -> Self {
        Self {
            pattern,
            data,
        }
    }
}
