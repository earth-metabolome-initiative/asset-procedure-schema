#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_grantees;
pub mod app_installations_model {
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
    #[table_model(ancestors(aps_grantees::grantees))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_grantees::App, foreign_key = app_id))]
    #[diesel(belongs_to(aps_grantees::Organization, foreign_key = organization_id))]
    #[table_model(foreign_key((app_id), (::aps_grantees::apps::id)))]
    #[table_model(foreign_key((organization_id), (::aps_grantees::organizations::id)))]
    #[table_model(default(
        aps_grantees::grantees::kind,
        ::aps_authz_grantee_kind::GranteeKind::Installation
    ))]
    #[diesel(table_name = app_installations)]
    pub struct AppInstallation {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[same_as(aps_grantees::grantees::kind)]
        #[table_model(default = ::aps_authz_grantee_kind::GranteeKind::Installation)]
        #[diesel(sql_type = ::aps_authz_grantee_kind::sql_types::GranteeKind)]
        kind: ::aps_authz_grantee_kind::GranteeKind,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        app_id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
    }
    ::diesel_builders::prelude::unique_index!(
        app_installations::organization_id,
        app_installations::id
    );
    impl ::diesel_builders::ValidateColumn<app_installations::kind>
        for <app_installations::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            kind: &::aps_authz_grantee_kind::GranteeKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if *kind != ::aps_authz_grantee_kind::GranteeKind::Installation {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::app_installations::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::app_installations::kind::NAME],
                    "kind = 'installation'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_grantees::grantees::id> for AppInstallation {
        fn get_column_ref(
            &self,
        ) -> &<app_installations::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(app_installations, ::aps_grantees::apps);
    ::diesel::allow_tables_to_appear_in_same_query!(app_installations, ::aps_grantees::grantees);
    ::diesel::allow_tables_to_appear_in_same_query!(
        app_installations,
        ::aps_grantees::organizations
    );
}
pub use app_installations_model::*;
pub mod apps_model {
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
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_grantees::Organization, foreign_key = organization_id))]
    #[table_model(foreign_key((organization_id), (::aps_grantees::organizations::id)))]
    #[diesel(table_name = apps)]
    pub struct App {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        name: String,
    }
    impl ::diesel_builders::ValidateColumn<apps::name>
        for <apps::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::apps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::apps::name::NAME,
                ));
            }
            if name.chars().count() > 200usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::apps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::apps::name::NAME,
                    200usize,
                ));
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(apps, ::aps_grantees::organizations);
}
pub use apps_model::*;
pub mod grantees_model {
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
    #[diesel(table_name = grantees)]
    pub struct Grantee {
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::aps_authz_grantee_kind::sql_types::GranteeKind)]
        kind: ::aps_authz_grantee_kind::GranteeKind,
    }
    ::diesel_builders::prelude::unique_index!(grantees::id, grantees::kind);
}
pub use grantees_model::*;
pub mod installation_permissions_model {
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
    #[diesel(primary_key(organization_id, installation_id, kind))]
    #[table_model(
        foreign_key(
            (organization_id, installation_id),
            (
                ::aps_grantees::app_installations::organization_id,
                ::aps_grantees::app_installations::id
            )
        )
    )]
    #[diesel(table_name = installation_permissions)]
    pub struct InstallationPermission {
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        installation_id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        kind: ::aps_authz_resource_kind::ResourceKind,
        #[diesel(sql_type = ::aps_authz_role::sql_types::Role)]
        max_role: ::aps_authz_role::Role,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        installation_permissions,
        ::aps_grantees::app_installations
    );
}
pub use installation_permissions_model::*;
pub mod invitations_model {
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
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_grantees::Organization, foreign_key = organization_id))]
    #[table_model(foreign_key((organization_id), (::aps_grantees::organizations::id)))]
    #[table_model(foreign_key((invited_by_id), (::aps_grantees::users::id)))]
    #[table_model(foreign_key((accepted_by_id), (::aps_grantees::users::id)))]
    #[diesel(table_name = invitations)]
    pub struct Invitation {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        email: String,
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        expires_at: ::rosetta_utc::TimestampUTC,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        invited_by_id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        accepted_by_id: Option<::rosetta_uuid::Uuid>,
    }
    ::diesel_builders::prelude::unique_index!(invitations::organization_id, invitations::id);
    impl ::diesel_builders::ValidateColumn<invitations::email>
        for <invitations::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(email: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if email.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::invitations::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::invitations::email::NAME,
                ));
            }
            if email.chars().count() > 320usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::invitations::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::invitations::email::NAME,
                    320usize,
                ));
            }
            if !({
                let text = email.as_str();
                'like: {
                    let mut rest = text.chars();
                    loop {
                        let mut candidate = rest.clone();
                        if candidate.next().is_some()
                            && candidate.next() == Some('@')
                            && candidate.next().is_some()
                        {
                            break 'like true;
                        }
                        if rest.next().is_none() {
                            break 'like false;
                        }
                    }
                }
            }) {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::invitations::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::invitations::email::NAME],
                    "email LIKE '%_@_%'",
                ));
            }
            if email.as_str() != email.as_str().to_lowercase().as_str() {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::invitations::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::invitations::email::NAME],
                    "email = lower(email)",
                ));
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(invitations, ::aps_grantees::organizations);
    ::diesel::allow_tables_to_appear_in_same_query!(invitations, ::aps_grantees::users);
}
pub use invitations_model::*;
pub mod memberships_model {
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
    #[diesel(belongs_to(aps_grantees::Organization, foreign_key = organization_id))]
    #[diesel(belongs_to(aps_grantees::User, foreign_key = user_id))]
    #[diesel(primary_key(organization_id, user_id))]
    #[table_model(foreign_key((organization_id), (::aps_grantees::organizations::id)))]
    #[table_model(foreign_key((user_id), (::aps_grantees::users::id)))]
    #[table_model(
        foreign_key(
            (organization_id, invitation_id),
            (
                ::aps_grantees::invitations::organization_id,
                ::aps_grantees::invitations::id
            )
        )
    )]
    #[diesel(table_name = memberships)]
    pub struct Membership {
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        user_id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::aps_authz_membership_state::sql_types::MembershipState)]
        state: ::aps_authz_membership_state::MembershipState,
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        invitation_id: Option<::rosetta_uuid::Uuid>,
    }
    ::diesel_builders::prelude::unique_index!(memberships::invitation_id);
    ::diesel::allow_tables_to_appear_in_same_query!(memberships, ::aps_grantees::invitations);
    ::diesel::allow_tables_to_appear_in_same_query!(memberships, ::aps_grantees::organizations);
    ::diesel::allow_tables_to_appear_in_same_query!(memberships, ::aps_grantees::users);
}
pub use memberships_model::*;
pub mod namespaces_model {
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
    #[diesel(belongs_to(aps_grantees::User, foreign_key = user_id))]
    #[table_model(foreign_key((user_id), (::aps_grantees::users::id)))]
    #[diesel(table_name = namespaces)]
    pub struct Namespace {
        #[table_model(default = ::rosetta_uuid::Uuid::new_v4())]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        user_id: Option<::rosetta_uuid::Uuid>,
        #[table_model(default = false)]
        #[infallible]
        reserved: bool,
    }
    impl ::diesel_builders::ValidateRecord<namespaces::table>
        for <namespaces::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(reserved), Some(user_id)) = (
                <Self as diesel_builders::MayGetColumn<namespaces::reserved>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<namespaces::user_id>>::may_get_column_ref(
                    self,
                ),
            ) && !(!*reserved || (*user_id).is_none())
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::namespaces::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::namespaces::reserved::NAME, crate::namespaces::user_id::NAME],
                    "NOT reserved OR user_id IS NULL",
                ));
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(namespaces, ::aps_grantees::users);
}
pub use namespaces_model::*;
pub mod organization_blocks_model {
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
    #[diesel(belongs_to(aps_grantees::Organization, foreign_key = organization_id))]
    #[diesel(primary_key(organization_id, principal_id))]
    #[table_model(foreign_key((organization_id), (::aps_grantees::organizations::id)))]
    #[table_model(
        foreign_key(
            (principal_id, principal_kind),
            (::aps_grantees::grantees::id, ::aps_grantees::grantees::kind)
        )
    )]
    #[diesel(table_name = organization_blocks)]
    pub struct OrganizationBlock {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        principal_id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::aps_authz_grantee_kind::sql_types::GranteeKind)]
        principal_kind: ::aps_authz_grantee_kind::GranteeKind,
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        expires_at: Option<::rosetta_utc::TimestampUTC>,
    }
    impl ::diesel_builders::ValidateColumn<organization_blocks::principal_kind>
        for <organization_blocks::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            principal_kind: &::aps_authz_grantee_kind::GranteeKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if !((*principal_kind == ::aps_authz_grantee_kind::GranteeKind::User)
                || (*principal_kind == ::aps_authz_grantee_kind::GranteeKind::Installation))
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::organization_blocks::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::organization_blocks::principal_kind::NAME],
                    "principal_kind IN ('user', 'installation')",
                ));
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(organization_blocks, ::aps_grantees::grantees);
    ::diesel::allow_tables_to_appear_in_same_query!(
        organization_blocks,
        ::aps_grantees::organizations
    );
}
pub use organization_blocks_model::*;
pub mod organizations_model {
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
    #[table_model(ancestors(aps_grantees::grantees))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_grantees::User, foreign_key = created_by_id))]
    #[table_model(foreign_key((created_by_id), (::aps_grantees::users::id)))]
    #[table_model(default(
        aps_grantees::grantees::kind,
        ::aps_authz_grantee_kind::GranteeKind::Organization
    ))]
    #[diesel(table_name = organizations)]
    pub struct Organization {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[same_as(aps_grantees::grantees::kind)]
        #[table_model(default = ::aps_authz_grantee_kind::GranteeKind::Organization)]
        #[diesel(sql_type = ::aps_authz_grantee_kind::sql_types::GranteeKind)]
        kind: ::aps_authz_grantee_kind::GranteeKind,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        created_by_id: ::rosetta_uuid::Uuid,
        /// The founder membership writes the admin grant that stamps
        /// `founded_at`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        founded_at: Option<::rosetta_utc::TimestampUTC>,
    }
    impl ::diesel_builders::ValidateColumn<organizations::kind>
        for <organizations::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            kind: &::aps_authz_grantee_kind::GranteeKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if *kind != ::aps_authz_grantee_kind::GranteeKind::Organization {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::organizations::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::organizations::kind::NAME],
                    "kind = 'organization'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_grantees::grantees::id> for Organization {
        fn get_column_ref(
            &self,
        ) -> &<organizations::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(organizations, ::aps_grantees::grantees);
    ::diesel::allow_tables_to_appear_in_same_query!(organizations, ::aps_grantees::users);
}
pub use organizations_model::*;
pub mod platform_operators_model {
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
    #[table_model(ancestors(aps_grantees::grantees, aps_grantees::users))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_grantees::User, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_grantees::users::id)))]
    #[table_model(default(
        aps_grantees::grantees::kind,
        ::aps_authz_grantee_kind::GranteeKind::User
    ))]
    #[diesel(table_name = platform_operators)]
    pub struct PlatformOperator {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_grantees::grantees::id> for PlatformOperator {
        fn get_column_ref(
            &self,
        ) -> &<platform_operators::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_grantees::users::id> for PlatformOperator {
        fn get_column_ref(
            &self,
        ) -> &<platform_operators::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(platform_operators, ::aps_grantees::grantees);
    ::diesel::allow_tables_to_appear_in_same_query!(platform_operators, ::aps_grantees::users);
}
pub use platform_operators_model::*;
pub mod share_links_model {
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
    #[table_model(ancestors(aps_grantees::grantees))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_grantees::Organization, foreign_key = organization_id))]
    #[diesel(belongs_to(aps_grantees::User, foreign_key = created_by_id))]
    #[table_model(foreign_key((organization_id), (::aps_grantees::organizations::id)))]
    #[table_model(foreign_key((created_by_id), (::aps_grantees::users::id)))]
    #[table_model(default(
        aps_grantees::grantees::kind,
        ::aps_authz_grantee_kind::GranteeKind::Link
    ))]
    #[diesel(table_name = share_links)]
    pub struct ShareLink {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[same_as(aps_grantees::grantees::kind)]
        #[table_model(default = ::aps_authz_grantee_kind::GranteeKind::Link)]
        #[diesel(sql_type = ::aps_authz_grantee_kind::sql_types::GranteeKind)]
        kind: ::aps_authz_grantee_kind::GranteeKind,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        token_hash: Vec<u8>,
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        expires_at: Option<::rosetta_utc::TimestampUTC>,
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        revoked_at: Option<::rosetta_utc::TimestampUTC>,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        created_by_id: ::rosetta_uuid::Uuid,
    }
    ::diesel_builders::prelude::unique_index!(share_links::token_hash);
    impl ::diesel_builders::ValidateColumn<share_links::kind>
        for <share_links::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            kind: &::aps_authz_grantee_kind::GranteeKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if *kind != ::aps_authz_grantee_kind::GranteeKind::Link {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::share_links::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::share_links::kind::NAME],
                    "kind = 'link'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<share_links::token_hash>
        for <share_links::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(token_hash: &Vec<u8>) -> Result<(), Self::Error> {
            use diesel::Column;
            if token_hash.len() != 32usize {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::share_links::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::share_links::token_hash::NAME],
                    "octet_length(token_hash) = 32",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_grantees::grantees::id> for ShareLink {
        fn get_column_ref(
            &self,
        ) -> &<share_links::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(share_links, ::aps_grantees::grantees);
    ::diesel::allow_tables_to_appear_in_same_query!(share_links, ::aps_grantees::organizations);
    ::diesel::allow_tables_to_appear_in_same_query!(share_links, ::aps_grantees::users);
}
pub use share_links_model::*;
pub mod team_members_model {
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
    #[diesel(primary_key(organization_id, team_id, user_id))]
    #[table_model(
        foreign_key(
            (organization_id, team_id),
            (::aps_grantees::teams::organization_id, ::aps_grantees::teams::id)
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, user_id),
            (
                ::aps_grantees::memberships::organization_id,
                ::aps_grantees::memberships::user_id
            )
        )
    )]
    #[diesel(table_name = team_members)]
    pub struct TeamMember {
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        team_id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        user_id: ::rosetta_uuid::Uuid,
        #[diesel(sql_type = ::aps_authz_team_role::sql_types::TeamRole)]
        team_role: ::aps_authz_team_role::TeamRole,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(team_members, ::aps_grantees::memberships);
    ::diesel::allow_tables_to_appear_in_same_query!(team_members, ::aps_grantees::teams);
}
pub use team_members_model::*;
pub mod teams_model {
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
    #[table_model(ancestors(aps_grantees::grantees))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_grantees::Organization, foreign_key = organization_id))]
    #[table_model(foreign_key((organization_id), (::aps_grantees::organizations::id)))]
    #[table_model(
        foreign_key((organization_id, parent_id), (teams::organization_id, teams::id))
    )]
    #[table_model(default(
        aps_grantees::grantees::kind,
        ::aps_authz_grantee_kind::GranteeKind::Team
    ))]
    #[diesel(table_name = teams)]
    pub struct Team {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[same_as(aps_grantees::grantees::kind)]
        #[table_model(default = ::aps_authz_grantee_kind::GranteeKind::Team)]
        #[diesel(sql_type = ::aps_authz_grantee_kind::sql_types::GranteeKind)]
        kind: ::aps_authz_grantee_kind::GranteeKind,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_id: Option<::rosetta_uuid::Uuid>,
    }
    ::diesel_builders::prelude::unique_index!(teams::organization_id, teams::id);
    impl ::diesel_builders::ValidateColumn<teams::kind>
        for <teams::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            kind: &::aps_authz_grantee_kind::GranteeKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if *kind != ::aps_authz_grantee_kind::GranteeKind::Team {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::teams::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::teams::kind::NAME],
                    "kind = 'team'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<teams::table>
        for <teams::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_id), Some(id)) = (
                <Self as diesel_builders::MayGetColumn<teams::parent_id>>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<teams::id>>::may_get_column_ref(self),
            ) && parent_id.as_ref().is_some_and(|parent_id| parent_id == id)
            {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::teams::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::teams::parent_id::NAME,
                    crate::teams::id::NAME,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_grantees::grantees::id> for Team {
        fn get_column_ref(&self) -> &<teams::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(teams, ::aps_grantees::grantees);
    ::diesel::allow_tables_to_appear_in_same_query!(teams, ::aps_grantees::organizations);
}
pub use teams_model::*;
pub mod users_model {
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
    #[table_model(ancestors(aps_grantees::grantees))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(default(
        aps_grantees::grantees::kind,
        ::aps_authz_grantee_kind::GranteeKind::User
    ))]
    #[diesel(table_name = users)]
    pub struct User {
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        #[same_as(aps_grantees::grantees::kind)]
        #[table_model(default = ::aps_authz_grantee_kind::GranteeKind::User)]
        #[diesel(sql_type = ::aps_authz_grantee_kind::sql_types::GranteeKind)]
        kind: ::aps_authz_grantee_kind::GranteeKind,
    }
    impl ::diesel_builders::ValidateColumn<users::kind>
        for <users::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            kind: &::aps_authz_grantee_kind::GranteeKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if *kind != ::aps_authz_grantee_kind::GranteeKind::User {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::users::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::users::kind::NAME],
                    "kind = 'user'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_grantees::grantees::id> for User {
        fn get_column_ref(&self) -> &<users::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(users, ::aps_grantees::grantees);
}
pub use users_model::*;
