use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tokio;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;

use crate::protocols::*;

#[async_trait]
pub trait HandleChangeReferencePrefab {
    /// 新建产品
    async fn handle_change_reference_prefab(
        &self,
        request: Request<ChangeReferencePrefabRequest>,
    ) -> UnaryResponseResult<ChangeReferencePrefabResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_change_reference_prefab)
            .await
    }
}


async fn validate_view_rules(
    request: Request<ChangeReferencePrefabRequest>,
) -> Result<Request<ChangeReferencePrefabRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = request.get_ref().subject_manage_id.clone();
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<ChangeReferencePrefabRequest>,
) -> Result<Request<ChangeReferencePrefabRequest>, Status> {
    Ok(request)
}

async fn handle_change_reference_prefab(
    request: Request<ChangeReferencePrefabRequest>,
) -> Result<Response<ChangeReferencePrefabResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let subject_manage_id = &request.get_ref().subject_manage_id;
    let subject_entity_id = &request.get_ref().subject_entity_id;
    let reference_field_id = &request.get_ref().reference_field_id;
    let old_reference = &request.get_ref().old_reference;
    let new_reference = &request.get_ref().new_reference;

    // TODO: 可能需要关联用户工程可读检查

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(*subject_manage_id)
        .unwrap();

    let query_doc = doc! {
            ID_FIELD_ID.to_string():subject_entity_id.clone(),
        };

    let mut modify_doc = Document::new();
    modify_doc.insert(
        reference_field_id,
        doc! {"$each":bson::to_document(&vec![bson::to_document(old_reference).unwrap()]).unwrap()},
    );
    let push_modify_doc = Document::new();
    modify_doc.insert(
        reference_field_id,
        doc! {"$each":bson::to_document(&vec![bson::to_document(new_reference).unwrap()]).unwrap()},
    );

    let pull_result = manager.add_to_array_field(query_doc.clone(), modify_doc, &account_id);
    let push_result = manager.add_to_array_field(query_doc, push_modify_doc, &account_id);

    let result = tokio::try_join!(pull_result, push_result);

    match result {
        Ok(_r) => Ok(Response::new(ChangeReferencePrefabResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}