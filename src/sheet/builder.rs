/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use smallvec::SmallVec;
use crate::sheet::Sheet;

#[cfg(not(feature = "allocator_api"))]
pub struct SheetBuilder {
	data: SmallVec<[u8; 3]>,
}

#[cfg(feature = "allocator_api")]
pub struct SheetBuilder {
	data: SmallVec<[u8; 3]>,
}


impl SheetBuilder {
	pub fn new() -> Self {
		SheetBuilder {
			data: SmallVec::new(),
		}
	}

	pub fn write_bool(mut self, value: bool) -> Self {
		let byte = if value { b"!" } else { b"?" };
		self.data.extend_from_slice(byte);
		self
	}

	pub fn write_u8(mut self, value: u8) -> Self {
		self.data.push(value);
		self
	}

	pub fn write_u16(mut self, value: u16) -> Self {
		self.data.extend_from_slice(&value.to_le_bytes());
		self
	}

	pub fn write_u32(mut self, value: u32) -> Self {
		self.data.extend_from_slice(&value.to_le_bytes());
		self
	}

	pub fn write_u64(mut self, value: u64) -> Self {
		self.data.extend_from_slice(&value.to_le_bytes());
		self
	}

	pub fn write_f32(mut self, value: f32) -> Self {
		self.data.extend_from_slice(&value.to_le_bytes());
		self
	}

	pub fn write_f64(mut self, value: f64) -> Self {
		self.data.extend_from_slice(&value.to_le_bytes());
		self
	}

	pub fn write_buf(mut self, value: &[u8]) -> Self {
		self = self.write_u32(value.len() as u32);
		self.data.extend_from_slice(value);
		self
	}

	pub fn write_str<T>(self, value: T) -> Self
	where
		T: AsRef<str>,
	{
		self.write_buf(value.as_ref().as_bytes())
	}

	pub fn into_sheet(self) -> Sheet {
		Sheet::new(self.data.into_boxed_slice())
	}
}

impl Default for SheetBuilder {
	fn default() -> Self {
		SheetBuilder::new()
	}
}
