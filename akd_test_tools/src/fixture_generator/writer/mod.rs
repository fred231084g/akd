// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under both the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree and the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree.

//! This module contains the Writer trait to serialize the tool's serde-compatible
//! objects to a format, as well as implementations of the trait.

use serde::Serialize;

/// Interface for writing output generated by the tool.
pub(crate) trait Writer {
    /// Writes a serde serializable object.
    fn write_object(&mut self, object: impl Serialize);

    /// Writes a comment that should be ignored by parsers.
    fn write_comment(&mut self, comment: &str);

    /// Writes a newline.
    fn write_line(&mut self);

    /// Flushes the internal buffer.
    fn flush(&mut self);
}

/// YAML implementor of Writer trait.
pub(crate) mod yaml;
