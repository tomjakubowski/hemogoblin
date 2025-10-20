use std::path::Path;

use plasma_sys::{protein_descrips, protein_ingests};

use crate::{
    Slaw,
    io::SlawInput,
    loam::{ObResult, ObRetort},
    slaw,
};

#[repr(transparent)]
pub struct Protein {
    slaw: Slaw,
}

impl Protein {
    pub fn read_from_path<P>(path: P) -> ObResult<Self>
    where
        P: AsRef<Path>,
    {
        let mut stream = SlawInput::open_file(path)?.into_iter();
        let pending_slaw = match stream.next() {
            Some(could_be_s) => could_be_s,
            None => Err(ObRetort::Empty),
        };
        let slaw = pending_slaw?;
        Protein::from_slaw(slaw).ok_or(ObRetort::InvalidArgument)
    }

    pub fn from_slaw(slaw: Slaw) -> Option<Self> {
        slaw.is_protein().then(|| Protein { slaw })
    }
}

impl std::fmt::Debug for Protein {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.slaw.fmt(f)
    }
}

impl std::ops::Deref for Protein {
    type Target = protein;
    fn deref(&self) -> &protein {
        protein::from_slaw(&self.slaw).unwrap()
    }
}

#[derive(Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct protein {
    slaw: slaw,
}

impl protein {
    pub fn from_slaw(slaw: &slaw) -> Option<&Self> {
        slaw.is_protein()
            .then(|| unsafe { &*(slaw as *const slaw as *const protein) })
    }

    /// Borrow this protein's digests as a slaw, if present.
    pub fn try_descrips(&self) -> Option<&slaw> {
        unsafe {
            let descrips = protein_descrips(self.slaw.as_bslaw());
            (!descrips.is_null()).then(|| slaw::from_bslaw(descrips))
        }
    }

    pub fn descrips(&self) -> &slaw {
        self.try_descrips().expect("protein has no descrips")
    }

    /// Borrow this protein's ingests as a slaw, if present.
    pub fn try_ingests(&self) -> Option<&slaw> {
        unsafe {
            let ingests = protein_ingests(self.slaw.as_bslaw());
            (!ingests.is_null()).then(|| slaw::from_bslaw(ingests))
        }
    }

    pub fn ingests(&self) -> &slaw {
        self.try_ingests().expect("protein has no ingests")
    }
}
