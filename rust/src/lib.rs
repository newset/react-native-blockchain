extern crate react_native_blockchain;

mod address;

pub use address::*;

#[cfg(target_os = "android")]
extern crate jni;

// #[cfg(target_os = "ios")]
// mod ios;
// #[cfg(target_os = "ios")]
// pub use self::ios::*;
