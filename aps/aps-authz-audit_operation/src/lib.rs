pub mod sql_types {
    #[derive(
        Debug, Clone, Copy, ::diesel::query_builder::QueryId, ::diesel::sql_types::SqlType,
    )]
    #[diesel(postgres_type(name = "audit_operation", schema = "authz"))]
    #[cfg_attr(feature = "sqlite", diesel(sqlite_type(name = "Text")))]
    pub struct AuditOperation;
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
#[diesel(sql_type = self::sql_types::AuditOperation)]
pub enum AuditOperation {
    #[serde(rename = "INSERT")]
    Insert,
    #[serde(rename = "UPDATE")]
    Update,
    #[serde(rename = "DELETE")]
    Delete,
}
impl ::diesel::serialize::ToSql<self::sql_types::AuditOperation, ::diesel::pg::Pg>
    for AuditOperation
{
    fn to_sql<'b>(
        &'b self,
        out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::pg::Pg>,
    ) -> ::diesel::serialize::Result {
        let label: &[u8] = match *self {
            Self::Insert => b"INSERT",
            Self::Update => b"UPDATE",
            Self::Delete => b"DELETE",
        };
        ::std::io::Write::write_all(out, label)?;
        Ok(::diesel::serialize::IsNull::No)
    }
}
impl ::diesel::deserialize::FromSql<self::sql_types::AuditOperation, ::diesel::pg::Pg>
    for AuditOperation
{
    fn from_sql(value: ::diesel::pg::PgValue<'_>) -> ::diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"INSERT" => Ok(Self::Insert),
            b"UPDATE" => Ok(Self::Update),
            b"DELETE" => Ok(Self::Delete),
            unknown => {
                Err(format!(
                    "Unrecognized label of the `{}` enum type: {}",
                    "authz.audit_operation",
                    String::from_utf8_lossy(unknown)
                )
                .into())
            }
        }
    }
}
#[cfg(feature = "sqlite")]
impl ::diesel::serialize::ToSql<self::sql_types::AuditOperation, ::diesel::sqlite::Sqlite>
    for AuditOperation
{
    fn to_sql<'b>(
        &'b self,
        out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::sqlite::Sqlite>,
    ) -> ::diesel::serialize::Result {
        out.set_value(match *self {
            Self::Insert => "INSERT",
            Self::Update => "UPDATE",
            Self::Delete => "DELETE",
        });
        Ok(::diesel::serialize::IsNull::No)
    }
}
#[cfg(feature = "sqlite")]
impl ::diesel::deserialize::FromSql<self::sql_types::AuditOperation, ::diesel::sqlite::Sqlite>
    for AuditOperation
{
    fn from_sql(
        mut value: ::diesel::sqlite::SqliteValue<'_, '_, '_>,
    ) -> ::diesel::deserialize::Result<Self> {
        match value.read_text() {
            "INSERT" => Ok(Self::Insert),
            "UPDATE" => Ok(Self::Update),
            "DELETE" => Ok(Self::Delete),
            unknown => {
                Err(format!(
                    "Unrecognized label of the `{}` enum type: {unknown}",
                    "authz.audit_operation"
                )
                .into())
            }
        }
    }
}
