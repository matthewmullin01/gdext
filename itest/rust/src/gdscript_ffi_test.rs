#![allow(dead_code)]

use gdext_class::api::RefCounted;
use gdext_class::builder::ClassBuilder;
use gdext_class::obj::Base;
use gdext_class::traits::GodotExt;
use gdext_macros::{godot_api, GodotClass};

#[derive(GodotClass, Debug)]
#[godot(base=RefCounted)]
struct RustFfi {
    to_mirror: i64,

    #[base]
    some_base: Base<RefCounted>,
}

#[godot_api]
impl RustFfi {
    #[godot]
    fn create_int(&self) -> i64 {
        -468192231
    }

    #[godot]
    fn accept_int(&self, i: i64) -> bool {
        i == -468192231
    }

    #[godot]
    fn mirror_int(&self, i: i64) -> i64 {
        i
    }
}

#[godot_api]
impl GodotExt for RustFfi {
    fn register_class(_builder: &mut ClassBuilder<Self>) {
        println!("Register class RustFfi!");
    }

    fn init(base: Base<Self::Base>) -> Self {
        Self {
            to_mirror: 77,
            some_base: base,
        }
    }
}

pub(crate) fn run() -> bool {
    let ok = true;
    ok
}

// pub(crate) fn register() {
//     gdext_class::register_class::<RustFfi>();
// }
