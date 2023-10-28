/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 工作管理
Created:  2020-11-28T02:17:47.146Z
Modified: !date!
*/

use std::sync::Arc;

use cash_core::{manage_from_document, Manage};
use cash_result::*;
use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;
// use log::{error, info, warn};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;
use manage_define::manage_ids::MANAGES_MANAGE_ID;
use managers::{declare_get_manager, ManagerTrait, Manager, ManagerInner};

use crate::ids_codes::manage_ids::EPICS_MANAGE_ID;

#[derive(Default)]
pub struct EpicsManager;

/// 缓存
static mut EPICS_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut EPICS_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;

/// 管理器
static mut EPICS_MANAGER: Option<Arc<Manager>> = None;

// 声明管理器取得函数
declare_get_manager!(EpicsManager, EPICS_MANAGER);

// 实现接口
#[async_trait]
impl ManagerTrait for EpicsManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed(
            "unregister",
            format!(
                    "{}-{}-{}",
                    t!("管理器不能被注销"),
                    self.get_id(),
                    self.get_name()
            ),
        ))
    }

    fn get_id(&self) -> i32 {
        EPICS_MANAGE_ID
    }

    fn get_name(&self) -> String {
        "EpicsManager".to_string()
    }

    fn has_cache(&self) -> bool {
        false
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if EPICS_MANAGE.is_some() {
                EPICS_MANAGE.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = EPICS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };
                let manage: Manage = manage_from_document(m_doc).unwrap();
                EPICS_MANAGE.replace(Arc::new(RwLock::new(manage)));
                EPICS_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if EPICS_MANAGE_DOCUMENT.is_some() {
                EPICS_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = EPICS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };

                EPICS_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                EPICS_MANAGE_DOCUMENT.clone().unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
