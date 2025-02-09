// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

use crate::content_info::ContentInfo;
use crate::schema_info::SchemaInfo;

/// A property specifies a value that may be read and in some cases also written.
pub trait PropertyInfo: ContentInfo {
    /// Returns the schema.
    fn schema(&self) -> &Option<Box<dyn SchemaInfo>>;

    /// Returns whether the property is writable.
    fn writable(&self) -> bool;
}
