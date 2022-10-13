use std::{path::Path, ptr::NonNull, rc::Rc};
use std::ffi::CStr;

use spine_sys::{spSkeletonData, spSkeletonData_dispose, spSkeletonJson_readSkeletonDataFile, spSkeletonBinary_readSkeletonDataFile};

use crate::{
    error::{Error, NullPointerError},
    result::Result,
    util,
};

use super::json::SkeletonJson;
use super::binary::SkeletonBinary;

pub(crate) enum SkeletonLoader {
    Json(SkeletonJson),
    Binary(SkeletonBinary)
}

pub struct SkeletonData {
    pub(crate) pointer: NonNull<spSkeletonData>,
    pub(crate) _skeleton_loader: SkeletonLoader,
}

impl SkeletonData {
    pub fn from_json_file(path: impl AsRef<Path>, skeleton_json: SkeletonJson) -> Result<Rc<Self>> {
        let path = util::c_path(path)?;

        let pointer = unsafe {
            spSkeletonJson_readSkeletonDataFile(skeleton_json.pointer.as_ptr(), path.as_ptr())
        };

        Ok(Rc::new(SkeletonData {
            pointer: NonNull::new(pointer).ok_or(Error::invalid_data(NullPointerError))?,
            _skeleton_loader: SkeletonLoader::Json(skeleton_json),
        }))
    }

    pub fn from_binary_file(path: impl AsRef<Path>, skeleton_binary: SkeletonBinary) -> Result<Rc<Self>> {
        let path = util::c_path(path)?;

        let pointer = unsafe {
            spSkeletonBinary_readSkeletonDataFile(skeleton_binary.pointer.as_ptr(), path.as_ptr())
        };

        Ok(Rc::new(SkeletonData {
            pointer: NonNull::new(pointer).ok_or(Error::invalid_data(NullPointerError))?,
            _skeleton_loader: SkeletonLoader::Binary(skeleton_binary),
        }))
    }
}

impl Drop for SkeletonData {
    fn drop(&mut self) {
        unsafe { spSkeletonData_dispose(self.pointer.as_ptr()) }
    }
}
