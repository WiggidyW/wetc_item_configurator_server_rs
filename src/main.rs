mod accessor_impl;
mod accessor;
mod error;
mod env;

//
// Required ENV Variables:
//     ESI_CLIENT_ID - String
//     SERVICE_ADDRESS - Address:Port
//     GCSTORAGE_CREDS - Json
//     GCSTORAGE_BUCKET - String
//     FIRESTORE_COLLECTION_PATH - '/' Separated Path
//     FIRESTORE_PROJECT_ID - String
//     SQLITE_URL - String
//
// Optional ENV Variables:
//     GCSTORAGE_JSON_ACL - String
//     GCSTORAGE_ITEM_ACL - String
//     FIRESTORE_MAX_RETRIES - Integer
//     FIRESTORE_SCOPES - ',' Separated List of Scopes
//     FIRESTORE_CREDS - Json
//     SQLITE_MAX_CONNECTIONS - Integer
//     SQLITE_MAX_CONNECTIONS - Integer
//     SQLITE_MIN_CONNECTIONS - Integer
//     SQLITE_ACQUIRE_TIMEOUT - Integer
//     SQLITE_MAX_LIFETIME - Duration
//     SQLITE_IDLE_TIMEOUT - Duration
//     SQLITE_TEST_BEFORE_ACQUIRE - Boolean
//

use accessor::Accessor;
use error::Error;

use eve_item_configurator_server::serve;

use tonic::transport::Server as TonicServer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut tonic_server = TonicServer::builder(); // TODO: Build server from ENV
    let accessor = Accessor::new(
        env::gcstorage_creds()?,
        env::gcstorage_bucket()?,
        env::gcstorage_json_acl()?,
        env::gcstorage_item_acl()?,
        env::sqlite_url()?,
        env::sqlite_pool_options()?,
        &env::firestore_collection_path()?,
        env::firestore_project_id()?,
        env::firestore_max_retries()?,
        None,
        env::firestore_scopes()?,
        env::firestore_creds()?,
    )
        .await?;
    serve(
        accessor,
        env::esi_client_id()?,
        env::service_address()?,
        &mut tonic_server,
    )
        .await?;
    Ok(())
}
