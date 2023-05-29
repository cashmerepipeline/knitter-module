use dependencies_sync::bson::{doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::protocols::*;

#[async_trait]
pub trait HandleAssociateSetCollectionsToProject {
    /// 新建产品
    async fn handle_associate_set_collections_to_project(
        &self,
        request: Request<AssociateSetCollectionsToProjectRequest>,
    ) -> UnaryResponseResult<AssociateSetCollectionsToProjectResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_associate_set_collections_to_project)
            .await
    }
}

async fn validate_view_rules(
    request: Request<AssociateSetCollectionsToProjectRequest>,
) -> Result<Request<AssociateSetCollectionsToProjectRequest>, Status> {
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
    request: Request<AssociateSetCollectionsToProjectRequest>,
) -> Result<Request<AssociateSetCollectionsToProjectRequest>, Status> {
    Ok(request)
}

async fn handle_associate_set_collections_to_project(
    request: Request<AssociateSetCollectionsToProjectRequest>,
) -> Result<Response<AssociateSetCollectionsToProjectResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let project_id = &request.get_ref().project_id;
    let library_ids = &request.get_ref().collection_ids;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(PROJECTS_MANAGE_ID).unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():project_id.clone(),
    };
    let mut modify_doc = Document::new();
    modify_doc.insert(
        PROJECTS_SET_COLLECTIONS_FIELD_ID.to_string(),
        doc! {"$each":library_ids.clone()},
    );

    let result = manager
        .add_to_array_field(query_doc, modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(AssociateSetCollectionsToProjectResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
