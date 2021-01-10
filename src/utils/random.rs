//! ## Random
//!
//! `random` is the module which provides utilities for rand

/*
*
*   Copyright (C) 2020-2021Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

// Deps
extern crate rand;
// Ext
use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// ## random_alphanumeric_with_len
///
/// Generate a random alphanumeric string with provided length
pub fn random_alphanumeric_with_len(len: usize) -> String {
    let mut rng = thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_utils_random_alphanumeric_with_len() {
        assert_eq!(random_alphanumeric_with_len(256).len(), 256);
    }
}
