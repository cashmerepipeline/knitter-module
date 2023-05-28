use dependencies_sync::bson::{self, doc};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use managers::traits::ManagerTrait;
use service_utils::types::UnaryResponseResult;

use crate::ids_codes::field_ids::SEQUENCES_EPIC_ID_FIELD_ID;
use crate::ids_codes::manage_ids::*;
use crate::protocols::*;

#[async_trait]
pub trait HandleGetEpicSequences {
    /// 新建产品
    async fn handle_get_epic_sequences(
        &self,
        request: Request<GetEpicSequencesRequest>,
    ) -> UnaryResponseResult<GetEpicSequencesResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let epic_id = &request.get_ref().epic_id;

        // TODO: 可能需要关联用户工程可读检查

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc
            .get_manager_by_id(SEQUENCES_MANAGE_ID)
            .unwrap();

        let query_doc = doc! {
            SEQUENCES_EPIC_ID_FIELD_ID.to_string(): epic_id,
        };

        let result = manager
            .get_entities_by_filter(&Some(query_doc))
            .await;

        match result {
            Ok(r) => Ok(Response::new(GetEpicSequencesResponse {
                sequences: r.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
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
    request: Request<GetEpicSequencesRequest>,
) -> Result<Request<GetEpicSequencesRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = SEQUENCES_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetEpicSequencesRequest>,
) -> Result<Request<GetEpicSequencesRequest>, Status> {
    Ok(request)
}
