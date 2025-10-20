use crate::{Slaw, slaw};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrongIlk;

macro_rules! slaw_from {
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
slaw_from!(() as |_| Slaw::nil());
slaw_from!(bool as Slaw::boolean);
slaw_from!(String as ref Slaw::string);
slaw_from!(&'a str as Slaw::string);

// Conversion from &slaw to standard Rust values
macro_rules! slaw_into {
    ($transubstance:ty) => {
        impl<'a> TryInto<$transubstance> for &'a slaw {
            type Error = WrongIlk;

            fn try_into(self) -> Result<$transubstance, Self::Error> {
                self.try_emit::<$transubstance>()
                    .map(|s| s.into())
                    .ok_or(WrongIlk)
            }
        }
    };
    ($t1:ty as $t2:ty) => {
        impl<'a> TryInto<$t1> for &'a slaw {
            type Error = WrongIlk;

            fn try_into(self) -> Result<$t1, Self::Error> {
                self.try_emit::<$t2>().map(|s| s.into()).ok_or(WrongIlk)
            }
        }
    };
}

slaw_into!(());
slaw_into!(&'a str);
slaw_into!(String as &str);
slaw_into!(bool);

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
        // assert_eq!(Slaw::nil().try_into(), Ok((())));
    }
}
