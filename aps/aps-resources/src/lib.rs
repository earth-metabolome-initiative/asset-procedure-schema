#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_resources;
pub mod asset_models_model {
    #[derive(
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Asset models describe kinds of things (a tube model, a balance model, a
    /// plant species), assets are the tracked things themselves. Both are
    /// connetto ownables living in one project, whose name is unique within
    /// that project.
    #[table_model(ancestors(aps_resources::resources, aps_resources::ownables))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::TableName, foreign_key = table_name_id))]
    #[table_model(foreign_key((id), (::aps_resources::ownables::id)))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[table_model(foreign_key((parent_model_id), (asset_models::id)))]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = asset_models)]
    pub struct AssetModel {
        /// Identifier of the connetto ownable this model extends.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_resources::ownables::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// connetto project holding the model.
        #[same_as(aps_resources::ownables::project_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        project_id: ::rosetta_uuid::Uuid,
        /// Most concrete APS table holding this model.
        #[infallible]
        table_name_id: String,
        /// Name, unique within the project.
        name: String,
        /// Free-text description.
        description: Option<String>,
        /// More general model this one specialises. Immutable, so the rows of
        /// `asset_model_ancestors` never go stale.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_model_id: Option<::rosetta_uuid::Uuid>,
    }
    ::diesel_builders::prelude::unique_index!(asset_models::project_id, asset_models::name);
    ::diesel_builders::prelude::unique_index!(asset_models::id, asset_models::project_id);
    ::diesel_builders::prelude::unique_index!(asset_models::id, asset_models::parent_model_id);
    ::diesel_builders::prelude::unique_index!(
        asset_models::id,
        asset_models::parent_model_id,
        asset_models::organization_id
    );
    impl ::diesel_builders::ValidateColumn<asset_models::name>
        for <asset_models::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_models::name::NAME,
                ));
            }
            if name.chars().count() > 255usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_models::name::NAME,
                    255usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<asset_models::description>
        for <asset_models::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(description: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if description.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_models::description::NAME,
                ));
            }
            if description.chars().count() > 8192usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_models::description::NAME,
                    8192usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<asset_models::table>
        for <asset_models::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_model_id), Some(id)) = (
                <Self as diesel_builders::MayGetColumn<
                    asset_models::parent_model_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    asset_models::id,
                >>::may_get_column_ref(self),
            )
                && parent_model_id
                    .as_ref()
                    .is_some_and(|parent_model_id| parent_model_id == id)
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::asset_models::parent_model_id::NAME,
                        crate::asset_models::id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for AssetModel {
        fn get_column_ref(
            &self,
        ) -> &<asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for AssetModel {
        fn get_column_ref(
            &self,
        ) -> &<asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(asset_models, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(asset_models, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(asset_models, ::aps_resources::table_names);
}
pub use asset_models_model::*;
pub mod brands_model {
    #[derive(
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Project-scoped brand catalog.
    #[table_model(ancestors(aps_resources::resources, aps_resources::ownables))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::TableName, foreign_key = table_name_id))]
    #[table_model(foreign_key((id), (::aps_resources::ownables::id)))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = brands)]
    pub struct Brand {
        /// Resource identity.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_resources::ownables::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Project containing this ownable.
        #[same_as(aps_resources::ownables::project_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        project_id: ::rosetta_uuid::Uuid,
        /// Concrete APS table holding this row.
        table_name_id: String,
        /// Brand display name.
        name: String,
    }
    ::diesel_builders::prelude::unique_index!(brands::project_id, brands::name);
    impl ::diesel_builders::ValidateColumn<brands::table_name_id>
        for <brands::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(table_name_id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if table_name_id.as_str() != "brands" {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::brands::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::brands::table_name_id::NAME],
                    "table_name_id = 'brands'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<brands::name>
        for <brands::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::brands::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::brands::name::NAME,
                ));
            }
            if name.chars().count() > 255usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::brands::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::brands::name::NAME,
                    255usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for Brand {
        fn get_column_ref(&self) -> &<brands::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Brand {
        fn get_column_ref(&self) -> &<brands::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(brands, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(brands, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(brands, ::aps_resources::table_names);
}
pub use brands_model::*;
pub mod digital_asset_models_model {
    #[derive(
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::AssetModel, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::asset_models::id)))]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = digital_asset_models)]
    pub struct DigitalAssetModel {
        /// Identifier of the asset model this row specialises.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_resources::asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Media type of the files this model describes.
        mime_type: String,
    }
    ::diesel_builders::prelude::unique_index!(
        digital_asset_models::id,
        digital_asset_models::mime_type,
        digital_asset_models::organization_id
    );
    impl ::diesel_builders::ValidateColumn<digital_asset_models::mime_type>
        for <digital_asset_models::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(mime_type: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if mime_type.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::digital_asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::digital_asset_models::mime_type::NAME,
                ));
            }
            if mime_type.chars().count() > 255usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::digital_asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::digital_asset_models::mime_type::NAME,
                    255usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for DigitalAssetModel {
        fn get_column_ref(
            &self,
        ) -> &<digital_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for DigitalAssetModel {
        fn get_column_ref(
            &self,
        ) -> &<digital_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for DigitalAssetModel {
        fn get_column_ref(
            &self,
        ) -> &<digital_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `digital_asset_models` the procedure runner renders.
    pub const DIGITAL_ASSET_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "digital_asset_models",
            asset_table_name: "digital_assets",
            model_column: "digital_asset_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "connetto organization owning the row.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "digital_asset_model_id",
                    label: "Model of the asset, which must be a digital asset model.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        digital_asset_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        digital_asset_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        digital_asset_models,
        ::aps_resources::resources
    );
}
pub use digital_asset_models_model::*;
pub mod grants_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel_builders::prelude::TableModel,
    )]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(primary_key(resource_id, grantee_id))]
    #[table_model(
        foreign_key(
            (organization_id, resource_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[table_model(
        foreign_key(
            (grantee_id, grantee_kind),
            (::aps_grantees::grantees::id, ::aps_grantees::grantees::kind)
        )
    )]
    #[diesel(table_name = grants)]
    pub struct Grant {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        resource_id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        grantee_id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::aps_authz_grantee_kind::sql_types::GranteeKind)]
        grantee_kind: ::aps_authz_grantee_kind::GranteeKind,
        #[infallible]
        #[diesel(sql_type = ::aps_authz_role::sql_types::Role)]
        role: ::aps_authz_role::Role,
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        expires_at: Option<::rosetta_utc::TimestampUTC>,
    }
    impl ::diesel_builders::ValidateRecord<grants::table>
        for <grants::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(grantee_kind), Some(role)) = (
                <Self as diesel_builders::MayGetColumn<grants::grantee_kind>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<grants::role>>::may_get_column_ref(self),
            ) && !((((*grantee_kind != ::aps_authz_grantee_kind::GranteeKind::Everyone)
                && (*grantee_kind != ::aps_authz_grantee_kind::GranteeKind::SignedIn))
                && (*grantee_kind != ::aps_authz_grantee_kind::GranteeKind::Link))
                || (*role < ::aps_authz_role::Role::Admin))
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::grants::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::grants::grantee_kind::NAME, crate::grants::role::NAME],
                    "grantee_kind NOT IN ('everyone', 'signed_in', 'link') OR role < 'admin'",
                ));
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(grants, ::aps_grantees::grantees);
    ::diesel::allow_tables_to_appear_in_same_query!(grants, ::aps_resources::resources);
}
pub use grants_model::*;
pub mod ownables_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    #[table_model(ancestors(aps_resources::resources))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_grantees::User, foreign_key = created_by_id))]
    #[table_model(foreign_key((created_by_id), (::aps_grantees::users::id)))]
    #[table_model(
        foreign_key(
            (project_id, project_kind),
            (::aps_resources::resources::id, ::aps_resources::resources::kind)
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, project_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = ownables)]
    pub struct Ownable {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[same_as(aps_resources::resources::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        #[table_model(generated)]
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        kind: ::aps_authz_resource_kind::ResourceKind,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        project_id: ::rosetta_uuid::Uuid,
        #[table_model(generated)]
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        project_kind: ::aps_authz_resource_kind::ResourceKind,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        created_by_id: ::rosetta_uuid::Uuid,
    }
    ::diesel_builders::prelude::unique_index!(ownables::id, ownables::project_id);
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Ownable {
        fn get_column_ref(&self) -> &<ownables::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(ownables, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(ownables, ::aps_grantees::users);
}
pub use ownables_model::*;
pub mod photograph_models_model {
    #[derive(
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Digital models describing image files.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_resources::digital_asset_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_resources::digital_asset_models::id)))]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = photograph_models)]
    pub struct PhotographModel {
        /// Model inherited from the digital model catalog.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_resources::digital_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Image media type pinned to the digital model.
        #[same_as(aps_resources::digital_asset_models::mime_type)]
        mime_type: String,
    }
    impl ::diesel_builders::ValidateColumn<photograph_models::mime_type>
        for <photograph_models::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(mime_type: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if !({
                let text = mime_type.as_str();
                'like: {
                    let mut rest = text.chars();
                    if !(rest.next() == Some('i')
                        && rest.next() == Some('m')
                        && rest.next() == Some('a')
                        && rest.next() == Some('g')
                        && rest.next() == Some('e')
                        && rest.next() == Some('/'))
                    {
                        break 'like false;
                    }
                    true
                }
            }) {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::photograph_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::photograph_models::mime_type::NAME],
                    "mime_type LIKE 'image/%'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for PhotographModel {
        fn get_column_ref(
            &self,
        ) -> &<photograph_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::digital_asset_models::id> for PhotographModel {
        fn get_column_ref(
            &self,
        ) -> &<photograph_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PhotographModel {
        fn get_column_ref(
            &self,
        ) -> &<photograph_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PhotographModel {
        fn get_column_ref(
            &self,
        ) -> &<photograph_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `photograph_models` the procedure runner renders.
    pub const PHOTOGRAPH_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "photograph_models",
            asset_table_name: "photographs",
            model_column: "photograph_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "photograph_model_id",
                    label: "Image model instantiated by this photograph.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_models,
        ::aps_resources::digital_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(photograph_models, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(photograph_models, ::aps_resources::resources);
}
pub use photograph_models_model::*;
pub mod resource_parents_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel_builders::prelude::TableModel,
    )]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(
        foreign_key(
            (organization_id, id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, parent_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[table_model(
        foreign_key(
            (id, resource_kind),
            (::aps_resources::resources::id, ::aps_resources::resources::kind)
        )
    )]
    #[table_model(
        foreign_key(
            (parent_id, parent_kind),
            (::aps_resources::resources::id, ::aps_resources::resources::kind)
        )
    )]
    #[diesel(table_name = resource_parents)]
    pub struct ResourceParent {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        resource_kind: ::aps_authz_resource_kind::ResourceKind,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        parent_kind: ::aps_authz_resource_kind::ResourceKind,
    }
    impl ::diesel_builders::ValidateColumn<resource_parents::resource_kind>
        for <resource_parents::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            resource_kind: &::aps_authz_resource_kind::ResourceKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if !((*resource_kind == ::aps_authz_resource_kind::ResourceKind::Folder)
                || (*resource_kind == ::aps_authz_resource_kind::ResourceKind::Project))
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::resource_parents::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::resource_parents::resource_kind::NAME],
                    "resource_kind IN ('folder', 'project')",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<resource_parents::parent_kind>
        for <resource_parents::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            parent_kind: &::aps_authz_resource_kind::ResourceKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if !((*parent_kind == ::aps_authz_resource_kind::ResourceKind::Organization)
                || (*parent_kind == ::aps_authz_resource_kind::ResourceKind::Folder))
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::resource_parents::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::resource_parents::parent_kind::NAME],
                    "parent_kind IN ('organization', 'folder')",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<resource_parents::table>
        for <resource_parents::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_id), Some(id)) = (
                <Self as diesel_builders::MayGetColumn<
                    resource_parents::parent_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    resource_parents::id,
                >>::may_get_column_ref(self),
            ) && parent_id == id
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::resource_parents::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::resource_parents::parent_id::NAME,
                        crate::resource_parents::id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(resource_parents, ::aps_resources::resources);
}
pub use resource_parents_model::*;
pub mod resources_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_grantees::Organization, foreign_key = organization_id))]
    #[table_model(foreign_key((organization_id), (::aps_grantees::organizations::id)))]
    #[diesel(table_name = resources)]
    pub struct Resource {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        kind: ::aps_authz_resource_kind::ResourceKind,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        archived_at: Option<::rosetta_utc::TimestampUTC>,
        #[table_model(default = false)]
        #[infallible]
        writers_can_share: bool,
    }
    ::diesel_builders::prelude::unique_index!(resources::id, resources::kind);
    ::diesel_builders::prelude::unique_index!(resources::organization_id, resources::id);
    impl ::diesel_builders::ValidateRecord<resources::table>
        for <resources::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(kind), Some(id), Some(organization_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    resources::kind,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    resources::id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    resources::organization_id,
                >>::may_get_column_ref(self),
            )
                && (*kind == ::aps_authz_resource_kind::ResourceKind::Organization)
                    != (*id == *organization_id)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::resources::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::resources::kind::NAME,
                            crate::resources::id::NAME,
                            crate::resources::organization_id::NAME,
                        ],
                        "(kind = 'organization') = (id = organization_id)",
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(resources, ::aps_grantees::organizations);
}
pub use resources_model::*;
pub mod table_name_admitted_parents_model {
    #[derive(
        Clone,
        Default,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Tables whose rows a model of another table may name as its parent model
    /// without extending them, such as the product a lot model belongs to. A
    /// model's parent sits on the model's table, a table it extends, or an
    /// admitted parent table of one of those or a table extending it. Each
    /// slice seeds the rows of the tables it creates.
    #[diesel(primary_key(table_name_id, parent_table_name_id))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[table_model(
        foreign_key((parent_table_name_id), (::aps_resources::table_names::id))
    )]
    #[diesel(table_name = table_name_admitted_parents)]
    pub struct TableNameAdmittedParent {
        /// Table of the model naming the parent.
        table_name_id: String,
        /// Table the parent model may sit on.
        parent_table_name_id: String,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        table_name_admitted_parents,
        ::aps_resources::table_names
    );
}
pub use table_name_admitted_parents_model::*;
pub mod table_name_ancestors_model {
    #[derive(
        Clone,
        Default,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Reflexive transitive closure of the APS table tree, one row for a table
    /// and every table it extends, so a row of a subtype table can be checked
    /// against the `table_name_id` of its root row. Each slice seeds the rows
    /// of the tables it creates.
    #[diesel(primary_key(descendant_table_name_id, ancestor_table_name_id))]
    #[table_model(
        foreign_key((descendant_table_name_id), (::aps_resources::table_names::id))
    )]
    #[table_model(
        foreign_key((ancestor_table_name_id), (::aps_resources::table_names::id))
    )]
    #[diesel(table_name = table_name_ancestors)]
    pub struct TableNameAncestor {
        /// Table whose ancestors the row lists.
        descendant_table_name_id: String,
        /// Table the descendant extends, or the descendant itself.
        ancestor_table_name_id: String,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        table_name_ancestors,
        ::aps_resources::table_names
    );
}
pub use table_name_ancestors_model::*;
pub mod table_name_instances_model {
    #[derive(
        Clone,
        Default,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel_builders::prelude::TableModel,
    )]
    /// The table holding the instances of a model or template table's rows, the
    /// asset table whose model column references the model table and the
    /// procedure table of a procedure template table. Each slice seeds the rows
    /// of the tables it creates.
    #[table_model(ancestors(aps_resources::table_names))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_resources::table_names::id)))]
    #[table_model(
        foreign_key((instance_table_name_id), (::aps_resources::table_names::id))
    )]
    #[diesel(table_name = table_name_instances)]
    pub struct TableNameInstance {
        /// Asset model or procedure template table.
        #[infallible]
        id: String,
        /// Asset or procedure table holding the instances.
        #[infallible]
        instance_table_name_id: String,
    }
    impl ::diesel_builders::GetColumn<aps_resources::table_names::id> for TableNameInstance {
        fn get_column_ref(
            &self,
        ) -> &<table_name_instances::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        table_name_instances,
        ::aps_resources::table_names
    );
}
pub use table_name_instances_model::*;
pub mod table_names_model {
    #[derive(
        Clone,
        Default,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Registry of APS tables, named by the `table_name_id` column of every
    /// root row so a client can resolve the most concrete table holding an
    /// ownable.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(table_name = table_names)]
    pub struct TableName {
        /// Name of the APS table.
        id: String,
    }
    impl ::diesel_builders::ValidateColumn<table_names::id>
        for <table_names::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if id.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::table_names::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::table_names::id::NAME,
                ));
            }
            if id.chars().count() > 63usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::table_names::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::table_names::id::NAME,
                    63usize,
                ));
            }
            Ok(())
        }
    }
}
pub use table_names_model::*;
