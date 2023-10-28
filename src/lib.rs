use dependencies_sync::once_cell;
use dependencies_sync::rust_i18n::{self, i18n, t};
i18n!("locales");

pub mod ids_codes;
pub mod managers;
pub mod service_handles;
pub mod protocols;