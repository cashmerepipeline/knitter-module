use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;

use crate::protocols::*;

#[async_trait]
pub trait HandleRemoveReferences {
    /// 新建产品
    async fn handle_remove_references(
        &self,
        request: Request<RemoveReferencesRequest>,
    ) -> UnaryResponseResult<RemoveReferencesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_remove_references)
            .await
    }
}


async fn validate_view_rules(
    request: Request<RemoveReferencesRequest>,
) -> Result<Request<RemoveReferencesRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().subject_manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<RemoveReferencesRequest>,
) -> Result<Request<RemoveReferencesRequest>, Status> {
    Ok(request)
}

async fn handle_remove_references(
    request: Request<RemoveReferencesRequest>,
) -> Result<Response<RemoveReferencesResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().subject_manage_id;
    let entity_id = &request.get_ref().subject_entity_id;
    let reference_field_id = &request.get_ref().reference_field_id;
    let references = &request.get_ref().references;

    // TODO: 可能需要关联用户工程可读检查

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(*manage_id)
        .unwrap();

    let query_doc = doc! {
            ID_FIELD_ID.to_string():entity_id.clone(),
        };
    let mut modify_doc = Document::new();
    modify_doc.insert(
        reference_field_id,
        doc! {"$each":bson::to_document(references).unwrap()},
    );

    let result = manager
        .remove_from_array_field(query_doc, modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(RemoveReferencesResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}