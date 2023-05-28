use cash_result::{Failed, OperationResult};
use dependencies_sync::bson::{self, doc};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;

use crate::protocols::*;

#[async_trait]
pub trait HandleListReferences {
    /// 新建产品
    async fn handle_list_references(
        &self,
        request: Request<ListReferencesRequest>,
    ) -> UnaryResponseResult<ListReferencesResponse> {}
}


async fn validate_view_rules(
    request: Request<ListReferencesRequest>,
) -> Result<Request<ListReferencesRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = request.get_ref().subject_manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<ListReferencesRequest>,
) -> Result<Request<ListReferencesRequest>, Status> {
    Ok(request)
}

async fn handle_list_references(
    request: Request<ListReferencesRequest>,
) -> Result<Response<ListReferencesResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let subject_manage_id = &request.get_ref().subject_manage_id;
    let subject_entity_id = &request.get_ref().subject_entity_id;
    let reference_field_id = &request.get_ref().reference_field_id;

    // TODO: 可能需要关联用户工程可读检查

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(*subject_manage_id)
        .unwrap();

    let _query_doc = doc! {
            ID_FIELD_ID.to_string():subject_entity_id.clone(),
        };

    let result = manager.get_entity_by_id(subject_entity_id).await;

    let result = if let Ok(refs) = result {
        match refs.get_array(reference_field_id) {
            Ok(r) => Ok(r
                .to_vec()
                .iter()
                .map(|x| bson::to_vec(x).unwrap())
                .collect()),
            _ => Err(OperationResult::Failed(Failed {
                details: "没有参考".to_string(),
                operation: "handle_list_references".to_string(),
            })),
        }
    } else {
        return Err(Status::data_loss("实体不存在"));
    };

    match result {
        Ok(r) => Ok(Response::new(ListReferencesResponse { references: r })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}