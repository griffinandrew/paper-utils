/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

/* 
use smallvec::SmallVec;
use crate::sheet::Sheet;

#[cfg(not(feature = "allocator_api"))]
pub struct SheetBuilder {
	data: SmallVec<[u8; 3]>,
}


/* 
#[cfg(feature = "allocator_api")]
pub struct SheetBuilder {
	data: SmallVec<[u8; 3]>,
}
*/



#[cfg(feature = "allocator_api")]
pub enum SheetData<'a> {
    Owned(Box<[u8]>),
    Borrowed(&'a [u8]),
}

#[cfg(feature = "allocator_api")]
pub struct Sheet<'a> {
    pub header: Box<[u8]>,
    pub payload: Option<SheetData<'a>>,
}


#[cfg(feature = "allocator_api")]
impl<'a> Sheet<'a> {
    pub fn with_borrowed_payload(header: Box<[u8]>, payload: &'a [u8]) -> Self {
        Sheet {
            header,
            payload: Some(SheetData::Borrowed(payload)),
        }
    }
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

	#[cfg(not(feature = "allocator_api"))]
	pub fn into_sheet(self) -> Sheet {
		Sheet::new(self.data.into_boxed_slice())
	}

	#[cfg(feature = "allocator_api")]
	pub fn into_sheet(self) -> Sheet {
		Sheet::new(self.data.into_boxed_slice())
	}
}

impl Default for SheetBuilder {
	fn default() -> Self {
		SheetBuilder::new()
	}
}


*/








/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use smallvec::SmallVec;

#[cfg(feature = "allocator_api")]
pub enum SheetData<'a> {
    Owned(Box<[u8]>),
    Borrowed(&'a [u8]),
}

#[cfg(feature = "allocator_api")]
pub struct Sheet<'a> {
    pub header: Box<[u8]>,
    pub payload: Option<SheetData<'a>>,
}

#[cfg(feature = "allocator_api")]
impl<'a> Sheet<'a> {
    /// Create a Sheet with only an owned header and no payload
    pub fn new(header: Box<[u8]>) -> Self {
        Sheet {
            header,
            payload: None,
        }
    }

    /// Create a Sheet with a borrowed payload (zero-copy)
    pub fn with_borrowed_payload(header: Box<[u8]>, payload: &'a [u8]) -> Self {
        Sheet {
            header,
            payload: Some(SheetData::Borrowed(payload)),
        }
    }

    /// Create a Sheet with an owned payload
    pub fn with_owned_payload(header: Box<[u8]>, payload: Box<[u8]>) -> Self {
        Sheet {
            header,
            payload: Some(SheetData::Owned(payload)),
        }
    }
	pub fn write_to<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        w.write_all(&self.header)?;
        if let Some(payload) = &self.payload {
            match payload {
                SheetData::Owned(b) => w.write_all(b)?,
                SheetData::Borrowed(s) => w.write_all(s)?,
            }
        }
        Ok(())
    }
}

pub struct SheetBuilder<'a> {
    data: SmallVec<[u8; 3]>,
    borrowed_payload: Option<&'a [u8]>,
}


impl<'a> SheetBuilder<'a> {
    pub fn new() -> Self {
        SheetBuilder {
            data: SmallVec::new(),
            borrowed_payload: None,
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

    /// Mark a borrowed payload to be included in the Sheet
    pub fn borrow_payload(mut self, payload: &'a [u8]) -> Self {
        self.borrowed_payload = Some(payload);
        self
    }

	#[cfg(not(feature = "allocator_api"))]
	pub fn into_sheet(self) -> Sheet {
		Sheet::new(self.data.into_boxed_slice())
	}

	#[cfg(feature = "allocator_api")]
    pub fn into_sheet_borrowed(self, payload: &'a [u8]) -> Sheet<'a> {
        Sheet::with_borrowed_payload(self.data.into_boxed_slice(), payload)
    }


	#[cfg(feature = "allocator_api")]
    pub fn into_sheet_owned(self) -> Sheet<'static> {
        Sheet::with_owned_payload(self.data.into_boxed_slice(), Box::new([]))
    }

}

impl<'a> Default for SheetBuilder<'a> {
    fn default() -> Self {
        SheetBuilder::new()
    }
}
