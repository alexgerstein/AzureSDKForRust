use crate::clients::{CollectionClient, CosmosUriBuilder, ResourceType};
use crate::document::Document;
use crate::prelude::*;
use crate::responses::ReplaceDocumentResponse;
use crate::CollectionClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use azure_sdk_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::Serialize;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    p_document: PhantomData<DocumentSet>,
    p_partition_keys: PhantomData<PartitionKeysSet>,
    p_document_id: PhantomData<DocumentIdSet>,
    document: Option<&'b Document<T>>,
    partition_keys: Option<&'b PartitionKeys>,
    document_id: Option<&'b str>,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    allow_tentative_writes: bool,
}

impl<'a, 'b, T, CUB> ReplaceDocumentBuilder<'a, 'b, T, CUB, No, No, No>
where
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> ReplaceDocumentBuilder<'a, 'b, T, CUB, No, No, No> {
        ReplaceDocumentBuilder {
            collection_client,
            p_document: PhantomData {},
            document: None,
            p_partition_keys: PhantomData {},
            partition_keys: None,
            p_document_id: PhantomData {},
            document_id: None,
            indexing_directive: IndexingDirective::Default,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: false,
        }
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> CollectionClientRequired<'a, CUB>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, T, CUB, PartitionKeysSet, DocumentIdSet> DocumentRequired<'b, T>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, Yes, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn document(&self) -> &'b Document<T> {
        self.document.unwrap()
    }
}

impl<'a, 'b, T, CUB, DocumentSet, DocumentIdSet> PartitionKeysRequired<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, Yes, DocumentIdSet>
where
    DocumentSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_keys(&self) -> &'b PartitionKeys {
        self.partition_keys.unwrap()
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet> DocumentIdRequired<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, Yes>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn document_id(&self) -> &'b str {
        self.document_id.unwrap()
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> IndexingDirectiveOption
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn indexing_directive(&self) -> IndexingDirective {
        self.indexing_directive
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> IfMatchConditionOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> IfModifiedSinceOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> UserAgentOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> ActivityIdOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> ConsistencyLevelOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> AllowTentativeWritesOption
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn allow_tentative_writes(&self) -> bool {
        self.allow_tentative_writes
    }
}

impl<'a, 'b, T, CUB, PartitionKeysSet, DocumentIdSet> DocumentSupport<'b, T>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, No, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceDocumentBuilder<'a, 'b, T, CUB, Yes, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_document(self, document: &'b Document<T>) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            document: Some(document),
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, T, CUB, DocumentSet, DocumentIdSet> PartitionKeysSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, No, DocumentIdSet>
where
    DocumentSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, Yes, DocumentIdSet>;

    #[inline]
    fn with_partition_keys(self, partition_keys: &'b PartitionKeys) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            document: self.document,
            partition_keys: Some(partition_keys),
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet> DocumentIdSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, No>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, Yes>;

    #[inline]
    fn with_document_id(self, document_id: &'b str) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            document: self.document,
            partition_keys: self.partition_keys,
            document_id: Some(document_id),
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> IndexingDirectiveSupport
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_indexing_directive(self, indexing_directive: IndexingDirective) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            document: self.document,
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> IfMatchConditionSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            document: self.document,
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: Some(if_match_condition),
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> IfModifiedSinceSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            document: self.document,
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: Some(if_modified_since),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> UserAgentSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            document: self.document,
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> ActivityIdSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            document: self.document,
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> ConsistencyLevelSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            document: self.document,
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet> AllowTentativeWritesSupport
    for ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>
where
    DocumentSet: ToAssign,
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceDocumentBuilder<'a, 'b, T, CUB, DocumentSet, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            document: self.document,
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, T, CUB> ReplaceDocumentBuilder<'a, 'b, T, CUB, Yes, Yes, Yes>
where
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<ReplaceDocumentResponse, AzureError> {
        trace!("ReplaceDocumentBuilder::execute() called");

        let mut req = self.collection_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}",
                self.collection_client.database_name().name(),
                self.collection_client.collection_name().name(),
                self.document_id()
            ),
            hyper::Method::PUT,
            ResourceType::Documents,
        );

        // add trait headers
        req = IndexingDirectiveOption::add_header(self, req);
        req = IfMatchConditionOption::add_header(self, req);
        req = IfModifiedSinceOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);
        req = PartitionKeysRequired::add_header(self, req);
        req = AllowTentativeWritesOption::add_header(self, req);

        let serialized = serde_json::to_string(self.document())?;

        let req = req.body(hyper::Body::from(serialized))?;
        debug!("request == {:#?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.collection_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        (&headers, &body as &[u8]).try_into()
    }
}
