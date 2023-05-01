#[macro_use]
extern crate rust_i18n;
i18n!("locales");

pub mod protocols {
    include!("knitter_module.rs");
}

pub mod ids_codes;
pub mod managers;
pub mod service_handles;