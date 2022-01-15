use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use log::{debug, LevelFilter};
use tempfile::NamedTempFile;

const BIN_NAME: &str = "bin/vasm";

#[allow(unused)]
pub fn init() {
    let _ = env_logger::builder()
        .filter(None, LevelFilter::Debug)
        .is_test(true)
        .try_init();
    debug!("Logger initialized")
}

#[allow(unused)]
pub fn assemble_string<S: AsRef<str>>(assembly: S) -> (Vec<u8>, NamedTempFile) {
    let mut tmpfile = tempfile::NamedTempFile::new().expect("Creating tempfile");
    tmpfile.write_all(assembly.as_ref().as_bytes()).expect("Writing assembly to tempfile");
    assemble_file(tmpfile.path())
}

#[allow(unused)]
pub fn assemble_file<P: AsRef<Path>>(path: P) -> (Vec<u8>, NamedTempFile) {
    std::env::set_current_dir(format!("{}/tests", env!("CARGO_MANIFEST_DIR"))).expect("Setting environmental variable for current directory");

    let mut tempfile = tempfile::NamedTempFile::new().expect("Creating tempfile");
    let child = Command::new(BIN_NAME)
        .arg("-dotdir")
        .arg("-Fbin")
        .arg("-o")
        .arg(tempfile.path().as_os_str())
        .arg("-i")
        .arg(path.as_ref().as_os_str())
        .stdout(Stdio::null())
        .spawn()
        .expect("Running vasm")
        .wait()
        .expect("Waiting for vasm to complete");

    if !child.success() {
        panic!("VASM failed");
    }

    let mut buf = Vec::default();
    tempfile.read_to_end(&mut buf).expect("Reading output file");
    (buf, tempfile)
}