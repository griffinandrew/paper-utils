/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */


#![feature(allocator_api)]

//#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

//#[cfg(feature = "allocator_api")]
pub mod allocator;
//#[cfg(feature = "allocator_api")]

   

pub use crate::allocator::HybridGlobal;


pub mod stream;
pub mod sheet;

pub mod command;

