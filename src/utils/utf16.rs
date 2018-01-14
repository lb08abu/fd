// Copyright (c) 2017 fd developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::iter::{Peekable, Rev};

#[derive(Debug)]
pub struct ReverseUtf16Iterator<'a, I>
where
    I: 'a + DoubleEndedIterator<Item = &'a u16>,
{
    it: Peekable<Rev<I>>,
    nx: Option<u16>,
}

impl<'a, I> ReverseUtf16Iterator<'a, I>
where
    I: DoubleEndedIterator<Item = &'a u16>,
{
    pub fn new(it: I) -> Self {
        ReverseUtf16Iterator {
            it: it.rev().peekable(),
            nx: None,
        }
    }
}

impl<'a, I> Iterator for ReverseUtf16Iterator<'a, I>
where
    I: DoubleEndedIterator<Item = &'a u16>,
{
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.nx {
            self.nx = None;
            Some(next)
        } else {
            if let Some(chr) = self.it.next() {
                if let Some(next_chr) = self.it.next() {
                    if (next_chr >= &&0xD800u16) && (next_chr <= &&0xDFFFu16) {
                        self.nx = Some(*chr);
                        Some(*next_chr)
                    } else {
                        self.nx = Some(*next_chr);
                        Some(*chr)
                    }
                } else {
                    Some(*chr)
                }
            } else {
                None
            }
        }
    }
}

pub fn reverse_iter<'a, I>(it: I) -> ReverseUtf16Iterator<'a, I>
where
    I: DoubleEndedIterator<Item = &'a u16>,
{
    ReverseUtf16Iterator::new(it)
}
