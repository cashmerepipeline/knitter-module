use dependencies_sync::bson::doc;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use service_utils::types::UnaryResponseResult;
use view;

use crate::ids_codes::field_ids::PROJECTS_STATUS_FIELD_ID;
use crate::ids_codes::manage_ids::PROJECTS_MANAGE_ID;
use crate::protocols::*;

#[async_trait]
pub trait HandleChangeProjectStatus {
    async fn handle_change_project_status(
        &self,
        request: Request<ChangeProjectStatusRequest>,
    ) -> UnaryResponseResult<ChangeProjectStatusResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let project_id = &request.get_ref().project_id;
        let status = &request.get_ref().status;
        let manage_id = PROJECTS_MANAGE_ID;

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

        let mut query_doc = doc! {};
        query_doc.insert(ID_FIELD_ID.to_string(), project_id);

        if !manager.entity_exists(&query_doc).await {
            return Err(Status::data_loss(t!("工程不存在")));
        };

        let mut modify_doc = doc! {};
        modify_doc.insert(PROJECTS_STATUS_FIELD_ID.to_string(), status);

        let result = manager.update_entity_field(query_doc, &mut modify_doc, &account_id).await;

        match result {
            Ok(_r) => Ok(Response::new(ChangeProjectStatusResponse {
                status: *status,
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}


async fn validate_view_rules(
    request: Request<ChangeProjectStatusRequest>,
) -> Result<Request<ChangeProjectStatusRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = PROJECTS_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<ChangeProjectStatusRequest>,
) -> Result<Request<ChangeProjectStatusRequest>, Status> {
    Ok(request)
}