// Copyright 2016 The pliOS Developers. See the LICENSE
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>. This file may not
// be copied, modified, or distributed except according
// to these terms.

//! Various utitlites

/// Round up integer to power of 2
pub fn round_up_2(value: usize, round_to: usize) -> usize {
    (value + round_to - 1) & !(round_to - 1)
}

/// Round down integer to power of 2
pub fn round_down_2(value: usize, round_to: usize) -> usize {
    value & !(round_to - 1)
}
