use crate::accessor::Accessor;

use eve_item_configurator_firestore_accessor::Error as FirestoreError;
use eve_item_configurator_gcstorage_accessor::Error as GCStorageError;
use eve_item_configurator_sqlite_accessor::Error as SqliteError;
use eve_item_configurator_server::{
    Accessor as AccessorTrait,
    TypeIdGetter,
    NameGetter,
    MarketGroupGetter,
    GroupGetter,
    CategoryGetter,
    JsonGetter,
    JsonSetter,
    ItemGetter,
    ItemSetter,
    CharacterGetter,
    CharacterSetter,
    async_trait,
    DivisionNames,
    TypeId,
};

use std::collections::HashMap;

#[async_trait]
impl TypeIdGetter for Accessor {
    type Error = SqliteError;
    async fn get_type_ids(&self) -> Result<Vec<TypeId>, Self::Error> {
        self.sqlite_accessor.get_type_ids().await
    }
}

#[async_trait]
impl NameGetter for Accessor {
    type Error = SqliteError;
    async fn get_names(
        &self,
        language: &str,
    ) -> Result<Vec<String>, Self::Error> {
        self.sqlite_accessor.get_names(language).await
    }
}

#[async_trait]
impl MarketGroupGetter for Accessor {
    type Error = SqliteError;
    async fn get_market_groups(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Self::Error> {
        self.sqlite_accessor.get_market_groups(language).await
    }
}

#[async_trait]
impl GroupGetter for Accessor {
    type Error = SqliteError;
    async fn get_groups(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Self::Error> {
        self.sqlite_accessor.get_groups(language).await
    }
}

#[async_trait]
impl CategoryGetter for Accessor {
    type Error = SqliteError;
    async fn get_categories(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Self::Error> {
        self.sqlite_accessor.get_categories(language).await
    }
}

#[async_trait]
impl JsonGetter for Accessor {
    type Error = GCStorageError;
    async fn get_json(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Self::Error> {
        self.gcstorage_accessor.get_json(config_name).await
    }
}

#[async_trait]
impl JsonSetter for Accessor {
    type Error = GCStorageError;
    async fn set_json(
        &self,
        config_name: &str,
        json: Vec<String>,
    ) -> Result<(), Self::Error> {
        self.gcstorage_accessor.set_json(config_name, json).await
    }
}

#[async_trait]
impl ItemGetter for Accessor {
    type Error = GCStorageError;
    async fn get_items(
        &self,
        config_name: &str,
    ) -> Result<HashMap<TypeId, HashMap<String, u32>>, Self::Error> {
        self.gcstorage_accessor.get_items(config_name).await
    }
}

#[async_trait]
impl ItemSetter for Accessor {
    type Error = GCStorageError;
    async fn set_items(
        &self,
        config_name: &str,
        items: HashMap<TypeId, HashMap<String, u32>>,
    ) -> Result<(), Self::Error> {
        self.gcstorage_accessor.set_items(config_name, items).await
    }
}

#[async_trait]
impl CharacterGetter for Accessor {
    type Error = FirestoreError;
    async fn get_characters(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Self::Error> {
        self.firestore_accessor.get_characters(config_name).await
    }
}

#[async_trait]
impl CharacterSetter for Accessor {
    type Error = FirestoreError;
    async fn set_characters(
        &self,
        config_name: &str,
        characters: Vec<String>
    ) -> Result<(), Self::Error> {
        self.firestore_accessor.set_characters(config_name, characters).await
    }
}

impl AccessorTrait for Accessor {}
