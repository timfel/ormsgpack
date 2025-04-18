// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::ffi::PyBytes_AS_STRING;
use crate::ffi::*;

use serde::ser::{Serialize, Serializer};

#[repr(transparent)]
pub struct Bytes {
    ptr: *mut pyo3::ffi::PyObject,
}

impl Bytes {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        Bytes { ptr: ptr }
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let buffer = unsafe { PyBytes_AS_STRING(self.ptr) as *const u8 };
        let length = unsafe { PyBytes_GET_SIZE(self.ptr) as usize };
        let contents = unsafe { std::slice::from_raw_parts(buffer, length) };
        serializer.serialize_bytes(contents)
    }
}
