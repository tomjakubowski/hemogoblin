use hemogoblin::{Protein, Slaw};
use std::path::Path;

#[test]
fn test_protein_from_file() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let path = fixtures.join("hello_yaml.protein");
    let protein = Protein::read_from_path(path).expect("couldn't read protein");
    // FIXME: write a better test which checks descrips/ingests
    let dbgstr = format!("{:?}", protein);
    assert!(dbgstr.contains("PROT") && dbgstr.contains("ob-standard-protein-greeting"));
}

#[test]
fn test_protein_from_file_2() {
    // test reading an unusual protein -- this is to support early development, before I have written SlawMap/SlawList support
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let path = fixtures.join("idiosync.protein");
    let protein = Protein::read_from_path(path).expect("couldn't read protein");
    assert_eq!(protein.descrips(), Slaw::string("unusual-protein"));
    assert_eq!(protein.ingests(), Slaw::boolean(true));
}
