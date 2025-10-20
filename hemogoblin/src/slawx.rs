//! Bindings to c slaw and bslaw

use libc::c_char;
use plasma_sys::{
    bslaw, slaw_boolean, slaw_nil, slaw_spew_overview_to_string, slaw_string_from_substring,
    slawx_equal,
};
use std::fmt::Debug;
use std::ops::Deref;
use std::ptr::NonNull;

use crate::{SlawEmit, protein};

/// A singly owned slaw value.
pub struct Slaw {
    cslaw: NonNull<plasma_sys::slaw_guts>,
}

impl Slaw {
    pub unsafe fn try_from_c_slaw(cslaw: plasma_sys::slaw) -> Option<Self> {
        let cslaw = NonNull::new(cslaw)?;
        Some(Slaw { cslaw })
    }

    pub unsafe fn from_c_slaw(cslaw: plasma_sys::slaw) -> Self {
        unsafe { Self::try_from_c_slaw(cslaw).expect("passed null to from_c_slaw_checked") }
    }

    pub fn string(s: &str) -> Self {
        unsafe {
            let byteslen: i64 = s.len().try_into().expect("impossibly huge string");
            let c_str = s.as_ptr();
            // safety: from_substring() does not expect a 0-terminated input
            Slaw::from_c_slaw(slaw_string_from_substring(c_str as *const c_char, byteslen))
        }
    }

    pub fn nil() -> Self {
        unsafe { Slaw::from_c_slaw(slaw_nil()) }
    }

    pub fn boolean(b: bool) -> Self {
        unsafe { Slaw::from_c_slaw(slaw_boolean(b)) }
    }

    pub fn as_slaw(&self) -> &slaw {
        self.as_ref()
    }
}

impl Debug for Slaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_ref(), f)
    }
}

impl Deref for Slaw {
    type Target = slaw;
    fn deref(&self) -> &slaw {
        self.as_ref()
    }
}

impl AsRef<slaw> for Slaw {
    fn as_ref(&self) -> &slaw {
        unsafe { slaw::from_bslaw(self.cslaw.as_ptr()) }
    }
}

impl Drop for Slaw {
    fn drop(&mut self) {
        unsafe { plasma_sys::slaw_free(self.cslaw.as_ptr()) }
    }
}

#[allow(non_camel_case_types)]
type oct = u64;

#[allow(non_camel_case_types)]
/// Dynamically sized type representing a borrowed slaw or subslaw. `bslaw` in libPlasma's C API is
/// represented as `&slaw` in Rust. `&slaw` is to `Slaw` as `&str` is to `String`.
pub struct slaw {
    octs: [oct],
}

impl slaw {
    /// Danger danger, you can supply whatever lifetime you want here.  Be wary!
    pub unsafe fn from_bslaw<'a>(slaw: bslaw) -> &'a slaw {
        unsafe {
            assert!(!slaw.is_null());
            let len = plasma_sys::slaw_len(slaw) as usize;
            assert_eq!(len % 8, 0);
            let octslice: &[oct] = std::slice::from_raw_parts(
                slaw as *const oct,
                plasma_sys::slaw_len(slaw) as usize / 8,
            );
            &*(octslice as *const [oct] as *const slaw)
        }
    }

    pub fn as_bslaw(&self) -> bslaw {
        &self.octs as *const [u64] as bslaw
    }

    pub fn can_emit<'a, T>(&'a self) -> bool
    where
        T: SlawEmit<'a> + 'a,
    {
        <T as SlawEmit<'a>>::can_emit(self)
    }

    pub fn try_emit<'a, T>(&'a self) -> Option<T>
    where
        T: SlawEmit<'a> + 'a,
    {
        if self.can_emit::<T>() {
            Some(<T as SlawEmit<'a>>::guarded_emit(self))
        } else {
            None
        }
    }

    pub fn emit<'a, T>(&'a self) -> T
    where
        T: SlawEmit<'a> + 'a,
    {
        match self.try_emit() {
            Some(emission) => emission,
            // FIXME: would be nice to have a SlawIlk enum which describes whether the slaw is Nil,
            // Boolean, String, etc.  It's risky to include the slaw's spew's here (it might be
            // really big, or sensitive data may lurk in the spew)
            None => panic!("failed to emit {} from slaw", std::any::type_name::<T>()),
        }
    }

    pub fn spew(&self) -> String {
        format!("{:?}", self)
    }

    pub fn is_protein(&self) -> bool {
        self.can_emit::<&protein>()
    }
}

impl Debug for slaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spew = unsafe { Slaw::try_from_c_slaw(slaw_spew_overview_to_string(self.as_bslaw())) };
        // spew will be `None` if allocating the slabu to build the spew fails
        match spew.as_ref().and_then(|s| s.try_emit::<&str>()) {
            None => write!(f, "[spew-error]"),
            Some(str) => std::fmt::Display::fmt(str, f),
        }
    }
}

// slaw equality rumpus
impl PartialEq<Slaw> for Slaw {
    fn eq(&self, other: &Slaw) -> bool {
        self.as_slaw() == other.as_slaw()
    }
}
impl PartialEq<&slaw> for Slaw {
    fn eq(&self, other: &&slaw) -> bool {
        unsafe { slawx_equal(self.as_bslaw(), other.as_bslaw()) }
    }
}
impl PartialEq<slaw> for slaw {
    fn eq(&self, other: &slaw) -> bool {
        unsafe { slawx_equal(self.as_bslaw(), other.as_bslaw()) }
    }
}
impl PartialEq<Slaw> for slaw {
    fn eq(&self, other: &Slaw) -> bool {
        self == other.as_slaw()
    }
}
impl PartialEq<Slaw> for &slaw {
    fn eq(&self, other: &Slaw) -> bool {
        *self == other.as_slaw()
    }
}

#[cfg(test)]
mod tests {
    use crate::Slaw;

    #[test]
    fn test_nil() {
        let nil = Slaw::nil();
        assert!(!nil.can_emit::<&str>());
        assert_eq!(nil.try_emit::<&str>(), None);
    }

    #[test]
    fn test_boolean() {
        let knight = Slaw::boolean(true);
        let knave = Slaw::boolean(false);
        assert!(knight.can_emit::<bool>());
        assert!(knave.can_emit::<bool>());
        assert!(!knight.can_emit::<()>());
        assert!(knight.emit::<bool>());
        assert!(!knave.emit::<bool>());
    }

    #[test]
    fn test_string() {
        let msg = "hello world!";
        let hello = Slaw::string(msg);
        assert!(hello.can_emit::<&str>());
        assert!(!hello.can_emit::<()>());
        assert_eq!(hello.emit::<&str>(), msg);
    }

    #[test]
    fn test_spew() {
        let nil = Slaw::nil();
        let should_spew = format!("slaw[1o.0x{:x}]: NIL.", nil.as_bslaw() as usize);
        assert_eq!(nil.spew(), should_spew);
        assert_eq!(format!("{:?}", nil), should_spew);
    }

    #[test]
    fn test_equality() {
        let nil = Slaw::nil();
        let hello = Slaw::string("hello");
        let hello2 = Slaw::string("hello");
        let goodbye = Slaw::string("goodbye");
        assert_eq!(nil, nil);
        assert_eq!(hello, hello2);
        assert_ne!(nil, hello);
        assert_ne!(goodbye, hello);
    }

    #[test]
    #[should_panic(expected = "failed to emit bool from slaw")]
    fn test_bad_emission_panics() {
        let nil = Slaw::nil();
        let _ = nil.emit::<bool>();
    }
}
