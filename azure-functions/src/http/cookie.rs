use crate::rpc::{typed_data::Data, TypedData};
// use serde_derive::{Deserialize, Serialize};

/// Represents a browser cookie.
#[derive(Clone, Debug)]
pub struct Cookie { 
    /// The Name of the cookie.
    pub Name: String,
    /// The value of the cookie.
    pub Value: String,
}

impl Cookie<'_> {
    // Should there be setters and getters?
    // What other impls could be needed?
}

#[doc(hidden)]
impl<'a> From<&'a TypedData> for Cookie<'a> {
    fn from(data: &'a TypedData) -> Self {
        match &data.data {
            _ => None
        }
    }
}