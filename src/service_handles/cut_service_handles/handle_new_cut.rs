use tonic::async_trait;
use bson::{doc, Document};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::{
    DESCRIPTIONS_FIELD_ID, ID_FIELD_ID, NAME_MAP_FIELD_ID,
};
use managers::traits::ManagerTrait;
use service_common_handles::name_utils::validate_name;
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::protocols::*;


#[async_trait]
pub trait HandleNewCut {
    /// 新建
    async fn handle_new_cut(
        &self,
        request: Request<NewCutRequest>,
    ) -> UnaryResponseResult<NewCutResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let name = &request.get_ref().name;
        let sequence_id = &request.get_ref().sequence_id;
        let description = &request.get_ref().description;

       if !view::can_collection_write(&account_id, &role_group, &CUTS_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        if validate_name(name).is_err() {
            return Err(Status::data_loss("名字不能为空."));
        }
        let name = name.as_ref().unwrap();

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(CUTS_MANAGE_ID)
            .await
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
            CUTS_SEQUENCE_ID_FIELD_ID.to_string(),
            sequence_id.clone()
        );
        new_entity_doc.insert(
            DESCRIPTIONS_FIELD_ID.to_string(),
            description.clone()
        );

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        //TODO: 发出新镜头事件

        match result {
            Ok(r) => Ok(Response::new(NewCutResponse {
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


