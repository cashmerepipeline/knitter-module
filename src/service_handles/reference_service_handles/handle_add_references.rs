use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::traits::ManagerTrait;
use service_utils::types::UnaryResponseResult;

use crate::protocols::*;

#[async_trait]
pub trait HandleAddReferences {
    /// 新建产品
    async fn handle_add_references(
        &self,
        request: Request<AddReferencesRequest>,
    ) -> UnaryResponseResult<AddReferencesResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

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
            doc! {"$each": references.iter().map(|r| bson::to_document(r).unwrap()).collect::<Vec<Document>>()},
        );

        let result = manager
            .add_to_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AddReferencesResponse {
                result: "ok".to_string(),
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
    request: Request<AddReferencesRequest>,
) -> Result<Request<AddReferencesRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        // TODO: 可能需要关联用户工程可读检查

        let manage_id = &request.get_ref().subject_manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<AddReferencesRequest>,
) -> Result<Request<AddReferencesRequest>, Status> {
    Ok(request)
}