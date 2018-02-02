/// Emissaries are data-transfer objects. They're designed to move data into a semi-permanent cache.
/// This could be the store of a Redux application, or just in-memory cache in a program.
///
/// Emissaries all have a key and the data they contain.
///
/// Emissaries are infinitely nestable.
///
/// ** THE KEY **
/// The key is a namespaced identifier for the root of the data. 'users.me' may be a key
/// for storing a user object.
///
/// The data is the data, which must be able to be serialized, that will be stored where the key
/// specifies. 'users.me.FirstName'. The name is specified by the rust parameter name. Overrides are not yet possible.

use std::vec::Vec;
use serde::*;
use serde;
use serde_derive::*;
use serde_json;

/// Emissary is the base trait in the system. It may contain any type, given that it's serializable.
pub trait Emissary {

    /// Whatever type of data is being stored
    type DataType;

    /// Where in a stateful cache this data can be referenced (i.e "user.details.account")
    fn serialization_key(&self) -> String;
}

/// EmissaryContainer is a type which you can use to wrap existing data in, without needing to
/// implement the Emissary trait.
#[derive(Serialize, Debug)]
pub struct EmissaryContainer<T: serde::Serialize> {

    /// The data, as long as it can be serialized.
    pub data: T,

    /// The serialization key. This creates the key property when serialized.
    pub key: String
}

/// All emissary containers are de-facto Emissary objects.
impl<T: serde::Serialize> Emissary for EmissaryContainer<T> {

    type DataType = T;

    fn serialization_key(&self) -> String {
        self.key.clone()
    }
}

/// Actually turns the emissary into JSON
pub fn serialize_emissary<T: Emissary + serde::Serialize>(emissary: T) -> String {
    match serde_json::to_string(&emissary) {
        Ok(val) => val,
        Err(_) => String::new()
    }
}

/// Wrapper of EmissaryContainer; Creates an ad-hoc emissary for immediate serialization
pub fn create_emissary<T: serde::Serialize, W: ToString>(emissary_key: W, inner_data: T) -> EmissaryContainer<T> {
    EmissaryContainer {
        data: inner_data,
        key: emissary_key.to_string().clone()
    }
}