//! Bindings to subset of libLoam, with some plasma retorts included.
//! Making an ObRetort<T> which can be extended by other libraries is an interesting challenge.

use plasma_sys::ob_retort;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// A subset of the retorts, which are enforced to be errors only (splend retorts are represented
/// as the Ok() variant of ObResult)
pub enum ObRetort {
    NoMem,
    BadIndex,
    ArgumentWasNull,
    NotFound,
    InvalidArgument,
    UnknownError,
    InadequateClass,
    AlreadyPresent,
    Empty,
    InvalidOperation,
    Disconnected,
    VersionMismatch,
    Other(ob_retort),
}

impl ObRetort {
    pub fn new(tort: ob_retort) -> Option<ObRetort> {
        use self::ObRetort::*;

        if tort >= 0 {
            return None;
        }
        Some(match tort {
            OB_NO_MEM => NoMem,
            OB_BAD_INDEX => BadIndex,
            OB_ARGUMENT_WAS_NULL => ArgumentWasNull,
            OB_NOT_FOUND => NotFound,
            OB_INVALID_ARGUMENT => InvalidArgument,
            OB_UNKNOWN_ERR => UnknownError,
            OB_INADEQUATE_CLASS => InadequateClass,
            OB_ALREADY_PRESENT => AlreadyPresent,
            OB_EMPTY => Empty,
            OB_INVALID_OPERATION => InvalidOperation,
            OB_DISCONNECTED => Disconnected,
            OB_VERSION_MISMATCH => VersionMismatch,
            _ => Other(tort),
        })
    }

    pub fn map<T>(tort: ob_retort, f: impl FnOnce(ob_retort) -> T) -> ObResult<T> {
        if tort >= 0 {
            Ok(f(tort))
        } else {
            let err = ObRetort::new(tort).expect("logic error, improper retort");
            Err(err)
        }
    }
}
impl TryFrom<ob_retort> for ObRetort {
    fn try_from(tort: ob_retort) -> Result<Self, Self::Error> {
        Self::new(tort).ok_or(())
    }

    type Error = ();
}

impl<'a> Into<i64> for ObRetort {
    fn into(self) -> i64 {
        use self::ObRetort::*;

        match self {
            NoMem => OB_NO_MEM,
            BadIndex => OB_BAD_INDEX,
            ArgumentWasNull => OB_ARGUMENT_WAS_NULL,
            NotFound => OB_NOT_FOUND,
            InvalidArgument => OB_INVALID_ARGUMENT,
            UnknownError => OB_UNKNOWN_ERR,
            InadequateClass => OB_INADEQUATE_CLASS,
            AlreadyPresent => OB_ALREADY_PRESENT,
            Empty => OB_EMPTY,
            InvalidOperation => OB_INVALID_OPERATION,
            Disconnected => OB_DISCONNECTED,
            VersionMismatch => OB_VERSION_MISMATCH,
            Other(tort) => tort,
        }
    }
}

pub type ObResult<T = ()> = Result<T, ObRetort>;

// Ideally all the 'torts would be generated for us by `bindgen`, but `bindgen` won't expand the
// macros ob-retorts.h uses. Somebody found a clever workaround, but it is blocked on moving our
// use of bindgen into build.rs.
// https://github.com/rust-lang/rust-bindgen/issues/753#issuecomment-459851952
// So let's use a gnarly macro instead!

macro_rules! expand_ob_retorts {
    ($(#define $name:ident $val:expr)+) => {
        $(#[allow(unused_parens)] pub const $name: i64 = $val;)+
    };
}

#[allow(non_snake_case)]
pub const fn OB_CONST_RETORT(n: i64) -> i64 {
    n
}

expand_ob_retorts! {
    #define OB_OK                OB_CONST_RETORT (0)
    #define OB_NO_MEM            OB_CONST_RETORT (-201)
    #define OB_BAD_INDEX         OB_CONST_RETORT (-202)
    #define OB_ARGUMENT_WAS_NULL OB_CONST_RETORT (-203)
    #define OB_NOT_FOUND         OB_CONST_RETORT (-204)
    #define OB_INVALID_ARGUMENT  OB_CONST_RETORT (-205)
    #define OB_UNKNOWN_ERR       OB_CONST_RETORT (-220)
    #define OB_INADEQUATE_CLASS  OB_CONST_RETORT (-221)
    #define OB_ALREADY_PRESENT   OB_CONST_RETORT (-222)
    #define OB_EMPTY             OB_CONST_RETORT (-223)
    #define OB_INVALID_OPERATION OB_CONST_RETORT (-224)
    #define OB_DISCONNECTED      OB_CONST_RETORT (-260)
    #define OB_VERSION_MISMATCH  OB_CONST_RETORT (-261)
    #define OB_PARSE_ERROR       OB_CONST_RETORT (-270)

    #define OB_RETORTS_PLASMA_FIRST OB_CONST_RETORT(200000)
    #define OB_RETORTS_PLASMA_POOLS (OB_RETORTS_PLASMA_FIRST)
    #define OB_RETORTS_PLASMA_SLAW  (OB_RETORTS_PLASMA_FIRST + 10000)
    #define OB_RETORTS_PLASMA_IO    (OB_RETORTS_PLASMA_FIRST + 20000)

    #define SLAW_CORRUPT_PROTEIN     -(OB_RETORTS_PLASMA_SLAW + 0)
    #define SLAW_CORRUPT_SLAW        -(OB_RETORTS_PLASMA_SLAW + 1)
    #define SLAW_FABRICATOR_BADNESS  -(OB_RETORTS_PLASMA_SLAW + 2)
    #define SLAW_NOT_NUMERIC         -(OB_RETORTS_PLASMA_SLAW + 3)
    #define SLAW_RANGE_ERR           -(OB_RETORTS_PLASMA_SLAW + 4)
    #define SLAW_UNIDENTIFIED_SLAW   -(OB_RETORTS_PLASMA_SLAW + 5)
    #define SLAW_WRONG_LENGTH        -(OB_RETORTS_PLASMA_SLAW + 6)
    #define SLAW_ALIAS_NOT_SUPPORTED -(OB_RETORTS_PLASMA_IO + 0)
    #define SLAW_BAD_TAG             -(OB_RETORTS_PLASMA_IO + 1)
    #define SLAW_END_OF_FILE         -(OB_RETORTS_PLASMA_IO + 2)
    #define SLAW_PARSING_BADNESS     -(OB_RETORTS_PLASMA_IO + 3)
    #define SLAW_WRONG_FORMAT        -(OB_RETORTS_PLASMA_IO + 4)
    #define SLAW_WRONG_VERSION       -(OB_RETORTS_PLASMA_IO + 5)
    #define SLAW_YAML_ERR            -(OB_RETORTS_PLASMA_IO + 6)
    #define SLAW_NO_YAML             -(OB_RETORTS_PLASMA_IO + 7)
}
