/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod error;
pub mod reader;

use std::{
	io::{Read, Write},
	net::TcpStream,
};



use std::mem::MaybeUninit;


//#[cfg(feature = "allocator_api")]
use crate::allocator::HybridGlobal;

//#[cfg(feature = "allocator_api")]
use std::alloc::Allocator;


//need to import allocator....
pub type BufferPMEM = Box<[u8], HybridGlobal>;


pub type Buffer = Box<[u8]>;
pub type StackBuffer<const N: usize> = [u8; N];


use typesize::TypeSize;   
impl TypeSize for BufferPMEM {
    fn get_size(&self) -> usize {
        self.len()
    }
}


pub fn read_buf(stream: &mut TcpStream, buf_size: usize) -> Result<Buffer, StreamError> {
	let mut buf = vec![0u8; buf_size].into_boxed_slice();

	match stream.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}

pub fn read_buf_pmem(stream: &mut TcpStream, buf_size: usize) -> Result<BufferPMEM, StreamError> {
	//let mut buf = vec![0u8; buf_size].into_boxed_slice();


	//new_uninit_slice_in
	//pub fn new_zeroed_slice_in(len: usize, alloc: A) -> Box<[MaybeUninit<T>], A>

	//let mut buf : Box<[MaybeUninit<BufferPMEM>], HybridGlobal> = Box::new_uninit_slice_in(buf_size, HybridGlobal);

	//pub fn into_boxed_slice(boxed: Box<T, A>) -> Box<[T], A>


	//let mut buf: Vec<u8, HybridGlobal> = Vec::with_capacity_in(buf_size, HybridGlobal);
	
	//i think this moves it back to a dram allocated buffer....
	//no i think im worng as no error when building....
	let mut buf: Box<[u8], HybridGlobal>  = (Vec::with_capacity_in(buf_size, HybridGlobal)).into_boxed_slice();

	//let mut buf = buf.into_boxed_slice();

	match stream.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}

pub fn read_stack_buf<const N: usize>(stream: &mut TcpStream) -> Result<StackBuffer<N>, StreamError> {
	let mut buf = [0u8; N];

	match stream.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}


pub fn write_buf(stream: &mut TcpStream, buf: &[u8]) -> Result<(), StreamError> {
	match stream.write_all(buf) {
		Ok(_) => Ok(()),
		Err(_) => Err(StreamError::InvalidStream),
	}
}

pub use crate::stream::error::*;
pub use crate::stream::reader::*;





/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */


/*
pub mod error;
pub mod reader;

use std::{
	io::{Read, Write},
	net::TcpStream,
};





use std::mem::MaybeUninit;


#[cfg(feature = "allocator_api")]
use crate::allocator::HybridBuffer;

#[cfg(feature = "allocator_api")]
use std::alloc::Allocator;


//need to import allocator....

#[cfg(feature = "allocator_api")]
pub type Buffer = Box<[u8], HybridBuffer>;



pub type BufferPMEM = Box<[u8], HybridBuffer>;


#[cfg(not(feature = "allocator_api"))]
pub type Buffer = Box<[u8]>;
pub type StackBuffer<const N: usize> = [u8; N];

#[cfg(feature = "allocator_api")]
pub fn read_buf(stream: &mut TcpStream, buf_size: usize) -> Result<Buffer, StreamError> {
	//let mut buf = vec![0u8; buf_size].into_boxed_slice();
	//let mut buf: Box<[u8], HybridBuffer> = vec![0u8; buf_size]
    //.into_boxed_slice_in(HybridBuffer);

	let mut buf: Vec<u8, HybridBuffer> = Vec::with_capacity_in(buf_size, HybridBuffer);
    buf.resize(buf_size, 0); // initialize with zeros
	
	//i think this moves it back to a dram allocated buffer....
	//no i think im worng as no error when building....
	let mut buf = buf.into_boxed_slice();
	


	match stream.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}


#[cfg(not(feature = "allocator_api"))]
pub fn read_buf(stream: &mut TcpStream, buf_size: usize) -> Result<Buffer, StreamError> {
	let mut buf = vec![0u8; buf_size].into_boxed_slice();

	match stream.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}

pub fn read_stack_buf<const N: usize>(stream: &mut TcpStream) -> Result<StackBuffer<N>, StreamError> {
	let mut buf = [0u8; N];

	match stream.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}


pub fn write_buf(stream: &mut TcpStream, buf: &[u8]) -> Result<(), StreamError> {
	match stream.write_all(buf) {
		Ok(_) => Ok(()),
		Err(_) => Err(StreamError::InvalidStream),
	}
}

pub use crate::stream::error::*;
pub use crate::stream::reader::*;


*/