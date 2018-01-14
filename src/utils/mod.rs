// Copyright (c) 2017 fd developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::path::Path;
use std::iter::Iterator;
use std::ascii::AsciiExt;

#[cfg(windows)]
pub mod utf16;

/// Determine if an os string ends with any of the given extensions (case insensitive).
pub fn path_has_any_extension<'a, I>(path: &Path, exts: I) -> bool
where
    I: 'a + IntoIterator<Item = &'a String>,
{
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        exts.into_iter().any(|x| {
            path.as_os_str()
                .as_bytes()
                .iter()
                .rev()
                .zip(x.as_bytes().iter().rev())
                .all(|(a, b)| a.eq_ignore_ascii_case(&b))
        })
    }

    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        use std::char::decode_utf16;

        if let Some(os_str) = path.file_name() {
            let utf16_vec: Vec<u16> = os_str.encode_wide().collect();
            exts.into_iter().any(|x| {
                decode_utf16(utf16::reverse_iter(utf16_vec.iter()))
                    .zip(x.chars().rev())
                    .all(|(a, b)| a.map(|c| c.eq_ignore_ascii_case(&b)).unwrap_or(false))
            })
        } else {
            false
        }
    }
}
