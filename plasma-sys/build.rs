fn main() {
    // FIXME: hack to get this building on tom's mac for POC... should either vendor yaml (sigh) or
    // stop vendoring altogether and use a prebuilt/installed libPlasma
    println!("cargo:rustc-link-search=/opt/homebrew/lib/");
    println!("cargo:rustc-link-lib=yaml");
    cc::Build::new()
        .includes(&["."])
        .includes(&[
            ".",
            // FIXME: hack to get this building on tom's mac... should either vendor yaml (sigh) or
            // stop vendoring altogether and use a prebuilt/installed libPlasma
            "/opt/homebrew/include/",
        ])
        .flags(&["-Wno-unused-parameter"])
        // FIXME: There is an actually potentially concerning comparison in slaw.c line 1356 which
        // I would like to investigate, but silencing for now
        .flags(&["-Wno-sign-compare"])
        .define("QUOTED_PREFIX", r#""/opt/hemogoblin""#)
        .files(&[
            "libLoam/c/datadir.c",
            "libLoam/c/ob-atomic.c",
            "libLoam/c/ob-dirs.c",
            "libLoam/c/ob-file.c",
            "libLoam/c/ob-hash-city.c",
            "libLoam/c/ob-hash.c",
            "libLoam/c/ob-log.c",
            "libLoam/c/ob-retorts.c",
            "libLoam/c/ob-string.c",
            "libLoam/c/ob-time.c",
            "libLoam/c/ob-util.c",
            "libLoam/c/ob-vers.c",
            "libLoam/c/prefix.c",
            "stubs/stubs.c",
        ])
        .files(&[
            "libPlasma/c/plasma-util.c",
            "libPlasma/c/protein.c",
            "libPlasma/c/slaw-concat.c",
            "libPlasma/c/slaw-interop.c",
            "libPlasma/c/slaw-io-convenience.c",
            "libPlasma/c/slaw-io-file.c",
            "libPlasma/c/slaw-io.c",
            "libPlasma/c/slaw-numerics.c",
            "libPlasma/c/slaw-ordering.c",
            "libPlasma/c/slaw-string.c",
            "libPlasma/c/slaw-v1.c",
            "libPlasma/c/slaw-walk.c",
            "libPlasma/c/slaw-yaml.c",
            "libPlasma/c/slaw.c",
        ])
        .compile("Plasma")
}
