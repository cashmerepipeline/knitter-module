#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod protocol {
    include!("io.knitter.rs");
}

pub mod ids_codes;
pub mod managers;
pub mod service_handles;