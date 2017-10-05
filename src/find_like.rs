/**
 * File: src/find_like.rs
 * Author: Anicka Burova <anicka.burova@gmail.com>
 * Date: 05.10.2017
 * Last Modified Date: 05.10.2017
 * Last Modified By: Anicka Burova <anicka.burova@gmail.com>
 */

/// Find like results.
pub enum Match {
    /// Exact match at the position.
    Exact(usize),
    /// Partial match at the position, number of same hits and average deviation.
    Partial(usize, usize, u8),
}

pub fn find_like<'a,'b>(pattern: &'a[u8], data: &'b[u8]) -> FindLike<'a,'b> {
    FindLike::create(pattern, data)
}


pub struct FindLike<'a, 'b> {
    pattern: &'a[u8],
    data: &'b[u8],
    index: usize,
}

impl<'a, 'b> FindLike<'a, 'b> {
    fn create(pattern: &'a[u8], data: &'b[u8]) -> Self {
        //println!("{}", data.tails());
        Self {
            pattern,
            data,
            index: 0,
        }
    }
}

impl<'a, 'b> Iterator for FindLike<'a, 'b> {
    type Item = Match;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            None
        } else {
            let pat_size = self.pattern.len();
            for i in self.index .. self.data.len() - pat_size {
                let mut hits = 0;
                let mut miss = 0;
                let mut deviation: usize = 0;
                for (a,b) in self.data[i..i + pat_size].iter().zip(self.pattern) {
                    if a == b {
                        hits += 1;
                    } else {
                        miss += 1;
                        deviation += abs(:w

                    }
                }

                if self.data[i .. i + pat_size] == *self.pattern {
                    self.index = i+1;
                    return Some(Match::Exact(i));
                }
            }
            None
        }
    }
}
