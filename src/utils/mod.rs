// Copyright (c) 2017 fd developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::path::Path;
use std::iter::Iterator;

/// Determine if an os string ends with any of the given extensions (case insensitive).
pub fn path_has_any_extension<'a, I>(path: &Path, exts: I) -> bool
where
    I: 'a + IntoIterator<Item = &'a String>,
{
    _path_has_any_extension(path, exts)
}

#[cfg(all(unix, not(target_os = "macos")))]
fn _path_has_any_extension<'a, I>(path: &Path, exts: I) -> bool
where
    I: 'a + IntoIterator<Item = &'a String>,
{
    use std::os::unix::ffi::OsStrExt;
    // TODO: remove these two lines when we drop support for Rust version < 1.23.
    #[allow(unused_imports)]
    use std::ascii::AsciiExt;

    exts.into_iter().any(|x| {
        let mut it = path.as_os_str().as_bytes().iter().rev();

        if x.as_bytes()
            .iter()
            .rev()
            .zip(&mut it)
            .all(|(a, b)| a.eq_ignore_ascii_case(&b))
        {
            match it.next() {
                Some(&b'/') | None => false,
                _ => true,
            }
        } else {
            false
        }
    })
}

#[cfg(any(target_os = "macos", windows))]
fn _path_has_any_extension<'a, I>(path: &Path, exts: I) -> bool
where
    I: 'a + IntoIterator<Item = &'a String>,
{
    if let Some(os_str) = path.file_name() {
        let name = os_str.to_string_lossy().to_lowercase();
        exts.into_iter().any(|x| name.ends_with(x) && &name != x)
    } else {
        false
    }
}
