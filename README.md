# JSON Key-Value serialization
This is a very simple library and Rust trait for serializing Rust structures to JSON with a unique key.

It was supposed to help build complicated state trees (like you might see in Redux) but it never really achieved that goal. Instead, it makes your JSON document act a little bit more like Redis by associating a unique key to it.

It CAN be used to built hierarchical state trees, but the package itself isn't really making that easier as intended.
