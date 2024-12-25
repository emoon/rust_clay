use core::ffi::c_void;

use crate::{bindings::*, id::Id, math::Dimensions, TypedConfig};

use super::ElementConfigType;

#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub data: *mut c_void,
    pub source_dimensions: Dimensions,
}

impl Image {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data<Data>(&mut self, data: &mut Data) -> &mut Self {
        self.data = data as *mut _ as *mut c_void;
        self
    }

    pub fn source_dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.source_dimensions = dimensions;
        self
    }

    pub fn end(&self, id: Id) -> TypedConfig {
        let memory = unsafe { Clay__StoreImageElementConfig((*self).into()) };

        TypedConfig {
            config_memory: memory as _,
            id: id.into(),
            config_type: ElementConfigType::Image as _,
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            data: core::ptr::null_mut(),
            source_dimensions: Dimensions::default(),
        }
    }
}

impl From<Clay_ImageElementConfig> for Image {
    fn from(value: Clay_ImageElementConfig) -> Self {
        Self {
            data: value.imageData,
            source_dimensions: value.sourceDimensions.into(),
        }
    }
}
impl From<Image> for Clay_ImageElementConfig {
    fn from(value: Image) -> Self {
        Self {
            imageData: value.data,
            sourceDimensions: value.source_dimensions.into(),
        }
    }
}
