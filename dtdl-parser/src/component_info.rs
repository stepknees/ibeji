// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

use crate::content_info::ContentInfo;
use crate::interface_info::InterfaceInfo;

/// A component specifies a reference to an interface.  It allows interfaces to contain other interfaces.
pub trait ComponentInfo: ContentInfo {
    /// Returns the interface, the component uses the term "schema" to refer to it.
    fn schema(&self) -> &Option<Box<dyn InterfaceInfo>>;
}
