#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_resources;
pub mod asset_model_ancestors_model {
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
    /// Reflexive transitive closure of `asset_models.parent_model_id`, written
    /// by the client together with the model. A non-reflexive row names the
    /// descendant's parent, and the two foreign keys through it make every row
    /// a true ancestor by induction. `check_asset_model_ancestors` makes the
    /// set complete at commit.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(primary_key(descendant_model_id, ancestor_model_id))]
    #[table_model(
        foreign_key((descendant_model_id), (::aps_resources::asset_models::id))
    )]
    #[table_model(foreign_key((ancestor_model_id), (::aps_resources::asset_models::id)))]
    #[table_model(
        foreign_key(
            (organization_id, descendant_model_id, archived),
            (
                ::aps_resources::resources::organization_id,
                ::aps_resources::resources::id,
                ::aps_resources::resources::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (descendant_model_id, parent_model_id, organization_id),
            (
                ::aps_resources::asset_models::id,
                ::aps_resources::asset_models::parent_model_id,
                ::aps_resources::asset_models::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (parent_model_id, ancestor_model_id),
            (
                asset_model_ancestors::descendant_model_id,
                asset_model_ancestors::ancestor_model_id
            )
        )
    )]
    #[diesel(table_name = asset_model_ancestors)]
    pub struct AssetModelAncestor {
        /// Model whose ancestors the row lists.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        descendant_model_id: ::rosetta_uuid::Uuid,
        /// Ancestor of the descendant, or the descendant itself.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        ancestor_model_id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the descendant.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Parent of the descendant, through which the ancestor is reached.
        /// Null on the reflexive row.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_model_id: Option<::rosetta_uuid::Uuid>,
        /// Archive state inherited from this resource.
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    impl ::diesel_builders::ValidateRecord<asset_model_ancestors::table>
        for <asset_model_ancestors::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_model_id), Some(descendant_model_id), Some(ancestor_model_id)) =
                (
                    <Self as diesel_builders::MayGetColumn<
                        asset_model_ancestors::parent_model_id,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        asset_model_ancestors::descendant_model_id,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        asset_model_ancestors::ancestor_model_id,
                    >>::may_get_column_ref(self),
                )
                && (*parent_model_id).is_none() != (*descendant_model_id == *ancestor_model_id)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::asset_model_ancestors::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::asset_model_ancestors::parent_model_id::NAME,
                            crate::asset_model_ancestors::descendant_model_id::NAME,
                            crate::asset_model_ancestors::ancestor_model_id::NAME,
                        ],
                        "(parent_model_id IS NULL) = (descendant_model_id = ancestor_model_id)",
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        asset_model_ancestors,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        asset_model_ancestors,
        ::aps_resources::resources
    );
}
pub use asset_model_ancestors_model::*;
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
    /// connetto `resources` of kind `'ownable'` living in one project, whose
    /// name is unique within that project.
    #[table_model(ancestors(aps_resources::resources))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::TableName, foreign_key = table_name_id))]
    #[table_model(foreign_key((id), (::aps_resources::resources::id)))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[table_model(foreign_key((parent_model_id), (asset_models::id)))]
    #[table_model(
        foreign_key(
            (parent_model_id, parent_model_archived),
            (asset_models::id, asset_models::archived)
        )
    )]
    #[diesel(table_name = asset_models)]
    pub struct AssetModel {
        /// Identifier of the connetto `resource` of kind `'ownable'` this model
        /// extends.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_resources::resources::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// connetto project holding the model.
        #[same_as(aps_resources::resources::parent_id)]
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
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::resources::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Resource kind fixed to `'ownable'`.
        #[same_as(aps_resources::resources::kind)]
        #[table_model(default = ::aps_authz_resource_kind::ResourceKind::Ownable)]
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        kind: ::aps_authz_resource_kind::ResourceKind,
        /// Archive state of the row named by `parent_model_id`.
        #[table_model(default = false)]
        #[infallible]
        parent_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(asset_models::project_id, asset_models::name);
    ::diesel_builders::prelude::unique_index!(asset_models::id, asset_models::project_id);
    ::diesel_builders::prelude::unique_index!(asset_models::id, asset_models::parent_model_id);
    ::diesel_builders::prelude::unique_index!(
        asset_models::id,
        asset_models::parent_model_id,
        asset_models::organization_id
    );
    ::diesel_builders::prelude::unique_index!(asset_models::id, asset_models::archived);
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
    impl ::diesel_builders::ValidateColumn<asset_models::kind>
        for <asset_models::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            kind: &::aps_authz_resource_kind::ResourceKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if *kind != ::aps_authz_resource_kind::ResourceKind::Ownable {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::asset_models::kind::NAME],
                    "kind = 'ownable'",
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
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for AssetModel {
        fn get_column_ref(
            &self,
        ) -> &<asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(asset_models, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(asset_models, ::aps_resources::table_names);
}
pub use asset_models_model::*;
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
    #[table_model(ancestors(aps_resources::resources, aps_resources::asset_models))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::AssetModel, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::asset_models::id)))]
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
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::asset_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
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
            (organization_id, resource_id, resource_archived),
            (
                ::aps_resources::resources::organization_id,
                ::aps_resources::resources::id,
                ::aps_resources::resources::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (grantee_id, grantee_kind),
            (::aps_organizations::grantees::id, ::aps_organizations::grantees::kind)
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
        #[table_model(default = false)]
        #[infallible]
        resource_archived: bool,
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
    ::diesel::allow_tables_to_appear_in_same_query!(grants, ::aps_organizations::grantees);
    ::diesel::allow_tables_to_appear_in_same_query!(grants, ::aps_resources::resources);
}
pub use grants_model::*;
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
        aps_resources::asset_models,
        aps_resources::digital_asset_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_resources::digital_asset_models::id)))]
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
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::digital_asset_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(photograph_models::id, photograph_models::archived);
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
    ::diesel::allow_tables_to_appear_in_same_query!(photograph_models, ::aps_resources::resources);
}
pub use photograph_models_model::*;
pub mod procedure_template_slot_bindings_model {
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
    /// Binds a slot of a step's child template to a slot of the parent
    /// template, so the child procedure must use the asset the parent procedure
    /// binds there. Two steps bound to one parent slot share that asset. The
    /// parent slot may narrow the child slot's model, and the pair of roles
    /// must be admitted by `slot_role_pairs`. A parent slot is produced by at
    /// most one child slot and consumed by at most one.
    #[diesel(primary_key(step_id, parent_template_id, child_slot_id))]
    #[table_model(
        foreign_key(
            (step_id, parent_template_id, child_template_id, organization_id),
            (
                ::aps_resources::procedure_template_steps::id,
                ::aps_resources::procedure_template_steps::parent_template_id,
                ::aps_resources::procedure_template_steps::child_template_id,
                ::aps_resources::procedure_template_steps::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (child_slot_id, child_template_id, child_asset_model_id, child_role_id),
            (
                ::aps_resources::procedure_template_slots::id,
                ::aps_resources::procedure_template_slots::procedure_template_id,
                ::aps_resources::procedure_template_slots::asset_model_id,
                ::aps_resources::procedure_template_slots::role_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (
                parent_slot_id,
                parent_template_id,
                parent_asset_model_id,
                parent_role_id,
                organization_id
            ),
            (
                ::aps_resources::procedure_template_slots::id,
                ::aps_resources::procedure_template_slots::procedure_template_id,
                ::aps_resources::procedure_template_slots::asset_model_id,
                ::aps_resources::procedure_template_slots::role_id,
                ::aps_resources::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (parent_asset_model_id, child_asset_model_id),
            (
                ::aps_resources::asset_model_ancestors::descendant_model_id,
                ::aps_resources::asset_model_ancestors::ancestor_model_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (child_role_id, parent_role_id),
            (
                ::aps_resources::slot_role_pairs::child_role_id,
                ::aps_resources::slot_role_pairs::parent_role_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, parent_template_id, archived),
            (
                ::aps_resources::resources::organization_id,
                ::aps_resources::resources::id,
                ::aps_resources::resources::archived
            )
        )
    )]
    #[diesel(table_name = procedure_template_slot_bindings)]
    pub struct ProcedureTemplateSlotBinding {
        /// Step whose child slot is bound.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        step_id: ::rosetta_uuid::Uuid,
        /// Slot of the step's child template.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        child_slot_id: ::rosetta_uuid::Uuid,
        /// Template the step belongs to.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_template_id: ::rosetta_uuid::Uuid,
        /// Template the step runs.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        child_template_id: ::rosetta_uuid::Uuid,
        /// Slot of the parent template that fills the child slot.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_slot_id: ::rosetta_uuid::Uuid,
        /// Model of the child slot.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        child_asset_model_id: ::rosetta_uuid::Uuid,
        /// Model of the parent slot, equal to or below the child slot's.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_asset_model_id: ::rosetta_uuid::Uuid,
        /// Role of the child slot.
        child_role_id: String,
        /// Role of the parent slot.
        parent_role_id: String,
        /// connetto organization owning the parent template.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[table_model(default = false)]
        archived: bool,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slot_bindings,
        ::aps_resources::asset_model_ancestors
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slot_bindings,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slot_bindings,
        ::aps_resources::procedure_template_steps
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slot_bindings,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slot_bindings,
        ::aps_resources::slot_role_pairs
    );
}
pub use procedure_template_slot_bindings_model::*;
pub mod procedure_template_slots_model {
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
    /// A slot is a place in the procedure that assets fill, typed by the most
    /// general admissible asset model and by the role the assets play. A slot
    /// may name another slot of its template whose assets end up holding its
    /// assets, and another whose assets held them until the procedure took them
    /// out.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_resources::ProcedureTemplate, foreign_key = procedure_template_id)
    )]
    #[diesel(belongs_to(aps_resources::SlotRole, foreign_key = role_id))]
    #[table_model(
        foreign_key((procedure_template_id), (::aps_resources::procedure_templates::id))
    )]
    #[table_model(foreign_key((asset_model_id), (::aps_resources::asset_models::id)))]
    #[table_model(foreign_key((role_id), (::aps_resources::slot_roles::id)))]
    #[table_model(
        foreign_key(
            (placed_into_slot_id, procedure_template_id, organization_id),
            (
                procedure_template_slots::id,
                procedure_template_slots::procedure_template_id,
                procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (retrieved_from_slot_id, procedure_template_id, organization_id),
            (
                procedure_template_slots::id,
                procedure_template_slots::procedure_template_id,
                procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (asset_model_id, asset_model_archived),
            (::aps_resources::asset_models::id, ::aps_resources::asset_models::archived)
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, procedure_template_id, archived),
            (
                ::aps_resources::resources::organization_id,
                ::aps_resources::resources::id,
                ::aps_resources::resources::archived
            )
        )
    )]
    #[diesel(table_name = procedure_template_slots)]
    pub struct ProcedureTemplateSlot {
        /// Identifier of the slot.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the template.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Template declaring the slot.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_template_id: ::rosetta_uuid::Uuid,
        /// Name, unique within the template.
        name: String,
        /// Free-text description of what fills the slot.
        description: Option<String>,
        /// Most general model the bound assets may have.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        asset_model_id: ::rosetta_uuid::Uuid,
        /// What the procedure does to the bound assets.
        #[infallible]
        role_id: String,
        /// Fewest assets a completed procedure binds.
        #[table_model(default = 1i32)]
        minimum_count: i32,
        /// Most assets a procedure binds. NULL leaves the number unbounded.
        #[infallible]
        maximum_count: Option<i32>,
        /// Slot whose assets hold this slot's assets once the procedure
        /// completes.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        placed_into_slot_id: Option<::rosetta_uuid::Uuid>,
        /// Slot whose assets held this slot's assets until the procedure took
        /// them out.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        retrieved_from_slot_id: Option<::rosetta_uuid::Uuid>,
        /// Archive state inherited from this resource.
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `asset_model_id`.
        #[table_model(default = false)]
        #[infallible]
        asset_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::name
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::asset_model_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::asset_model_id,
        procedure_template_slots::role_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::asset_model_id,
        procedure_template_slots::role_id,
        procedure_template_slots::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::placed_into_slot_id,
        procedure_template_slots::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::retrieved_from_slot_id,
        procedure_template_slots::organization_id
    );
    impl ::diesel_builders::ValidateColumn<procedure_template_slots::name>
        for <procedure_template_slots::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::name::NAME,
                    ),
                );
            }
            if name.chars().count() > 255usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::name::NAME,
                        255usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_slots::description>
        for <procedure_template_slots::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(description: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if description.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::description::NAME,
                    ),
                );
            }
            if description.chars().count() > 8192usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::description::NAME,
                        8192usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_slots::minimum_count>
        for <procedure_template_slots::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(minimum_count: &i32) -> Result<(), Self::Error> {
            use diesel::Column;
            if minimum_count < &0i32 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::minimum_count::NAME,
                        0f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<procedure_template_slots::table>
        for <procedure_template_slots::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(maximum_count), Some(minimum_count)) =
                (
                    <Self as diesel_builders::MayGetColumn<
                        procedure_template_slots::maximum_count,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        procedure_template_slots::minimum_count,
                    >>::may_get_column_ref(self),
                )
            {
                if (*maximum_count).zip(Some(1i32)).map(|(left, right)| left >= right)
                    == Some(false)
                {
                    return Err(
                        ::validation_errors::ValidationError::check_violated(
                            <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                            &[crate::procedure_template_slots::maximum_count::NAME],
                            "maximum_count >= 1",
                        ),
                    );
                }
                if maximum_count.as_ref().is_some_and(|maximum_count| maximum_count < minimum_count)
                {
                    return Err(
                        ::validation_errors::ValidationError::greater_than(
                            <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                            crate::procedure_template_slots::maximum_count::NAME,
                            crate::procedure_template_slots::minimum_count::NAME,
                        ),
                    );
                }
            }
            if let (Some(placed_into_slot_id), Some(id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_slots::placed_into_slot_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_slots::id,
                >>::may_get_column_ref(self),
            )
                && placed_into_slot_id
                    .as_ref()
                    .is_some_and(|placed_into_slot_id| placed_into_slot_id == id)
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::placed_into_slot_id::NAME,
                        crate::procedure_template_slots::id::NAME,
                    ),
                );
            }
            if let (Some(retrieved_from_slot_id), Some(id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_slots::retrieved_from_slot_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_slots::id,
                >>::may_get_column_ref(self),
            )
                && retrieved_from_slot_id
                    .as_ref()
                    .is_some_and(|retrieved_from_slot_id| retrieved_from_slot_id == id)
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::retrieved_from_slot_id::NAME,
                        crate::procedure_template_slots::id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slots,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slots,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slots,
        ::aps_resources::slot_roles
    );
}
pub use procedure_template_slots_model::*;
pub mod procedure_template_states_model {
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
    /// Lifecycle of a procedure template. Only a `draft` changes, a `published`
    /// template may be executed and a `deprecated` one is kept for history.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(table_name = procedure_template_states)]
    pub struct ProcedureTemplateState {
        /// State name.
        id: String,
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_states::id>
        for <procedure_template_states::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if id.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_states::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_states::id::NAME,
                    ),
                );
            }
            if id.chars().count() > 63usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_states::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_states::id::NAME,
                        63usize,
                    ),
                );
            }
            Ok(())
        }
    }
}
pub use procedure_template_states_model::*;
pub mod procedure_template_step_edges_model {
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
    /// The successor step starts only after the predecessor completes. A
    /// predecessor sits at a strictly lower position, so the steps of a parent
    /// form a DAG without a recursive check.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(primary_key(predecessor_step_id, parent_template_id, successor_step_id))]
    #[table_model(
        foreign_key(
            (
                predecessor_step_id,
                parent_template_id,
                predecessor_position,
                organization_id
            ),
            (
                ::aps_resources::procedure_template_steps::id,
                ::aps_resources::procedure_template_steps::parent_template_id,
                ::aps_resources::procedure_template_steps::position,
                ::aps_resources::procedure_template_steps::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (successor_step_id, parent_template_id, successor_position, organization_id),
            (
                ::aps_resources::procedure_template_steps::id,
                ::aps_resources::procedure_template_steps::parent_template_id,
                ::aps_resources::procedure_template_steps::position,
                ::aps_resources::procedure_template_steps::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, parent_template_id, archived),
            (
                ::aps_resources::resources::organization_id,
                ::aps_resources::resources::id,
                ::aps_resources::resources::archived
            )
        )
    )]
    #[diesel(table_name = procedure_template_step_edges)]
    pub struct ProcedureTemplateStepEdge {
        /// Step that completes first.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        predecessor_step_id: ::rosetta_uuid::Uuid,
        /// Step that starts after the predecessor completed.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        successor_step_id: ::rosetta_uuid::Uuid,
        /// Template both steps belong to.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_template_id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the parent template.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Position of the predecessor, kept equal by the cascade.
        #[infallible]
        predecessor_position: i32,
        /// Position of the successor, kept equal by the cascade.
        #[infallible]
        successor_position: i32,
        /// Archive state inherited from this resource.
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    impl ::diesel_builders::ValidateRecord<procedure_template_step_edges::table>
        for <procedure_template_step_edges::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(predecessor_position), Some(successor_position)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_step_edges::predecessor_position,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_step_edges::successor_position,
                >>::may_get_column_ref(self),
            ) && predecessor_position >= successor_position
            {
                return Err(
                    ::validation_errors::ValidationError::strictly_smaller_than(
                        <crate::procedure_template_step_edges::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_step_edges::predecessor_position::NAME,
                        crate::procedure_template_step_edges::successor_position::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_step_edges,
        ::aps_resources::procedure_template_steps
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_step_edges,
        ::aps_resources::resources
    );
}
pub use procedure_template_step_edges_model::*;
pub mod procedure_template_steps_model {
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
    /// A step runs a published child template inside a parent template. The
    /// same child may run in several steps of one parent, told apart by the
    /// step name. Steps with equal `position` may run in parallel. A step with
    /// a repeat slot runs once for every asset the parent procedure binds to
    /// that slot.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(
        foreign_key((parent_template_id), (::aps_resources::procedure_templates::id))
    )]
    #[table_model(
        foreign_key((child_template_id), (::aps_resources::procedure_templates::id))
    )]
    #[table_model(
        foreign_key(
            (repeat_slot_id, parent_template_id, organization_id),
            (
                ::aps_resources::procedure_template_slots::id,
                ::aps_resources::procedure_template_slots::procedure_template_id,
                ::aps_resources::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (child_template_id, child_template_archived),
            (
                ::aps_resources::procedure_templates::id,
                ::aps_resources::procedure_templates::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, parent_template_id, archived),
            (
                ::aps_resources::resources::organization_id,
                ::aps_resources::resources::id,
                ::aps_resources::resources::archived
            )
        )
    )]
    #[diesel(table_name = procedure_template_steps)]
    pub struct ProcedureTemplateStep {
        /// Identifier of the step.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the parent template.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Template the step belongs to.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_template_id: ::rosetta_uuid::Uuid,
        /// Template the step runs.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        child_template_id: ::rosetta_uuid::Uuid,
        /// Name, unique within the parent template.
        name: String,
        /// What the step achieves within the parent.
        description: Option<String>,
        /// Order of the step. A predecessor sits at a strictly lower position.
        position: i32,
        /// An optional step may be left without a completed procedure.
        #[table_model(default = false)]
        #[infallible]
        optional: bool,
        /// Parent slot whose every bound asset gets its own run of the step.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        repeat_slot_id: Option<::rosetta_uuid::Uuid>,
        /// Archive state inherited from this resource.
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `child_template_id`.
        #[table_model(default = false)]
        #[infallible]
        child_template_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        procedure_template_steps::parent_template_id,
        procedure_template_steps::name
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_steps::id,
        procedure_template_steps::parent_template_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_steps::id,
        procedure_template_steps::parent_template_id,
        procedure_template_steps::position,
        procedure_template_steps::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_steps::id,
        procedure_template_steps::parent_template_id,
        procedure_template_steps::child_template_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_steps::id,
        procedure_template_steps::parent_template_id,
        procedure_template_steps::child_template_id,
        procedure_template_steps::organization_id
    );
    impl ::diesel_builders::ValidateColumn<procedure_template_steps::name>
        for <procedure_template_steps::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::name::NAME,
                    ),
                );
            }
            if name.chars().count() > 255usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::name::NAME,
                        255usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_steps::description>
        for <procedure_template_steps::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(description: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if description.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::description::NAME,
                    ),
                );
            }
            if description.chars().count() > 8192usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::description::NAME,
                        8192usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_steps::position>
        for <procedure_template_steps::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(position: &i32) -> Result<(), Self::Error> {
            use diesel::Column;
            if position < &0i32 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::position::NAME,
                        0f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<procedure_template_steps::table>
        for <procedure_template_steps::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_template_id), Some(child_template_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_steps::parent_template_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_steps::child_template_id,
                >>::may_get_column_ref(self),
            ) && parent_template_id == child_template_id
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::parent_template_id::NAME,
                        crate::procedure_template_steps::child_template_id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_steps,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_steps,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_steps,
        ::aps_resources::resources
    );
}
pub use procedure_template_steps_model::*;
pub mod procedure_templates_model {
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
    /// A procedure template is the blueprint of a procedure. It declares the
    /// slots its assets fill, and, when composite, the steps it runs as other
    /// templates. Only a `draft` changes. Publishing freezes it, and a new
    /// version is a new draft naming the published one as
    /// `previous_version_id`.
    #[table_model(ancestors(aps_resources::resources))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::TableName, foreign_key = table_name_id))]
    #[diesel(belongs_to(aps_resources::ProcedureTemplateState, foreign_key = state_id))]
    #[table_model(foreign_key((id), (::aps_resources::resources::id)))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[table_model(
        foreign_key((state_id), (::aps_resources::procedure_template_states::id))
    )]
    #[table_model(
        foreign_key(
            (previous_version_id, project_id, name, previous_version, organization_id),
            (
                procedure_templates::id,
                procedure_templates::project_id,
                procedure_templates::name,
                procedure_templates::version,
                procedure_templates::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (previous_version_id, previous_version_archived),
            (procedure_templates::id, procedure_templates::archived)
        )
    )]
    #[diesel(table_name = procedure_templates)]
    pub struct ProcedureTemplate {
        /// Identifier of the connetto `resource` of kind `'ownable'` this
        /// template extends.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_resources::resources::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// connetto project holding the template.
        #[same_as(aps_resources::resources::parent_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        project_id: ::rosetta_uuid::Uuid,
        /// Most concrete APS table holding this template.
        #[infallible]
        table_name_id: String,
        /// Name shared by every version of the template, unique per version
        /// within the project.
        name: String,
        /// Free-text description of what the procedure achieves.
        description: Option<String>,
        /// Instruction a person follows to perform the procedure.
        instruction: Option<String>,
        /// Lifecycle state, `draft`, `published` or `deprecated`.
        #[table_model(default = "draft")]
        #[infallible]
        state_id: String,
        /// Version number, one more than the previous version's.
        #[table_model(default = 1i32)]
        version: i32,
        /// Published version this draft revises, absent on the first version.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        previous_version_id: Option<::rosetta_uuid::Uuid>,
        /// Version number of `previous_version_id`.
        #[infallible]
        previous_version: Option<i32>,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::resources::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Resource kind fixed to `'ownable'`.
        #[same_as(aps_resources::resources::kind)]
        #[table_model(default = ::aps_authz_resource_kind::ResourceKind::Ownable)]
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        kind: ::aps_authz_resource_kind::ResourceKind,
        /// Archive state of the row named by `previous_version_id`.
        #[table_model(default = false)]
        #[infallible]
        previous_version_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        procedure_templates::project_id,
        procedure_templates::name,
        procedure_templates::version
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_templates::id,
        procedure_templates::project_id,
        procedure_templates::name,
        procedure_templates::version,
        procedure_templates::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_templates::id,
        procedure_templates::archived
    );
    impl ::diesel_builders::ValidateColumn<procedure_templates::name>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::name::NAME,
                ));
            }
            if name.chars().count() > 255usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::name::NAME,
                    255usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_templates::description>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(description: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if description.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::description::NAME,
                ));
            }
            if description.chars().count() > 8192usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::description::NAME,
                    8192usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_templates::instruction>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(instruction: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if instruction.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::instruction::NAME,
                ));
            }
            if instruction.chars().count() > 65536usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::instruction::NAME,
                    65536usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_templates::version>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(version: &i32) -> Result<(), Self::Error> {
            use diesel::Column;
            if version <= &0i32 {
                return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::version::NAME,
                    0f64,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_templates::kind>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            kind: &::aps_authz_resource_kind::ResourceKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if *kind != ::aps_authz_resource_kind::ResourceKind::Ownable {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::procedure_templates::kind::NAME],
                    "kind = 'ownable'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<procedure_templates::table>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(previous_version_id), Some(version)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_templates::previous_version_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_templates::version,
                >>::may_get_column_ref(self),
            ) && (*previous_version_id).is_none() != (*version == 1i32)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_templates::previous_version_id::NAME,
                            crate::procedure_templates::version::NAME,
                        ],
                        "(previous_version_id IS NULL) = (version = 1)",
                    ),
                );
            }
            if let (Some(previous_version_id), Some(previous_version)) =
                (
                    <Self as diesel_builders::MayGetColumn<
                        procedure_templates::previous_version_id,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        procedure_templates::previous_version,
                    >>::may_get_column_ref(self),
                )
                && (*previous_version_id).is_none() != (*previous_version).is_none()
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[
                        crate::procedure_templates::previous_version_id::NAME,
                        crate::procedure_templates::previous_version::NAME,
                    ],
                    "(previous_version_id IS NULL) = (previous_version IS NULL)",
                ));
            }
            if let (Some(version), Some(previous_version)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_templates::version,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_templates::previous_version,
                >>::may_get_column_ref(self),
            )
                && Some(*version)
                    .zip(
                        (*previous_version)
                            .zip(Some(1i32))
                            .map(|(left, right)| left + right),
                    )
                    .map(|(left, right)| left == right) == Some(false)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_templates::version::NAME,
                            crate::procedure_templates::previous_version::NAME,
                        ],
                        "version = previous_version + 1",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for ProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_templates,
        ::aps_resources::procedure_template_states
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_templates,
        ::aps_resources::table_names
    );
}
pub use procedure_templates_model::*;
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
    #[diesel(belongs_to(aps_organizations::Organization, foreign_key = organization_id))]
    #[diesel(belongs_to(aps_organizations::User, foreign_key = created_by_id))]
    #[table_model(
        foreign_key((organization_id), (::aps_organizations::organizations::id))
    )]
    #[table_model(foreign_key((created_by_id), (::aps_organizations::users::id)))]
    #[table_model(
        foreign_key(
            (organization_id, parent_id, parent_kind, parent_archived),
            (
                resources::organization_id,
                resources::id,
                resources::kind,
                resources::archived
            )
        )
    )]
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
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_id: Option<::rosetta_uuid::Uuid>,
        #[infallible]
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        parent_kind: Option<::aps_authz_resource_kind::ResourceKind>,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        created_by_id: Option<::rosetta_uuid::Uuid>,
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        archived_at: Option<::rosetta_utc::TimestampUTC>,
        #[table_model(default = false)]
        #[infallible]
        parent_archived: bool,
        #[table_model(default = false)]
        #[infallible]
        inherits_archive: bool,
        #[table_model(generated)]
        archived: bool,
        #[table_model(default = false)]
        #[infallible]
        writers_can_share: bool,
    }
    ::diesel_builders::prelude::unique_index!(resources::id, resources::parent_id);
    ::diesel_builders::prelude::unique_index!(
        resources::organization_id,
        resources::id,
        resources::archived
    );
    ::diesel_builders::prelude::unique_index!(
        resources::organization_id,
        resources::id,
        resources::kind,
        resources::archived
    );
    impl ::diesel_builders::ValidateRecord<resources::table>
        for <resources::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_id), Some(id)) = (
                <Self as diesel_builders::MayGetColumn<resources::parent_id>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<resources::id>>::may_get_column_ref(self),
            ) && parent_id.as_ref().is_some_and(|parent_id| parent_id == id)
            {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::resources::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::resources::parent_id::NAME,
                    crate::resources::id::NAME,
                ));
            }
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
            if let (Some(kind), Some(parent_id)) = (
                <Self as diesel_builders::MayGetColumn<resources::kind>>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<resources::parent_id>>::may_get_column_ref(
                    self,
                ),
            ) && (*kind == ::aps_authz_resource_kind::ResourceKind::Organization)
                != (*parent_id).is_none()
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::resources::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::resources::kind::NAME, crate::resources::parent_id::NAME],
                    "(kind = 'organization') = (parent_id IS NULL)",
                ));
            }
            if let (Some(kind), Some(parent_kind)) = (
                <Self as diesel_builders::MayGetColumn<resources::kind>>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<resources::parent_kind>>::may_get_column_ref(
                    self,
                ),
            ) && (*kind == ::aps_authz_resource_kind::ResourceKind::Organization)
                != (*parent_kind).is_none()
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::resources::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::resources::kind::NAME, crate::resources::parent_kind::NAME],
                    "(kind = 'organization') = (parent_kind IS NULL)",
                ));
            }
            if let (Some(kind), Some(parent_kind)) = (
                <Self as diesel_builders::MayGetColumn<resources::kind>>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<resources::parent_kind>>::may_get_column_ref(
                    self,
                ),
            ) && (match (
                match (
                    Some(*kind == ::aps_authz_resource_kind::ResourceKind::Organization),
                    match (
                        Some(
                            (*kind == ::aps_authz_resource_kind::ResourceKind::Folder)
                                || (*kind == ::aps_authz_resource_kind::ResourceKind::Project),
                        ),
                        match (
                            (*parent_kind)
                                .zip(Some(::aps_authz_resource_kind::ResourceKind::Organization))
                                .map(|(left, right)| left == right),
                            (*parent_kind)
                                .zip(Some(::aps_authz_resource_kind::ResourceKind::Folder))
                                .map(|(left, right)| left == right),
                        ) {
                            (Some(true), _) | (_, Some(true)) => Some(true),
                            (Some(_), Some(_)) => Some(!true),
                            _ => None,
                        },
                    ) {
                        (Some(false), _) | (_, Some(false)) => Some(false),
                        (Some(_), Some(_)) => Some(!false),
                        _ => None,
                    },
                ) {
                    (Some(true), _) | (_, Some(true)) => Some(true),
                    (Some(_), Some(_)) => Some(!true),
                    _ => None,
                },
                match (
                    Some(*kind == ::aps_authz_resource_kind::ResourceKind::Ownable),
                    (*parent_kind)
                        .zip(Some(::aps_authz_resource_kind::ResourceKind::Project))
                        .map(|(left, right)| left == right),
                ) {
                    (Some(false), _) | (_, Some(false)) => Some(false),
                    (Some(_), Some(_)) => Some(!false),
                    _ => None,
                },
            ) {
                (Some(true), _) | (_, Some(true)) => Some(true),
                (Some(_), Some(_)) => Some(!true),
                _ => None,
            }) == Some(false)
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::resources::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::resources::kind::NAME, crate::resources::parent_kind::NAME],
                    "kind = 'organization' OR (kind IN ('folder', 'project') AND parent_kind IN ('organization', 'folder')) OR (kind = 'ownable' AND parent_kind = 'project')",
                ));
            }
            if let (Some(kind), Some(created_by_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    resources::kind,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    resources::created_by_id,
                >>::may_get_column_ref(self),
            )
                && (*kind == ::aps_authz_resource_kind::ResourceKind::Ownable)
                    != (*created_by_id).is_some()
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::resources::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::resources::kind::NAME,
                            crate::resources::created_by_id::NAME,
                        ],
                        "(kind = 'ownable') = (created_by_id IS NOT NULL)",
                    ),
                );
            }
            if let (Some(inherits_archive), Some(parent_archived)) = (
                <Self as diesel_builders::MayGetColumn<
                    resources::inherits_archive,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    resources::parent_archived,
                >>::may_get_column_ref(self),
            ) && *inherits_archive != *parent_archived
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::resources::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::resources::inherits_archive::NAME,
                            crate::resources::parent_archived::NAME,
                        ],
                        "inherits_archive = parent_archived",
                    ),
                );
            }
            if let (Some(kind), Some(parent_archived)) = (
                <Self as diesel_builders::MayGetColumn<
                    resources::kind,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    resources::parent_archived,
                >>::may_get_column_ref(self),
            )
                && !((*kind != ::aps_authz_resource_kind::ResourceKind::Organization)
                    || !*parent_archived)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::resources::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::resources::kind::NAME,
                            crate::resources::parent_archived::NAME,
                        ],
                        "kind <> 'organization' OR NOT parent_archived",
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(resources, ::aps_organizations::organizations);
    ::diesel::allow_tables_to_appear_in_same_query!(resources, ::aps_organizations::users);
}
pub use resources_model::*;
pub mod slot_role_pairs_model {
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
    /// Which role a step's child slot may play towards the parent slot it is
    /// bound to, so the child's effect on the asset fits what the parent
    /// declares.
    #[diesel(primary_key(child_role_id, parent_role_id))]
    #[table_model(foreign_key((child_role_id), (::aps_resources::slot_roles::id)))]
    #[table_model(foreign_key((parent_role_id), (::aps_resources::slot_roles::id)))]
    #[diesel(table_name = slot_role_pairs)]
    pub struct SlotRolePair {
        /// Role of the child template's slot.
        child_role_id: String,
        /// Role of the parent template's slot.
        parent_role_id: String,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(slot_role_pairs, ::aps_resources::slot_roles);
}
pub use slot_role_pairs_model::*;
pub mod slot_roles_model {
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
    /// How a procedure treats the asset bound to a slot. `input` exists before
    /// and after, `consumed` ceases to exist, `produced` is created by the
    /// procedure, `intermediate` is created and used up by steps of the
    /// procedure, and `instrument` performs it without entering the lineage of
    /// what is produced.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(table_name = slot_roles)]
    pub struct SlotRole {
        /// Role name.
        id: String,
    }
    impl ::diesel_builders::ValidateColumn<slot_roles::id>
        for <slot_roles::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if id.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::slot_roles::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::slot_roles::id::NAME,
                ));
            }
            if id.chars().count() > 63usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::slot_roles::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::slot_roles::id::NAME,
                    63usize,
                ));
            }
            Ok(())
        }
    }
}
pub use slot_roles_model::*;
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
    /// root row so a client can resolve the most concrete table holding a
    /// `resource`.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((instance_table_name_id), (table_names::id)))]
    #[diesel(table_name = table_names)]
    pub struct TableName {
        /// Name of the APS table.
        id: String,
        /// Table holding the instances of an asset model or procedure template
        /// table's rows, the asset table whose model column references the
        /// model table or the procedure table of a procedure template table.
        /// NULL for every other table. The slice creating the instance table
        /// seeds it.
        #[infallible]
        instance_table_name_id: Option<String>,
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
