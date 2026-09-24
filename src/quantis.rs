#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, c_void};
use thiserror::Error;

pub struct Quantis {
    handle: *mut QuantisDeviceHandle,
}

impl Quantis {
    /// establish a Quantis device connection
    pub fn create() -> Result<Quantis, QuantisErrors> {
        let cnt = unsafe { QuantisCount(QuantisDeviceType_QUANTIS_DEVICE_USB) };
        if cnt < 1 {
            return Err(QuantisErrors::NoDevice);
        }

        let mut handle: *mut QuantisDeviceHandle = std::ptr::null_mut();
        let res = unsafe {
            QuantisOpen(
                QuantisDeviceType_QUANTIS_DEVICE_USB,
                0, // always use the first device, as we don't expect to have more
                &mut handle,
            )
        };
        QuantisErrors::test(Quantis { handle }, res)
    }

    /// terminate the Quantis device connection
    fn kill(&mut self) {
        unsafe { QuantisClose(self.handle) }
    }

    /// Returns the version of the library as a number composed of a major and a minor version number.
    /// The value before the point represents the major version number, while the value after the point represents
    /// the minor version number.
    pub fn GetLibVersion() -> f32 {
        unsafe { QuantisGetLibVersion() }
    }

    pub fn GetSerialNumber(&self) -> Result<String, QuantisErrors> {
        let fun = unsafe { (*(*self.handle).ops).GetSerialNumber }
            .ok_or(QuantisErrors::FunctionNotFound)?;
        let ser = unsafe {
            let ser = fun(self.handle);
            CStr::from_ptr(ser as *const _).to_str()
        }?;

        Ok(ser.to_string())
    }

    pub fn GetModulesStatus(&self) -> Result<[bool; 4], QuantisErrors> {
        let fun = unsafe { (*(*self.handle).ops).GetModulesStatus }
            .ok_or(QuantisErrors::FunctionNotFound)?;
        let mstat = unsafe { fun(self.handle) };

        let mstat = (0..4)
            .map(|i| (1 << i) & mstat != 0)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| QuantisErrors::ConversionError)?;

        Ok(mstat)
    }

    pub fn Read(&self, buf: &mut [u8]) -> Result<(), QuantisErrors> {
        let fun = unsafe { (*(*self.handle).ops).Read }.ok_or(QuantisErrors::FunctionNotFound)?;
        let res = unsafe { fun(self.handle, buf as *mut _ as *mut c_void, buf.len()) };
        QuantisErrors::test((), res)
    }
}

impl Drop for Quantis {
    fn drop(&mut self) {
        self.kill();
    }
}

#[derive(Error, Debug)]
pub enum QuantisErrors {
    #[error("Invalid device number (out of bounds)")]
    InvalidDevicNumber,
    #[error("Invalid parameter")]
    InvalidParameter,
    #[error("Invalid size to reaad (too high)")]
    InvalidReadSize,
    #[error("Module status error")]
    InvalidStatus,
    #[error("Input/output error")]
    ErrorIo,
    #[error("No such device (it may have been disconnected)")]
    NoDevice,
    #[error("Invalid driver")]
    NoDriver,
    #[error("Insufficient memory")]
    NoMemory,
    #[error("No module found or no module active")]
    NoModule,
    #[error("Operation not supported or unimplemented")]
    OperationNotSupported,
    #[error("Function not found")]
    FunctionNotFound,
    #[error("Other error")]
    ErrorOther,
    #[error("Unknown error")]
    UnknownError(QuantisError),
    #[error("Error converting the data")]
    ConversionError,
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
}

impl QuantisErrors {
    /// convert the c_int returned from Quantis functions into QuantisError, or pass on the value if successful.
    pub fn test<T>(payload: T, err: QuantisError) -> Result<T, QuantisErrors> {
        // positive values can represent the bytes read.
        if err > 0 {
            return Ok(payload);
        }
        match err {
            QuantisError_QUANTIS_SUCCESS => Ok(payload),
            QuantisError_QUANTIS_ERROR_INVALID_DEVICE_NUMBER => {
                Err(QuantisErrors::InvalidDevicNumber)
            }
            QuantisError_QUANTIS_ERROR_INVALID_PARAMETER => Err(QuantisErrors::InvalidParameter),
            QuantisError_QUANTIS_ERROR_INVALID_READ_SIZE => Err(QuantisErrors::InvalidReadSize),
            QuantisError_QUANTIS_ERROR_INVALID_STATUS => Err(QuantisErrors::InvalidStatus),
            QuantisError_QUANTIS_ERROR_IO => Err(QuantisErrors::ErrorIo),
            QuantisError_QUANTIS_ERROR_NO_DEVICE => Err(QuantisErrors::NoDevice),
            QuantisError_QUANTIS_ERROR_NO_DRIVER => Err(QuantisErrors::NoDriver),
            QuantisError_QUANTIS_ERROR_NO_MEMORY => Err(QuantisErrors::NoMemory),
            QuantisError_QUANTIS_ERROR_NO_MODULE => Err(QuantisErrors::NoModule),
            QuantisError_QUANTIS_ERROR_OPERATION_NOT_SUPPORTED => {
                Err(QuantisErrors::OperationNotSupported)
            }
            QuantisError_QUANTIS_ERROR_OTHER => Err(QuantisErrors::ErrorOther),
            _ => Err(QuantisErrors::UnknownError(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantis_get_lib_version() {
        let vers = Quantis::GetLibVersion();
        assert_eq!(vers, 20.2);
    }

    #[test]
    #[should_panic(expected = "NoDevice")]
    fn test_quantis_serial() {
        let idq = Quantis::create().unwrap();
        let _ser = idq.GetSerialNumber().unwrap();
    }

    #[test]
    #[should_panic(expected = "NoDevice")]
    fn test_quantis_buf() {
        let idq = Quantis::create().unwrap();
        let mut buf = [0_u8; 32];
        idq.Read(&mut buf).unwrap();
    }
}
