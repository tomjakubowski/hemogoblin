#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use crate::{
        slaw_free, slaw_hash, slaw_len, slaw_nil, slaw_string, slaw_string_emit, slawx_equal,
    };

    #[test]
    fn test_slaw_string() {
        let hello = c"hello world";
        unsafe {
            let slaw = slaw_string(hello.as_ptr());
            let out = CStr::from_ptr(slaw_string_emit(slaw));
            assert_eq!(hello, out);
            slaw_free(slaw);
        }
    }

    #[test]
    fn test_slaw_len() {
        unsafe {
            let nil = slaw_nil();
            // nil is one oct
            assert_eq!(slaw_len(nil), 8);
            // short string is one oct
            let a = slaw_string(c"a".as_ptr());
            assert_eq!(slaw_len(a), 8);
            // slightly longer string is two oct
            let b = slaw_string(c"hello w".as_ptr());
            assert_eq!(slaw_len(b), 16);
            for s in [nil, a, b] {
                slaw_free(s);
            }
        }
    }

    #[test]
    fn test_slaw_hash() {
        unsafe {
            // nil is one oct
            let a = slaw_string(c"a".as_ptr());
            let b = slaw_string(c"b".as_ptr());
            assert_ne!(slaw_hash(a), slaw_hash(b));

            for s in [a, b] {
                slaw_free(s);
            }
        }
    }

    #[test]
    fn test_slawx_equal() {
        unsafe {
            // nil is one oct
            let a = slaw_string(c"a".as_ptr());
            let b = slaw_string(c"a".as_ptr());
            assert!(slawx_equal(a, b));

            for s in [a, b] {
                slaw_free(s);
            }
        }
    }
}
