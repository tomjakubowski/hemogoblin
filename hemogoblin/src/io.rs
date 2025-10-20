use std::{ffi::CString, path::Path};

use plasma_sys::{slaw_input, slaw_input_close, slaw_input_open, slaw_input_read};

use crate::loam::SLAW_END_OF_FILE;
use crate::{
    Slaw,
    loam::{ObResult, ObRetort},
};

pub struct SlawInput {
    input: slaw_input,
}

impl Drop for SlawInput {
    fn drop(&mut self) {
        unsafe {
            slaw_input_close(self.input);
        }
    }
}

impl SlawInput {
    pub fn open_file<P>(p: P) -> ObResult<SlawInput>
    where
        P: AsRef<Path>,
    {
        use std::os::unix::ffi::OsStrExt;
        unsafe {
            let bytes = p.as_ref().as_os_str().as_bytes();
            let c_str = CString::new(bytes).map_err(|_| ObRetort::InvalidArgument)?;
            let mut input = std::mem::MaybeUninit::uninit();
            let tort = slaw_input_open(c_str.as_ptr(), input.as_mut_ptr());
            ObRetort::map(tort, |_| {
                let input = input.assume_init();
                SlawInput { input }
            })
        }
    }
}

impl IntoIterator for SlawInput {
    type Item = ObResult<Slaw>;
    type IntoIter = SlawInputIterator;

    fn into_iter(self) -> SlawInputIterator {
        SlawInputIterator { input: self }
    }
}
pub struct SlawInputIterator {
    input: SlawInput,
}

impl Iterator for SlawInputIterator {
    type Item = ObResult<Slaw>;
    fn next(&mut self) -> Option<ObResult<Slaw>> {
        unsafe {
            let mut slaw = std::mem::MaybeUninit::uninit();
            let tort = slaw_input_read(self.input.input, slaw.as_mut_ptr());
            if tort == SLAW_END_OF_FILE {
                return None;
            }
            Some(ObRetort::map(tort, |_| {
                Slaw::from_c_slaw(slaw.assume_init())
            }))
        }
    }
}
