/// 新项目
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewProjectRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="2")]
    pub inner_root_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub external_root_path: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub picture: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewProjectResponse {
    /// 成功返回项目id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记项目已经完成
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeProjectStatusRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(enumeration="ProjectStatus", tag="2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeProjectStatusResponse {
    #[prost(enumeration="ProjectStatus", tag="1")]
    pub status: i32,
}
/// 关联资产集合到项目
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateAssetCollectionsToProjectRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateAssetCollectionsToProjectResponse {
    /// "ok" if succeed
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取消项目关联资产集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeassociateAssetCollectionsFromProjectRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeassociateAssetCollectionsFromProjectResponse {
    /// "ok" if succeed
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 关联布景集合到项目
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateSetCollectionsToProjectRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateSetCollectionsToProjectResponse {
    /// "ok" if succeed
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取消关联项目布景集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeassociateSetCollectionsFromProjectRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeassociateSetCollectionsFromProjectResponse {
    /// "ok" if succeed
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得关联资产集表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectAssociatedAssetCollectionsRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectAssociatedAssetCollectionsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub asset_collections: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得项目景集合表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectAssociatedSetCollectionsRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectAssociatedSetCollectionsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub set_collections: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得项目集表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectEpicsRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectEpicsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub epics: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProjectStatus {
    ProjectCanceled = 0,
    ProjectRunning = 1,
    ProjectSuspended = 2,
    ProjectComplete = 3,
}
/// 新资产集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssetCollectionRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="2")]
    pub inner_root_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub external_root_path: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub picture: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssetCollectionResponse {
    /// 成功返回项目id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得资产数量
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssetTotalCountRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssetTotalCountResponse {
    #[prost(uint64, tag="1")]
    pub total_count: u64,
}
/// 取得组合件数量
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssemblyTotalCountRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssemblyTotalCountResponse {
    #[prost(uint64, tag="1")]
    pub total_count: u64,
}
/// 取得资产页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssetsPageRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub page_index: u32,
    #[prost(uint32, tag="3")]
    pub total_pages_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssetsPageResponse {
    /// bson list
    #[prost(bytes="vec", repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得组合页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssembliesPageRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub page_index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssembliesPageResponse {
    /// bson list
    #[prost(bytes="vec", repeated, tag="1")]
    pub assemblies: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 标记资产集状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkAssetCollectionStatusRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(enumeration="AssetCollectionStatus", tag="2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkAssetCollectionStatusResponse {
    /// 修改成功返回当前集状态
    #[prost(enumeration="AssetCollectionStatus", tag="1")]
    pub status: i32,
}
/// 资产集合状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetCollectionStatus {
    AssetCollectionClosed = 0,
    AssetCollectionOpenning = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reference {
    #[prost(enumeration="ReferenceType", tag="1")]
    pub reference_type: i32,
    /// 源
    #[prost(string, tag="2")]
    pub source_id: ::prost::alloc::string::String,
    /// 规格
    #[prost(string, tag="4")]
    pub specs_id: ::prost::alloc::string::String,
    /// 预制件
    #[prost(string, tag="3")]
    pub prefab_id: ::prost::alloc::string::String,
}
/// 引用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddReferencesRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub subject_manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub subject_entity_id: ::prost::alloc::string::String,
    /// 主引用字段编码
    #[prost(string, tag="3")]
    pub reference_field_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub references: ::prost::alloc::vec::Vec<Reference>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddReferencesResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 列出引用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReferencesRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub subject_manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub subject_entity_id: ::prost::alloc::string::String,
    /// 主引用字段编码
    #[prost(string, tag="3")]
    pub reference_field_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReferencesResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub references: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 移除引用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveReferencesRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub subject_manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub subject_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reference_field_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub references: ::prost::alloc::vec::Vec<Reference>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveReferencesResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 修改引用预制件
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeReferencePrefabRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub subject_manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub subject_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reference_field_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub old_reference: ::core::option::Option<Reference>,
    #[prost(message, optional, tag="5")]
    pub new_reference: ::core::option::Option<Reference>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeReferencePrefabResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReferenceType {
    /// 资产
    RefAsset = 0,
    /// 组合
    RefAssembly = 1,
    /// 景
    RefSet = 2,
}
/// 新建资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssetRequest {
    #[prost(string, tag="1")]
    pub asset_collection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssetResponse {
    /// 成功返回新资产id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkAssetStatusRequest {
    #[prost(string, tag="1")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(enumeration="AssetStatus", tag="2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkAssetStatusResponse {
    /// 成功返回  "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得引用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferencedAssetsRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferencedAssetsResponse {
    /// 成功返回 "ok"
    #[prost(bytes="vec", repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 资产状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetStatus {
    AssetDone = 0,
    AssetSuspended = 1,
    AssetCanceled = 2,
}
/// 新建组装
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssemblyRequest {
    #[prost(string, tag="1")]
    pub asset_collection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssemblyResponse {
    /// 成功返回新资产组合id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 更新资产到组
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssemblyRequest {
    #[prost(string, tag="1")]
    pub assembly_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssemblyResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 引用资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceAssembliesRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reference_field_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub assembly_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceAssembliesResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 新建集
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEpicRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEpicResponse {
    /// 成功返回新id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得集的章节
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEpicSequencesRequest {
    #[prost(string, tag="1")]
    pub epic_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEpicSequencesResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub sequences: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 新建章节
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSequenceRequest {
    #[prost(string, tag="1")]
    pub epic_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSequenceResponse {
    /// 成功返回新id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得章节的镜头表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSequenceCutsRequest {
    #[prost(string, tag="1")]
    pub sequence_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSequenceCutsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub cuts: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 新建镜头
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCutRequest {
    #[prost(string, tag="1")]
    pub sequence_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCutResponse {
    /// 成功返回新id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 引用资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CutReferenceAssetsRequest {
    #[prost(string, tag="1")]
    pub cut_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CutReferenceAssetsResponse {
    /// 成功返回  "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 引用景
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CutRefereceSetsRequest {
    #[prost(string, tag="1")]
    pub cut_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CutRefereceSetsResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkCutStatusRequest {
    #[prost(string, tag="1")]
    pub cut_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkCutStatusResponse {
    /// 成功返回  "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得引用的资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCutReferencedAssetsRequest {
    #[prost(string, tag="1")]
    pub cut_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCutReferencedAssetsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 新项目
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSetCollectionRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="2")]
    pub inner_root_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub external_root_path: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub picture: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSetCollectionResponse {
    /// 成功返回id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得资产页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetCollectionSetsPageRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub page_index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetCollectionSetsPageResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub sets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得景数量
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetCollectionSetTotalCountRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetCollectionSetTotalCountResponse {
    #[prost(uint64, tag="1")]
    pub total_count: u64,
}
/// 新建景
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSetRequest {
    #[prost(string, tag="1")]
    pub set_collection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSetResponse {
    /// 成功返回新id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 添加引用资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferencAssetsRequest {
    #[prost(string, tag="1")]
    pub set_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub asset_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferencAssetsResponse {
    /// 成功返回"ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 更新引用资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReferencedAssetsRequest {
    #[prost(string, tag="1")]
    pub set_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReferencedAssetsResponse {
    /// 成功返回"ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 引用，原则上只被章节引用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceSetsRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceSetsResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkSetStatusRequest {
    #[prost(string, tag="1")]
    pub set_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkSetStatusResponse {
    /// 成功返回  "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
