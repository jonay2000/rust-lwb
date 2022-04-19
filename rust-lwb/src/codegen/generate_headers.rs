use crate::codegen::error::CodegenError;
use chrono::{DateTime, Local, Utc};
use std::io::Write;
use crate::codegen::FormattingFile;

fn write_header(file: &mut FormattingFile, _codegen_stamp: &str) -> Result<(), CodegenError> {
    write!(
        file,
        "\
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
// |==========================================================|
// |      WARNING: THIS FILE IS AUTOMATICALLY GENERATED.      |
// |      CHANGES TO IT WILL BE DELETED WHEN REGENERATED.     |
// | IN GENERAL, THIS FILE SHOULD NOT BE MODIFIED IN ANY WAY. |
// |==========================================================|
"
    )?;

    Ok(())
}

pub fn write_headers(
    modrs: &mut FormattingFile,
    files: &mut [(&mut FormattingFile, &str)],
    prelude_import_location: &str,
) -> Result<(), CodegenError> {
    let now: DateTime<Local> = Local::now();
    let now_utc: DateTime<Utc> = Utc::now();
    let codegen_stamp = format!(
        "{} - {}",
        now.format("%d/%m/%Y %T %Z"),
        now_utc.format("%d/%m/%Y %T %Z")
    );

    for (file, _) in files.iter_mut() {
        write_header(file, &codegen_stamp)?;
    }

    write_header(modrs, &codegen_stamp)?;

    for (_, name) in files.iter_mut() {
        write!(
            modrs,
            "
#[rustfmt::skip]
mod {name};
pub use {name}::*;
"
        )?;
    }

    write!(
        modrs,
        "
#[rustfmt::skip]
mod prelude {{
    pub use {prelude_import_location}::codegen_prelude::*;
    pub use super::*;
}}"
    )?;

    Ok(())
}
