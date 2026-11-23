pub mod sql_types {
    #[derive(
        Debug, Clone, Copy, ::diesel::query_builder::QueryId, ::diesel::sql_types::SqlType,
    )]
    #[diesel(postgres_type(name = "team_role", schema = "authz"))]
    #[cfg_attr(feature = "sqlite", diesel(sqlite_type(name = "Text")))]
    pub struct TeamRole;
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
#[diesel(sql_type = self::sql_types::TeamRole)]
pub enum TeamRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Admin,
}
impl ::diesel::serialize::ToSql<self::sql_types::TeamRole, ::diesel::pg::Pg> for TeamRole {
    fn to_sql<'b>(
        &'b self,
        out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::pg::Pg>,
    ) -> ::diesel::serialize::Result {
        let label: &[u8] = match *self {
            Self::Member => b"member",
            Self::Admin => b"admin",
        };
        ::std::io::Write::write_all(out, label)?;
        Ok(::diesel::serialize::IsNull::No)
    }
}
impl ::diesel::deserialize::FromSql<self::sql_types::TeamRole, ::diesel::pg::Pg> for TeamRole {
    fn from_sql(value: ::diesel::pg::PgValue<'_>) -> ::diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"member" => Ok(Self::Member),
            b"admin" => Ok(Self::Admin),
            unknown => {
                Err(format!(
                    "Unrecognized label of the `{}` enum type: {}",
                    "authz.team_role",
                    String::from_utf8_lossy(unknown)
                )
                .into())
            }
        }
    }
}
#[cfg(feature = "sqlite")]
impl ::diesel::serialize::ToSql<self::sql_types::TeamRole, ::diesel::sqlite::Sqlite> for TeamRole {
    fn to_sql<'b>(
        &'b self,
        out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::sqlite::Sqlite>,
    ) -> ::diesel::serialize::Result {
        out.set_value(match *self {
            Self::Member => "member",
            Self::Admin => "admin",
        });
        Ok(::diesel::serialize::IsNull::No)
    }
}
#[cfg(feature = "sqlite")]
impl ::diesel::deserialize::FromSql<self::sql_types::TeamRole, ::diesel::sqlite::Sqlite>
    for TeamRole
{
    fn from_sql(
        mut value: ::diesel::sqlite::SqliteValue<'_, '_, '_>,
    ) -> ::diesel::deserialize::Result<Self> {
        match value.read_text() {
            "member" => Ok(Self::Member),
            "admin" => Ok(Self::Admin),
            unknown => {
                Err(format!(
                    "Unrecognized label of the `{}` enum type: {unknown}",
                    "authz.team_role"
                )
                .into())
            }
        }
    }
}
