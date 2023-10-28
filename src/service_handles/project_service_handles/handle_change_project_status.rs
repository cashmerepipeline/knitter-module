use dependencies_sync::bson::doc;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};

use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::*;
use managers::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;


use crate::ids_codes::field_ids::PROJECTS_STATUS_FIELD_ID;
use crate::ids_codes::manage_ids::PROJECTS_MANAGE_ID;
use crate::protocols::*;

#[async_trait]
pub trait HandleChangeProjectStatus {
    async fn handle_change_project_status(
        &self,
        request: Request<ChangeProjectStatusRequest>,
    ) -> UnaryResponseResult<ChangeProjectStatusResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_change_project_status)
            .await
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

async fn handle_change_project_status(
    request: Request<ChangeProjectStatusRequest>,
) -> Result<Response<ChangeProjectStatusResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

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

    let result = manager
        .update_entity_field(query_doc, &mut modify_doc, &account_id)
        .await;

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
