//! This crate adds convenient sort functions for Iterators.
//!
//! # Example
//! ```
//! use sortby::*;
//!
//! #[derive(Clone, Debug, Eq, PartialEq)]
//! struct Person {
//!   pub age: i32,
//!   pub name: &'static str,
//! }
//!
//! fn main() {
//!   let data = vec![
//!     Person {
//!       name: "Rich",
//!       age: 18,
//!     },
//!     Person {
//!       name: "Bob",
//!       age: 9,
//!     },
//!     Person {
//!       name: "Marc",
//!       age: 21,
//!     },
//!     Person {
//!       name: "Alice",
//!       age: 18,
//!     },
//!   ];
//!
//!   let sorted: Vec<_> = data.iter()
//!     .sort_by(|p| p.age)
//!     .then_sort_by(|p| p.name)
//!     .collect();
//!
//!    println!("{:#?}", sorted);
//! }
#![warn(rust_2018_idioms)]

use std::cmp::Ordering;

enum IterState<I: Iterator> {
    Unsorted(Option<I>),
    Sorted(std::vec::IntoIter<I::Item>),
}

impl<I: Iterator> IterState<I> {
    fn unwrap_sorted(&mut self) -> &mut std::vec::IntoIter<I::Item> {
        match self {
            IterState::Unsorted(_) => panic!("unsorted"),
            IterState::Sorted(ref mut iter) => iter,
        }
    }
}

pub struct SortBy<'a, I: Iterator> {
    iter: IterState<I>,
    compare: Box<dyn Fn(&I::Item, &I::Item) -> Ordering + 'a>,
}

impl<'a, I> SortBy<'a, I>
where
    I: Iterator,
{
    pub fn then_sort_by<G, U>(self, f: G) -> SortBy<'a, I>
    where
        U: PartialOrd,
        G: Fn(&I::Item) -> U + 'a,
        Self: Sized,
        <I as std::iter::Iterator>::Item: 'a,
    {
        let prev = self.compare;
        SortBy {
            iter: self.iter,
            compare: Box::new(move |a, b| match (prev)(a, b) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => f(a).partial_cmp(&f(b)).unwrap_or(Ordering::Equal),
            }),
        }
    }
}

impl<'a, I> Iterator for SortBy<'a, I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter {
            IterState::Unsorted(ref mut iter) => {
                let mut vec: Vec<_> = iter.take().unwrap().collect();
                vec.sort_by(|a, b| (self.compare)(a, b));
                self.iter = IterState::Sorted(vec.into_iter());
                self.iter.unwrap_sorted().next()
            }
            IterState::Sorted(ref mut iter) => iter.next(),
        }
    }
}

pub trait Itertools: Iterator {
    fn sort_by<'a, F, V>(self, f: F) -> SortBy<'a, Self>
    where
        V: PartialOrd,
        F: Fn(&Self::Item) -> V + 'a,
        Self: Sized,
    {
        SortBy {
            iter: IterState::Unsorted(Some(self)),
            compare: Box::new(move |a, b| f(a).partial_cmp(&f(b)).unwrap_or(Ordering::Equal)),
        }
    }
}

impl<T: ?Sized> Itertools for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Person {
        pub age: i32,
        pub name: &'static str,
    }

    #[test]
    fn sorts_floats() {
        let input = vec![5.0, 1.0, 2.0];
        let actual: Vec<_> = input.iter().sort_by(|v| *v).map(|v| *v).collect();

        assert_equal(actual, vec![1.0, 2.0, 5.0]);
    }

    #[test]
    fn it_works() {
        let data = vec![
            Person {
                name: "Rich",
                age: 18,
            },
            Person {
                name: "Bob",
                age: 9,
            },
            Person {
                name: "Marc",
                age: 21,
            },
            Person {
                name: "Alice",
                age: 18,
            },
        ];

        let expected = vec![
            data[1].clone(), // 9, Bob
            data[3].clone(), // 18, Alice
            data[0].clone(), // 18, Rich
            data[2].clone(), // 21, Marc
        ];

        let actual = data.into_iter().sort_by(|v| v.age).then_sort_by(|v| v.name);

        assert_equal(actual, expected);
    }

    fn assert_equal<I, J>(a: I, b: J)
    where
        I: IntoIterator,
        J: IntoIterator,
        I::Item: std::fmt::Debug + PartialEq<J::Item>,
        J::Item: std::fmt::Debug,
    {
        let mut ia = a.into_iter();
        let mut ib = b.into_iter();
        let mut i = 0;
        loop {
            match (ia.next(), ib.next()) {
                (None, None) => return,
                (a, b) => {
                    let equal = match (&a, &b) {
                        (&Some(ref a), &Some(ref b)) => a == b,
                        _ => false,
                    };
                    assert!(
                        equal,
                        "Failed assertion {a:?} == {b:?} for iteration {i}",
                        i = i,
                        a = a,
                        b = b
                    );
                    i += 1;
                }
            }
        }
    }
}
