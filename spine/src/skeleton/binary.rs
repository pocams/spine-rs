use std::{ptr::NonNull, rc::Rc};

use spine_sys::{spSkeletonBinary, spSkeletonBinary_create, spSkeletonBinary_dispose};

use crate::atlas::Atlas;

pub struct SkeletonBinary {
    pub(crate) pointer: NonNull<spSkeletonBinary>,
    pub(crate) _atlas: Rc<Atlas>,
}

impl SkeletonBinary {
    pub fn new(atlas: &Rc<Atlas>) -> Self {
        let pointer = unsafe { spSkeletonBinary_create(atlas.pointer.as_ptr()) };

        Self {
            pointer: NonNull::new(pointer).unwrap(),
            _atlas: atlas.clone(),
        }
    }

    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        unsafe { self.pointer.as_mut().scale = scale }
        self
    }
}

impl Drop for SkeletonBinary {
    fn drop(&mut self) {
        unsafe { spSkeletonBinary_dispose(self.pointer.as_ptr()) }
    }
}
