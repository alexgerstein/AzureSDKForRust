use crate::clients::{
    Client, CosmosUriBuilder, DatabaseClient, DocumentClient, ResourceType, StoredProcedureClient,
    TriggerClient, UserDefinedFunctionClient,
};
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::document::DocumentName;
use crate::requests;
use crate::stored_procedure::StoredProcedureName;
use crate::trigger::TriggerName;
use crate::user_defined_function::UserDefinedFunctionName;
use crate::{CollectionBuilderTrait, CollectionTrait, DatabaseTrait, PartitionKeys};
use azure_sdk_core::No;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct CollectionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    database_client: &'a DatabaseClient<'a, CUB>,
    collection_name: &'a dyn CollectionName,
}

impl<'a, CUB> CollectionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        database_client: &'a DatabaseClient<'a, CUB>,
        collection_name: &'a dyn CollectionName,
    ) -> Self {
        CollectionClient {
            database_client,
            collection_name,
        }
    }

    pub(crate) fn main_client(&self) -> &Client<CUB> {
        self.database_client.main_client()
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> CollectionTrait<'a, CUB> for CollectionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.database_client.database_name()
    }

    fn collection_name(&self) -> &'a dyn CollectionName {
        self.collection_name
    }

    fn get_collection(&self) -> requests::GetCollectionBuilder<'_, CUB> {
        requests::GetCollectionBuilder::new(self)
    }

    fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_, CUB> {
        requests::DeleteCollectionBuilder::new(self)
    }

    fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, CUB, No, No> {
        requests::ReplaceCollectionBuilder::new(self)
    }

    fn list_documents(&self) -> requests::ListDocumentsBuilder<'_, '_, CUB> {
        requests::ListDocumentsBuilder::new(self)
    }

    fn create_document<T>(&self) -> requests::CreateDocumentBuilder<'_, '_, T, CUB, No, No>
    where
        T: Serialize,
    {
        requests::CreateDocumentBuilder::new(self)
    }

    fn replace_document<T>(&self) -> requests::ReplaceDocumentBuilder<'_, '_, T, CUB, No, No, No>
    where
        T: Serialize,
    {
        requests::ReplaceDocumentBuilder::new(self)
    }

    fn query_documents(&self) -> requests::QueryDocumentsBuilder<'_, '_, CUB, No> {
        requests::QueryDocumentsBuilder::new(self)
    }

    fn with_stored_procedure<'c>(
        &'c self,
        stored_procedure_name: &'c dyn StoredProcedureName,
    ) -> StoredProcedureClient<'c, CUB> {
        StoredProcedureClient::new(&self, stored_procedure_name)
    }

    fn with_user_defined_function<'c>(
        &'c self,
        user_defined_function_name: &'c dyn UserDefinedFunctionName,
    ) -> UserDefinedFunctionClient<'c, CUB> {
        UserDefinedFunctionClient::new(&self, user_defined_function_name)
    }

    fn with_trigger<'c>(&'c self, trigger_name: &'c dyn TriggerName) -> TriggerClient<'c, CUB> {
        TriggerClient::new(&self, trigger_name)
    }

    fn list_stored_procedures(&self) -> requests::ListStoredProceduresBuilder<'_, CUB> {
        requests::ListStoredProceduresBuilder::new(self)
    }

    fn list_user_defined_functions(
        &self,
    ) -> requests::ListUserDefinedFunctionsBuilder<'_, '_, CUB> {
        requests::ListUserDefinedFunctionsBuilder::new(self)
    }

    fn list_triggers(&self) -> requests::ListTriggersBuilder<'_, '_, CUB> {
        requests::ListTriggersBuilder::new(self)
    }

    fn get_partition_key_ranges(&self) -> requests::GetPartitionKeyRangesBuilder<'_, '_, CUB> {
        requests::GetPartitionKeyRangesBuilder::new(self)
    }

    fn with_document<'c>(
        &'c self,
        document_name: &'c dyn DocumentName,
        partition_keys: &'c PartitionKeys,
    ) -> DocumentClient<'c, CUB> {
        DocumentClient::new(&self, document_name, partition_keys)
    }
}

impl<'a, CUB> CollectionBuilderTrait<'a, CUB> for CollectionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}",
                self.database_name().name(),
                self.collection_name().name()
            ),
            method,
            ResourceType::Collections,
        )
    }
}
