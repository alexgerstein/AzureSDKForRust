use crate::attachment::AttachmentName;
use crate::clients::CosmosUriBuilder;
use crate::clients::ResourceType;
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::document::DocumentName;
use crate::permission::PermissionName;
use crate::stored_procedure::StoredProcedureName;
use crate::trigger::TriggerName;
use crate::user::UserName;
use crate::user_defined_function::UserDefinedFunctionName;
use crate::PartitionKeys;
use http::request::Builder;
use hyper_rustls::HttpsConnector;
use std::fmt::Debug;

pub trait HasHyperClient: Debug {
    fn hyper_client(&self) -> &hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;
}

pub trait CosmosClient: HasHyperClient {
    fn prepare_request(
        &self,
        uri_path: &str,
        http_method: hyper::Method,
        resource_type: ResourceType,
    ) -> Builder;
}

pub trait HasCosmosClient<C>: Debug
where
    C: CosmosClient,
{
    fn cosmos_client(&self) -> &C;
}

pub trait DatabaseClient<C>: HasCosmosClient<C>
where
    C: CosmosClient,
{
    fn database_name(&self) -> &str;
    fn list_collections(&self) -> crate::requests::ListCollectionsBuilder<'_, C>;
}

pub trait HasDatabaseClient<C, D>: Debug
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn database_client(&self) -> &D;
}

pub trait IntoDatabaseClient<C, D>: Debug
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn with_database(self, database_name: String) -> D;
}

pub(crate) trait DatabaseClientRequestPreparer<C>: DatabaseClient<C>
where
    C: CosmosClient,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder;
}

pub trait CollectionClient<C, D>: HasDatabaseClient<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn collection_name(&self) -> &str;
}

//// New implementation
pub trait CosmosTrait<CUB>
where
    CUB: CosmosUriBuilder,
{
    //fn list_databases(&self) -> requests::ListDatabasesBuilder<'_, CUB>;
    //fn with_database<'d>(&'d self, database_name: &'d dyn DatabaseName) -> DatabaseClient<'d, CUB>;
    //fn create_database<DB>(&self) -> requests::CreateDatabaseBuilder<'_, CUB, DB, No>
    //where
    //    DB: DatabaseName;
}

pub trait DatabaseTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    //fn list_collections(&self) -> requests::ListCollectionsBuilder<'_, CUB>;
    //fn get_database(&self) -> requests::GetDatabaseBuilder<'_, CUB>;
    //fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_, CUB>;
    //fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, CUB, No, No, No, No>;
    //fn with_collection<'c>(
    //    &'c self,
    //    collection_name: &'c dyn CollectionName,
    //) -> CollectionClient<'c, CUB>;
    //fn with_user<'c>(&'c self, user_name: &'c dyn UserName) -> UserClient<'c, CUB>;
    //fn list_users(&self) -> requests::ListUsersBuilder<'_, CUB>;
}

pub trait CollectionTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn collection_name(&self) -> &'a dyn CollectionName;
    //fn get_collection(&self) -> requests::GetCollectionBuilder<'_, CUB>;
    //fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_, CUB>;
    //fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, CUB, No, No>;
    //fn list_documents(&self) -> requests::ListDocumentsBuilder<'_, '_, CUB>;
    //fn create_document<T>(&self) -> requests::CreateDocumentBuilder<'_, '_, T, CUB, No, No>
    //where
    //    T: Serialize;
    //fn replace_document<T>(&self) -> requests::ReplaceDocumentBuilder<'_, '_, T, CUB, No, No, No>
    //where
    //    T: Serialize;
    //fn query_documents(&self) -> requests::QueryDocumentsBuilder<'_, '_, CUB, No>;
    //fn with_stored_procedure<'c>(
    //    &'c self,
    //    stored_procedure_name: &'c dyn StoredProcedureName,
    //) -> StoredProcedureClient<'c, CUB>;
    //fn with_user_defined_function<'c>(
    //    &'c self,
    //    user_defined_function_name: &'c dyn UserDefinedFunctionName,
    //) -> UserDefinedFunctionClient<'c, CUB>;
    //fn with_trigger<'c>(&'c self, trigger_name: &'c dyn TriggerName) -> TriggerClient<'c, CUB>;
    //fn list_stored_procedures(&self) -> requests::ListStoredProceduresBuilder<'_, CUB>;
    //fn list_user_defined_functions(&self)
    //    -> requests::ListUserDefinedFunctionsBuilder<'_, '_, CUB>;
    //fn list_triggers(&self) -> requests::ListTriggersBuilder<'_, '_, CUB>;
    //fn get_partition_key_ranges(&self) -> requests::GetPartitionKeyRangesBuilder<'_, '_, CUB>;
    //fn with_document<'c>(
    //    &'c self,
    //    document_name: &'c dyn DocumentName,
    //    partition_keys: &'c PartitionKeys,
    //) -> DocumentClient<'c, CUB>;
}

pub(crate) trait CollectionBuilderTrait<'a, CUB>: CollectionTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder;
}

pub trait DocumentTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn collection_name(&self) -> &'a dyn CollectionName;
    fn document_name(&self) -> &'a dyn DocumentName;
    fn partition_keys(&self) -> &'a PartitionKeys;
    //fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_, CUB>;
    //fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_, CUB>;
    //fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_, CUB>;
    //fn with_attachment(
    //    &'a self,
    //    attachment_name: &'a dyn AttachmentName,
    //) -> AttachmentClient<'_, CUB>;
}

pub(crate) trait DocumentBuilderTrait<'a, CUB>: DocumentTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder;
}

pub trait StoredProcedureTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn collection_name(&self) -> &'a dyn CollectionName;
    fn stored_procedure_name(&self) -> &'a dyn StoredProcedureName;
    //fn create_stored_procedure(&self) -> requests::CreateStoredProcedureBuilder<'_, CUB, No>;
    //fn replace_stored_procedure(&self) -> requests::ReplaceStoredProcedureBuilder<'_, CUB, No>;
    //fn execute_stored_procedure(&self) -> requests::ExecuteStoredProcedureBuilder<'_, '_, CUB>;
    //fn delete_stored_procedure(&self) -> requests::DeleteStoredProcedureBuilder<'_, CUB>;
}

pub(crate) trait StoredProcedureBuilderTrait<'a, CUB>:
    StoredProcedureTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder;
}

pub trait UserDefinedFunctionTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn collection_name(&self) -> &'a dyn CollectionName;
    fn user_defined_function_name(&self) -> &'a dyn UserDefinedFunctionName;
    //fn create_user_defined_function(
    //    &self,
    //) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, CUB, No>;
    //fn replace_user_defined_function(
    //    &self,
    //) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, CUB, No>;
    //fn delete_user_defined_function(&self) -> requests::DeleteUserDefinedFunctionBuilder<'_, CUB>;
}

pub(crate) trait UserDefinedFunctionBuilderTrait<'a, CUB>:
    UserDefinedFunctionTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(
        &self,
        method: hyper::Method,
        specify_user_defined_function_name: bool,
    ) -> http::request::Builder;
}

pub trait TriggerTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn collection_name(&self) -> &'a dyn CollectionName;
    fn trigger_name(&self) -> &'a dyn TriggerName;
    //fn create_trigger(&self) -> requests::CreateOrReplaceTriggerBuilder<'_, CUB, No, No, No>;
    //fn replace_trigger(&self) -> requests::CreateOrReplaceTriggerBuilder<'_, CUB, No, No, No>;
    //fn delete_trigger(&self) -> requests::DeleteTriggerBuilder<'_, CUB>;
}

pub(crate) trait TriggerBuilderTrait<'a, CUB>: TriggerTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(
        &self,
        method: hyper::Method,
        specify_trigger_name: bool,
    ) -> http::request::Builder;
}

pub trait AttachmentTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn collection_name(&self) -> &'a dyn CollectionName;
    fn document_name(&self) -> &'a dyn DocumentName;
    fn attachment_name(&self) -> &'a dyn AttachmentName;
    //fn create_slug(&self) -> requests::CreateSlugAttachmentBuilder<'_, '_, CUB, No, No>;
    //fn replace_slug(&self) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, CUB, No, No>;
    //fn create_reference(&self) -> requests::CreateReferenceAttachmentBuilder<'_, '_, CUB, No, No>;
    //fn replace_reference(&self)
    //    -> requests::ReplaceReferenceAttachmentBuilder<'_, '_, CUB, No, No>;
    //fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_, CUB>;
    //fn get(&self) -> requests::GetAttachmentBuilder<'_, '_, CUB>;
}

pub(crate) trait AttachmentBuilderTrait<'a, CUB>: AttachmentTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder;
}

pub trait UserTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn user_name(&self) -> &'a dyn UserName;
    //fn create_user(&self) -> requests::CreateUserBuilder<'_, CUB>;
    //fn get_user(&self) -> requests::GetUserBuilder<'_, CUB>;
    //fn replace_user(&self) -> requests::ReplaceUserBuilder<'_, CUB, No>;
    //fn delete_user(&self) -> requests::DeleteUserBuilder<'_, CUB>;
    //fn with_permission<'c>(
    //    &'c self,
    //    permission_name: &'c dyn PermissionName,
    //) -> PermissionClient<'c, CUB>;
    //fn list_permissions(&self) -> requests::ListPermissionsBuilder<'_, CUB>;
}

pub trait PermissionTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn user_name(&self) -> &'a dyn UserName;
    fn permission_name(&self) -> &'a dyn PermissionName;
    //fn create_permission<R>(&self) -> requests::CreatePermissionBuilder<'_, CUB, R, No>
    //where
    //    R: PermissionResource;
    //fn replace_permission<R>(&self) -> requests::ReplacePermissionBuilder<'_, CUB, R, No>
    //where
    //    R: PermissionResource;
    //fn get_permission(&self) -> requests::GetPermissionBuilder<'_, CUB>;
    //fn delete_permission(&self) -> requests::DeletePermissionsBuilder<'_, CUB>;
}