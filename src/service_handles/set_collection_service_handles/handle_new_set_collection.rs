use dependencies_sync::tonic::{async_trait};
use dependencies_sync::bson::{self, doc, Document};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use managers::traits::ManagerTrait;
use service_utils::validate_name;
use service_utils::types::UnaryResponseResult;
use dependencies_sync::tonic::{Request, Response, Status};

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::protocols::*;


#[async_trait]
pub trait HandleNewSetCollection {
    /// 新建产品
     async fn handle_new_set_collection(
        &self,
        request: Request<NewSetCollectionRequest>,
    ) -> UnaryResponseResult<NewSetCollectionResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let name = &request.get_ref().name;
        let inner_root = &request.get_ref().inner_root_path;
        let external_root = &request.get_ref().external_root_path;
        let picture = &request.get_ref().picture;

        if validate_name(name).is_err() {
            return Err(Status::data_loss("名字不能为空."));
        }
        let name = name.as_ref().unwrap();

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc
            .get_manager_by_id(SET_COLLECTIONS_MANAGE_ID)
            .unwrap();

        // 新建条目
        let new_id = manager.get_new_entity_id().await.unwrap();
        let mut new_entity_doc = Document::new();
        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            doc! {name.language.clone():name.name.clone()},
        );
        new_entity_doc.insert(
            SET_COLLECTIONS_INNER_ROOT_PATH_FIELD_ID.to_string(),
            inner_root.clone(),
        );
        new_entity_doc.insert(
            SET_COLLECTIONS_EXTERNAL_ROOT_PATH_FIELD_ID.to_string(),
            external_root.clone(),
        );
        new_entity_doc.insert(
            SET_COLLECTIONS_PICTURE_FIELD_ID.to_string(),
            bson::to_bson(picture).unwrap(),
        );

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewSetCollectionResponse {
                // TODO: 发送新建事件
                result: r,
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
