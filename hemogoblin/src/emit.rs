use plasma_sys::{
    slaw_boolean_emit, slaw_is_boolean, slaw_is_nil, slaw_is_protein, slaw_is_string,
    slaw_string_emit, slaw_string_emit_length, slaw_string_is_valid_utf8,
};

use crate::{protein, slaw};

/// Types which can be emitted from a Slaw.
/// You probably want to call the more ergonomic methods on slaw:
/// * `slaw::can_emit::<T>()`
/// * `slaw::try_emit::<T>()`
/// * `slaw::emit::<T>()`
/// Rather than call this trait's methods directly.
pub trait SlawEmit<'a>: 'a {
    fn can_emit(slaw: &'a slaw) -> bool;
    /// Impls should panic in `guarded_emit(slaw)` iff. `can_emit::<Self>(slaw)` returns false
    fn guarded_emit(slaw: &'a slaw) -> Self;
}

macro_rules! guard {
    ($bslaw: expr) => {
        assert!(<Self as SlawEmit>::can_emit(slaw::from_bslaw($bslaw)))
    };
}

impl<'a> SlawEmit<'a> for &'a str {
    fn can_emit(slaw: &slaw) -> bool {
        // FIXME: fix the need to cast to *mut _ upstream
        // (https://github.com/plasma-hamper/plasma/pull/19).

        // Arguably it is too expensive to call slaw_string_is_valid_utf8 here, but it's More
        // Correct (tm) since we cannot emit a &str unless the contents are assuredly utf8. The
        // alternative is to panic (as below). This may be a sign the design of this trait can be
        // improved.
        unsafe { slaw_string_is_valid_utf8(slaw.as_bslaw() as *mut _) }
    }
    fn guarded_emit(slaw: &'a slaw) -> &'a str {
        let bslaw = slaw.as_bslaw();
        unsafe {
            guard!(bslaw);
            let cstr = slaw_string_emit(slaw.as_bslaw()) as *const u8;
            let cstr_len = slaw_string_emit_length(slaw.as_bslaw());
            assert!(cstr_len >= 0);
            let byteslice = std::slice::from_raw_parts(cstr, cstr_len as usize);
            // Safety: we checked utf8ness in the guard (and trust that ob_analyze_utf8 is correct)
            std::str::from_utf8_unchecked(byteslice)
        }
    }
}

impl SlawEmit<'_> for () {
    fn can_emit(slaw: &slaw) -> bool {
        unsafe { slaw_is_nil(slaw.as_bslaw()) }
    }
    fn guarded_emit(slaw: &slaw) -> () {
        let bslaw = slaw.as_bslaw();
        unsafe {
            guard!(bslaw);
            ()
        }
    }
}

impl SlawEmit<'_> for bool {
    fn can_emit(slaw: &slaw) -> bool {
        unsafe { slaw_is_boolean(slaw.as_bslaw()) }
    }
    fn guarded_emit(slaw: &slaw) -> bool {
        let bslaw = slaw.as_bslaw();
        unsafe {
            guard!(bslaw);
            *slaw_boolean_emit(bslaw)
        }
    }
}

impl<'a> SlawEmit<'a> for &'a protein {
    fn can_emit(slaw: &slaw) -> bool {
        unsafe { slaw_is_protein(slaw.as_bslaw()) }
    }
    fn guarded_emit(slaw: &slaw) -> &protein {
        let bslaw = slaw.as_bslaw();
        unsafe {
            guard!(bslaw);
            todo!()
            // *(bslaw)
        }
    }
}
