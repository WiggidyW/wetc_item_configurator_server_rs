use crate::error::CreateAccessorError;

use eve_item_configurator_firestore_accessor::{
    Client as FirestoreAccessor,
    TokenSourceType,
};
use eve_item_configurator_gcstorage_accessor::{
    Client as GCStorageAccessor,
    PredefinedObjectAcl,
};
use eve_item_configurator_sqlite_accessor::{
    Client as SqliteAccessor,
    SqlitePoolOptions,
};

pub struct Accessor {
    pub sqlite_accessor: SqliteAccessor,
    pub firestore_accessor: FirestoreAccessor,
    pub gcstorage_accessor: GCStorageAccessor,
}

impl Accessor {
    pub async fn new<S: AsRef<str>>(
        gcstorage_creds: impl AsRef<[u8]>,
        gcstorage_bucket: String,
        gcstorage_json_acl: Option<PredefinedObjectAcl>,
        gcstorage_item_acl: Option<PredefinedObjectAcl>,
        sqlite_url: impl AsRef<str>,
        sqlite_pool_options: Option<SqlitePoolOptions>,
        firestore_collection_path: &[S],
        firestore_google_project_id: String,
        firestore_max_retries: Option<usize>,
        firestore_firebase_api_url: Option<String>,
        firestore_token_scopes: Option<Vec<String>>,
        firestore_token_source_type: Option<TokenSourceType>,
    ) -> Result<Self, CreateAccessorError> {
        let gcstorage_fut = GCStorageAccessor::new(
            gcstorage_creds,
            gcstorage_bucket,
            gcstorage_json_acl,
            gcstorage_item_acl,
        );
        let firestore_fut = FirestoreAccessor::new(
            firestore_collection_path,
            firestore_google_project_id,
            firestore_max_retries,
            firestore_firebase_api_url,
            firestore_token_scopes,
            firestore_token_source_type,
        );
        let sqlite_fut = SqliteAccessor::new(
            sqlite_url,
            sqlite_pool_options,
        );
        Ok(Self {
            gcstorage_accessor: gcstorage_fut.await?,
            firestore_accessor: firestore_fut.await?,
            sqlite_accessor: sqlite_fut.await?,
        })
    }
}
