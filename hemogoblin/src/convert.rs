use crate::{Protein, Slaw, protein, slaw};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrongIlk;

macro_rules! impl_slaw_from {
    ($host:ty as $conv:expr) => {
        impl<'a> From<$host> for Slaw {
            fn from(val: $host) -> Slaw {
                $conv(val)
            }
        }
    };
    ($host:ty as ref $conv:expr) => {
        impl<'a> From<$host> for Slaw {
            fn from(val: $host) -> Slaw {
                $conv(&val)
            }
        }
    };
}

// Conversion from standard Rust values into Slaw
impl_slaw_from!(() as |_| Slaw::nil());
impl_slaw_from!(bool as Slaw::boolean);
impl_slaw_from!(String as ref Slaw::string);
impl_slaw_from!(&'a str as Slaw::string);

// Conversion from &slaw to standard Rust values
macro_rules! impl_try_from_slaw {
    ($transubstance:ty) => {
        impl_try_from_slaw!($transubstance as $transubstance);
    };
    ($transubstance:ty as consuming) => {
        impl_try_from_slaw!($transubstance as $transubstance as consuming);
    };
    ($t1:ty as $t2:ty) => {
        impl<'a> TryFrom<&'a slaw> for $t1 {
            type Error = WrongIlk;

            fn try_from(s: &'a slaw) -> Result<$t1, Self::Error> {
                s.try_emit::<$t2>().map(|s| s.into()).ok_or(WrongIlk)
            }
        }
    };
    ($t1:ty as $t2:ty as consuming) => {
        impl_try_from_slaw!($t1 as $t2);
        impl<'a> TryFrom<Slaw> for $t1 {
            type Error = WrongIlk;

            fn try_from(s: Slaw) -> Result<$t1, Self::Error> {
                s.try_emit::<$t2>().map(|s| s.into()).ok_or(WrongIlk)
            }
        }
    };
}

impl_try_from_slaw!(() as consuming);
impl_try_from_slaw!(&'a str);
impl_try_from_slaw!(String as &str as consuming);
impl_try_from_slaw!(bool as consuming);
// impl_try_from_slaw!(&'a protein);

impl TryFrom<Slaw> for Protein {
    type Error = WrongIlk;

    fn try_from(s: Slaw) -> Result<Self, Self::Error> {
        Protein::from_slaw(s).ok_or(WrongIlk)
    }
}

#[cfg(test)]
mod slaw_from_tests {
    use crate::Slaw;

    #[test]
    fn test_slaw_from_rumpus() {
        assert_eq!(Slaw::from(()), Slaw::nil());
        assert_eq!(Slaw::from(true), Slaw::boolean(true));
        assert_eq!(Slaw::from("hello"), Slaw::string("hello"));
        assert_eq!(Slaw::from("hello".to_string()), Slaw::string("hello"));
    }
}

#[cfg(test)]
mod slaw_into_tests {
    use crate::Slaw;

    #[test]
    fn test_slaw_into_rumpus() {
        assert_eq!(Slaw::nil().try_into(), Ok(()));
        assert_eq!(Slaw::boolean(true).try_into(), Ok(true));
        assert_eq!(Slaw::string("hello").try_into(), Ok("hello".to_string()));
        assert_eq!(Slaw::string("hello").as_slaw().try_into(), Ok("hello"));
        // FIXME: add protein and Protein tests
    }
}
