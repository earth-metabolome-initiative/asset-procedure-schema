pub mod sql_types {
    #[derive(
        Debug, Clone, Copy, ::diesel::query_builder::QueryId, ::diesel::sql_types::SqlType,
    )]
    #[diesel(postgres_type(name = "grantee_kind", schema = "authz"))]
    #[cfg_attr(feature = "sqlite", diesel(sqlite_type(name = "Text")))]
    pub struct GranteeKind;
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::diesel::AsExpression,
    ::diesel::FromSqlRow,
)]
#[diesel(sql_type = self::sql_types::GranteeKind)]
pub enum GranteeKind {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "installation")]
    Installation,
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "signed_in")]
    SignedIn,
    #[serde(rename = "link")]
    Link,
}
impl ::diesel::serialize::ToSql<self::sql_types::GranteeKind, ::diesel::pg::Pg> for GranteeKind {
    fn to_sql<'b>(
        &'b self,
        out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::pg::Pg>,
    ) -> ::diesel::serialize::Result {
        let label: &[u8] = match *self {
            Self::User => b"user",
            Self::Installation => b"installation",
            Self::Team => b"team",
            Self::Organization => b"organization",
            Self::Everyone => b"everyone",
            Self::SignedIn => b"signed_in",
            Self::Link => b"link",
        };
        ::std::io::Write::write_all(out, label)?;
        Ok(::diesel::serialize::IsNull::No)
    }
}
impl ::diesel::deserialize::FromSql<self::sql_types::GranteeKind, ::diesel::pg::Pg>
    for GranteeKind
{
    fn from_sql(value: ::diesel::pg::PgValue<'_>) -> ::diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"user" => Ok(Self::User),
            b"installation" => Ok(Self::Installation),
            b"team" => Ok(Self::Team),
            b"organization" => Ok(Self::Organization),
            b"everyone" => Ok(Self::Everyone),
            b"signed_in" => Ok(Self::SignedIn),
            b"link" => Ok(Self::Link),
            unknown => {
                Err(format!(
                    "Unrecognized label of the `{}` enum type: {}",
                    "authz.grantee_kind",
                    String::from_utf8_lossy(unknown)
                )
                .into())
            }
        }
    }
}
#[cfg(feature = "sqlite")]
impl ::diesel::serialize::ToSql<self::sql_types::GranteeKind, ::diesel::sqlite::Sqlite>
    for GranteeKind
{
    fn to_sql<'b>(
        &'b self,
        out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::sqlite::Sqlite>,
    ) -> ::diesel::serialize::Result {
        out.set_value(match *self {
            Self::User => "user",
            Self::Installation => "installation",
            Self::Team => "team",
            Self::Organization => "organization",
            Self::Everyone => "everyone",
            Self::SignedIn => "signed_in",
            Self::Link => "link",
        });
        Ok(::diesel::serialize::IsNull::No)
    }
}
#[cfg(feature = "sqlite")]
impl ::diesel::deserialize::FromSql<self::sql_types::GranteeKind, ::diesel::sqlite::Sqlite>
    for GranteeKind
{
    fn from_sql(
        mut value: ::diesel::sqlite::SqliteValue<'_, '_, '_>,
    ) -> ::diesel::deserialize::Result<Self> {
        match value.read_text() {
            "user" => Ok(Self::User),
            "installation" => Ok(Self::Installation),
            "team" => Ok(Self::Team),
            "organization" => Ok(Self::Organization),
            "everyone" => Ok(Self::Everyone),
            "signed_in" => Ok(Self::SignedIn),
            "link" => Ok(Self::Link),
            unknown => {
                Err(format!(
                    "Unrecognized label of the `{}` enum type: {unknown}",
                    "authz.grantee_kind"
                )
                .into())
            }
        }
    }
}
