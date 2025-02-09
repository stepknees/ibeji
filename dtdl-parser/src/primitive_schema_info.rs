// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

use crate::schema_info::SchemaInfo;

/// A primitive schema is the trait that represents all primitive schemas, like boolean, integer, string and time.
pub trait PrimitiveSchemaInfo: SchemaInfo {}
