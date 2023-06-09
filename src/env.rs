use crate::error::{EnvError, EnvErrorInner};

use eve_item_configurator_gcstorage_accessor::PredefinedObjectAcl;
use eve_item_configurator_firestore_accessor::TokenSourceType;
use eve_item_configurator_sqlite_accessor::SqlitePoolOptions;

use std::{
    num::ParseIntError,
    time::Duration,
    str::FromStr,
    env::var,
    net::SocketAddr,
};

pub fn esi_client_id() -> Result<String, EnvError> {
    env_get_assert("ESI_CLIENT_ID")
}

pub fn service_address() -> Result<SocketAddr, EnvError> {
    env_get_assert("SERVICE_ADDRESS").and_then(|s| {
        s.parse().map_err(|e| {
            EnvError::new(
                "SERVICE_ADDRESS",
                EnvErrorInner::InvalidSocketAddrError(e),
            )
        })
    })
}

pub fn gcstorage_creds() -> Result<String, EnvError> {
    env_get_assert("GCSTORAGE_CREDS")
}

pub fn gcstorage_bucket() -> Result<String, EnvError> {
    env_get_assert("GCSTORAGE_BUCKET")
}

pub fn gcstorage_json_acl() -> Result<Option<PredefinedObjectAcl>, EnvError> {
    flatten(env_get_option("GCSTORAGE_JSON_ACL")
        .map(|opt| opt.map(|s| {
            predefined_object_acl_from_str("GCSTORAGE_JSON_ACL", s)
        }))
    )
}

pub fn gcstorage_item_acl() -> Result<Option<PredefinedObjectAcl>, EnvError> {
    flatten(env_get_option("GCSTORAGE_ITEM_ACL")
        .map(|opt| opt.map(|s| {
            predefined_object_acl_from_str("GCSTORAGE_ITEM_ACL", s)
        }))
    )
}

pub fn sqlite_url() -> Result<String, EnvError> {
    env_get_assert("SQLITE_URL")
}

pub fn sqlite_pool_options() -> Result<Option<SqlitePoolOptions>, EnvError> {
    let mut sqlite_pool_options = SqlitePoolOptions::new();
    if let Some(s) = env_get_option("SQLITE_MAX_CONNECTIONS")? {
        sqlite_pool_options = sqlite_pool_options.max_connections(
            number_from_str("SQLITE_MAX_CONNECTIONS", s)?
        );
    }
    if let Some(s) = env_get_option("SQLITE_MIN_CONNECTIONS")? {
        sqlite_pool_options = sqlite_pool_options.min_connections(
            number_from_str("SQLITE_MIN_CONNECTIONS", s)?
        );
    }
    if let Some(s) = env_get_option("SQLITE_ACQUIRE_TIMEOUT")? {
        sqlite_pool_options = sqlite_pool_options.acquire_timeout(
            Duration::from_secs(
                number_from_str("SQLITE_CONNECTION_TIMEOUT", s)?
            )
        );
    }
    if let Some(s) = env_get_option("SQLITE_MAX_LIFETIME")? {
        sqlite_pool_options = sqlite_pool_options.max_lifetime(
            match number_from_str("SQLITE_MAX_LIFETIME", s)? {
                0 => None,
                n => Some(Duration::from_secs(n)),
            }
        );
    }
    if let Some(s) = env_get_option("SQLITE_IDLE_TIMEOUT")? {
        sqlite_pool_options = sqlite_pool_options.idle_timeout(
            match number_from_str("SQLITE_IDLE_TIMEOUT", s)? {
                0 => None,
                n => Some(Duration::from_secs(n)),
            }
        );
    }
    if let Some(s) = env_get_option("SQLITE_TEST_BEFORE_ACQUIRE")? {
        sqlite_pool_options = sqlite_pool_options.test_before_acquire(
            bool_from_str("SQLITE_TEST_BEFORE_ACQUIRE", s)?
        );
    }
    Ok(Some(sqlite_pool_options))
}

pub fn firestore_collection_path() -> Result<Vec<String>, EnvError> {
    env_get_assert("FIRESTORE_COLLECTION_PATH").map(|s| {
        s.split('/').map(|s| s.to_owned()).collect()
    })
}

pub fn firestore_project_id() -> Result<String, EnvError> {
    env_get_assert("FIRESTORE_PROJECT_ID")
}

pub fn firestore_max_retries() -> Result<Option<usize>, EnvError> {
    flatten(env_get_option("FIRESTORE_MAX_RETRIES")
        .map(|opt| opt
            .map(|s| {
                number_from_str("FIRESTORE_MAX_RETRIES", s)
            })
        )
    )
}

pub fn firestore_scopes() -> Result<Option<Vec<String>>, EnvError> {
    env_get_option("FIRESTORE_SCOPES").map(|opt| opt.map(|s| {
        s.split(',').map(|s| s.trim().to_owned()).collect()
    }))
}

pub fn firestore_creds() -> Result<Option<TokenSourceType>, EnvError> {
    env_get_option("FIRESTORE_CREDS").map(|opt| opt.map(|s| {
        TokenSourceType::Json(s)
    }))
}

fn env_get_assert(key: &'static str) -> Result<String, EnvError> {
    var(key).map_err(|e| {
        EnvError::new(key, EnvErrorInner::MissingOrInvalidError(e))
    })
}

fn env_get_option(key: &'static str) -> Result<Option<String>, EnvError> {
    match var(key) {
        Ok(s) => Ok(Some(s)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(e) => Err(
            EnvError::new(key, EnvErrorInner::MissingOrInvalidError(e))
        ),
    }
}

fn bool_from_str(key: &'static str, s: String) -> Result<bool, EnvError> {
    match s.as_str() {
        "true" | "True" => Ok(true),
        "false" | "False" => Ok(false),
        _ => Err(EnvError::new(key, EnvErrorInner::InvalidBoolError(s))),
    }
}

fn number_from_str<I: FromStr<Err = ParseIntError>>(
    key: &'static str,
    s: String,
) -> Result<I, EnvError> {
    s.parse::<I>().map_err(|_| {
        EnvError::new(key, EnvErrorInner::InvalidNumberError(s))
    })
}

fn predefined_object_acl_from_str(
    key: &'static str,
    s: String,
) -> Result<PredefinedObjectAcl, EnvError> {
    match s.as_str() {
        "BucketOwnerFullControl" => Ok(PredefinedObjectAcl::BucketOwnerFullControl),
        "AuthenticatedRead" => Ok(PredefinedObjectAcl::AuthenticatedRead),
        "BucketOwnerRead" => Ok(PredefinedObjectAcl::BucketOwnerRead),
        "ProjectPrivate" => Ok(PredefinedObjectAcl::ProjectPrivate),
        "PublicRead" => Ok(PredefinedObjectAcl::PublicRead),
        "Private" => Ok(PredefinedObjectAcl::Private),
        _ => Err(EnvError::new(key, EnvErrorInner::InvalidObjectAclError(s))),
    }
}

fn flatten<T>(any: Result<Option<Result<T, EnvError>>, EnvError>) -> Result<Option<T>, EnvError> {
    match any {
        Ok(Some(Ok(t))) => Ok(Some(t)),
        Ok(Some(Err(e))) => Err(e),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}
