#[derive(Debug)]
pub enum EnvErrorInner {
    MissingOrInvalidError(std::env::VarError),
    InvalidObjectAclError(String),
    InvalidNumberError(String),
    InvalidBoolError(String),
    InvalidSocketAddrError(std::net::AddrParseError),
}

#[derive(Debug)]
pub struct EnvError {
    pub key: &'static str,
    pub inner: EnvErrorInner,
}

impl EnvError {
    pub fn new(key: &'static str, inner: EnvErrorInner) -> Self {
        EnvError { key, inner }
    }
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            EnvErrorInner::MissingOrInvalidError(err) => {
                write!(f, "Missing or invalid environment variable: {}", err)
            },
            EnvErrorInner::InvalidObjectAclError(err) => {
                write!(f, "Invalid object ACL: {}: {}", self.key, err)
            },
            EnvErrorInner::InvalidNumberError(err) => {
                write!(f, "Invalid number: {}: {}", self.key, err)
            },
            EnvErrorInner::InvalidBoolError(err) => {
                write!(f, "Invalid boolean: {}: {}", self.key, err)
            },
            EnvErrorInner::InvalidSocketAddrError(err) => {
                write!(f, "Invalid socket address: {}: {}", self.key, err)
            },
        }
    }
}

impl std::error::Error for EnvError {}

#[derive(Debug)]
pub enum CreateAccessorError {
    SqliteError(eve_item_configurator_sqlite_accessor::Error),
    FirestoreError(eve_item_configurator_firestore_accessor::Error),
    GCStorageError(eve_item_configurator_gcstorage_accessor::Error),
}

impl std::fmt::Display for CreateAccessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAccessorError::SqliteError(err) => {
                write!(f, "SqliteError: {}", err)
            },
            CreateAccessorError::FirestoreError(err) => {
                write!(f, "FirestoreError: {}", err)
            },
            CreateAccessorError::GCStorageError(err) => {
                write!(f, "GCStorageError: {}", err)
            },
        }
    }
}

impl std::error::Error for CreateAccessorError {}

impl From<eve_item_configurator_sqlite_accessor::Error> for CreateAccessorError {
    fn from(err: eve_item_configurator_sqlite_accessor::Error) -> Self {
        CreateAccessorError::SqliteError(err)
    }
}

impl From<eve_item_configurator_firestore_accessor::Error> for CreateAccessorError {
    fn from(err: eve_item_configurator_firestore_accessor::Error) -> Self {
        CreateAccessorError::FirestoreError(err)
    }
}

impl From<eve_item_configurator_gcstorage_accessor::Error> for CreateAccessorError {
    fn from(err: eve_item_configurator_gcstorage_accessor::Error) -> Self {
        CreateAccessorError::GCStorageError(err)
    }
}

#[derive(Debug)]
pub enum Error {
    CreateAccessorError(CreateAccessorError),
    EnvError(EnvError),
    GRPCServerError(eve_item_configurator_server::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CreateAccessorError(err) => {
                write!(f, "CreateAccessorError: {}", err)
            },
            Error::EnvError(err) => {
                write!(f, "EnvError: {}", err)
            },
            Error::GRPCServerError(err) => {
                write!(f, "GRPCServerError: {}", err)
            },
        }
    }
}

impl std::error::Error for Error {}

impl From<CreateAccessorError> for Error {
    fn from(err: CreateAccessorError) -> Self {
        Error::CreateAccessorError(err)
    }
}

impl From<EnvError> for Error {
    fn from(err: EnvError) -> Self {
        Error::EnvError(err)
    }
}

impl From<eve_item_configurator_server::Error> for Error {
    fn from(err: eve_item_configurator_server::Error) -> Self {
        Error::GRPCServerError(err)
    }
}
