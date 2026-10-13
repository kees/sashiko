// Copyright 2026 The Sashiko Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The prompt bundle's content fingerprint, shared by build.rs, which names
//! the bundle's revision after it, and by `prompt_bundle`'s tests, which check
//! that the revision covers every bundled file. Both include this file by
//! path, so neither keeps a copy of the other's algorithm.

use std::hash::{Hash, Hasher};

/// A value that changes whenever any file's path or content changes.
///
/// Only inequality matters: a collision would leave an edited prompt
/// uninstalled, and a spurious change merely reinstalls the bundle.
pub fn content_fingerprint<'a>(files: impl IntoIterator<Item = (&'a str, &'a [u8])>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for (relative, content) in files {
        relative.hash(&mut hasher);
        content.hash(&mut hasher);
    }
    hasher.finish()
}
