/**
 * File: src/find_like.rs
 * Author: Anicka Burova <anicka.burova@gmail.com>
 * Date: 05.10.2017
 * Last Modified Date: 05.10.2017
 * Last Modified By: Anicka Burova <anicka.burova@gmail.com>
 */

/// Find like results.
#[derive(Debug, PartialEq)]
pub enum Match {
    /// Exact match at the position.
    Exact,
    /// Partial match at the position, number of same hits and average deviation.
    Partial(usize, u8),
}

pub trait FindLike {
    fn find_like(&self, pattern: &[u8]) -> Option<Match>;
}

impl FindLike for Vec<u8> {
    fn find_like(&self, pattern: &[u8]) -> Option<Match> {
        if self.len() < pattern.len() {
            None
        } else {
            let mut hits = 0;
            let mut miss = 0;
            let mut deviation: i32 = 0;
            for (a,b) in self.iter().zip(pattern) {
                if *a == *b {
                    hits += 1;
                } else {
                    miss += 1;
                    let a = *a as i32;
                    let b = *b as i32;
                    deviation += (a-b).abs();
                }
            }
            if miss == 0 {
                Some(Match::Exact)
            } else if hits == 0 {
                None
            } else {
                Some(Match::Partial(hits, (deviation / miss) as u8))
            }
        }
    }
}


impl<'a> FindLike for &'a[u8] {
    fn find_like(&self, pattern: &[u8]) -> Option<Match> {
        if self.len() < pattern.len() {
            None
        } else {
            let mut hits = 0;
            let mut miss = 0;
            let mut deviation: i32 = 0;
            for (a,b) in self.iter().zip(pattern) {
                if *a == *b {
                    hits += 1;
                } else {
                    miss += 1;
                    let a = *a as i32;
                    let b = *b as i32;
                    deviation += (a-b).abs();
                }
            }
            if miss == 0 {
                Some(Match::Exact)
            } else if hits == 0 {
                None
            } else {
                Some(Match::Partial(hits, (deviation / miss) as u8))
            }
        }
    }
}

#[test]
fn vec_find_like_vec() {
    use find_like::FindLike;
    use rustkell::DataList;
    let data = vec![1u8,2,3,4,5,6,3,4,5,6,8,9,2,3,4,5,2,2,4,5];
    let pattern = vec![2u8,3,4,5];
    let res = vec![
        None,
        Some(Match::Exact),
        None,
        None,
        None,
        Some(Match::Partial(3, 4)),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Match::Exact),
        None,
        None,
        None,
        Some(Match::Partial(3, 1)),
        None,
        None,
        None,
        None,
    ];
    for (a,b) in data.tails().map(|t| t.find_like(&pattern[..])).zip(res) {
        assert_eq!(a,b);
    }
}
