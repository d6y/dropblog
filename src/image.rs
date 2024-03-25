use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

pub fn thumbnail(source: &Path, target: &Path, width: u16) -> Result<(u16, u16), Error> {
    let _convert_status = Command::new("convert")
        .arg(source)
        .arg("-resize")
        .arg(width.to_string())
        .arg("-auto-orient")
        .arg(target)
        .status()
        .expect("failed to execute convert process");

    let identify_output = Command::new("identify")
        .arg("-format")
        .arg("%wx%h")
        .arg(target)
        .output()
        .expect("failed to identify thumbnail");

    let output_text = String::from_utf8_lossy(&identify_output.stdout);

    let width_height: Vec<u16> = output_text.split('x').flat_map(|str| str.parse()).collect();

    match width_height.as_slice() {
        [w, h] => Ok((*w, *h)),
        _ => {
            let msg = format!("Expected wxh, not: {:?}", output_text);
            let cause = Error::new(ErrorKind::InvalidData, msg);
            Err(cause)
        }
    }
}

pub fn imagemagic_installed() -> bool {
    let convert_status = Command::new("convert").arg("-version").output();

    convert_status.is_ok()
}
