#![feature(prelude_import)]
//! Auto-generated crate for the `procedure_template_asset_models` table.
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
/// Table to store procedure template asset models
#[table_model(error = ::validation_errors::ValidationError)]
#[diesel(
    belongs_to(
        aps_procedure_templates::ProcedureTemplate,
        foreign_key = procedure_template_id
    )
)]
#[diesel(belongs_to(aps_asset_models::AssetModel, foreign_key = asset_model_id))]
#[table_model(
    foreign_key(
        (procedure_template_id),
        (::aps_procedure_templates::procedure_templates::id)
    )
)]
#[table_model(foreign_key((based_on_id), (procedure_template_asset_models::id)))]
#[table_model(foreign_key((asset_model_id), (::aps_asset_models::asset_models::id)))]
#[table_model(
    foreign_key(
        (based_on_id, asset_model_id),
        (
            procedure_template_asset_models::id,
            procedure_template_asset_models::asset_model_id
        )
    )
)]
#[diesel(table_name = procedure_template_asset_models)]
pub struct ProcedureTemplateAssetModel {
    /// Identifier of the procedure template asset model
    #[table_model(default = ::rosetta_uuid::Uuid::new_v4())]
    #[infallible]
    #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The name of the procedure template asset model
    name: String,
    /// Procedure template this asset model is associated with
    #[infallible]
    #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
    procedure_template_id: ::rosetta_uuid::Uuid,
    /// which this procedure template asset model is based on
    #[infallible]
    #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
    based_on_id: Option<::rosetta_uuid::Uuid>,
    /// The asset model this procedure template asset model is associated with
    #[infallible]
    #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
    asset_model_id: ::rosetta_uuid::Uuid,
}
#[automatically_derived]
impl ::core::clone::Clone for ProcedureTemplateAssetModel {
    #[inline]
    fn clone(&self) -> ProcedureTemplateAssetModel {
        ProcedureTemplateAssetModel {
            id: ::core::clone::Clone::clone(&self.id),
            name: ::core::clone::Clone::clone(&self.name),
            procedure_template_id: ::core::clone::Clone::clone(
                &self.procedure_template_id,
            ),
            based_on_id: ::core::clone::Clone::clone(&self.based_on_id),
            asset_model_id: ::core::clone::Clone::clone(&self.asset_model_id),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for ProcedureTemplateAssetModel {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "ProcedureTemplateAssetModel",
            "id",
            &self.id,
            "name",
            &self.name,
            "procedure_template_id",
            &self.procedure_template_id,
            "based_on_id",
            &self.based_on_id,
            "asset_model_id",
            &&self.asset_model_id,
        )
    }
}
#[automatically_derived]
impl ::core::hash::Hash for ProcedureTemplateAssetModel {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
        ::core::hash::Hash::hash(&self.id, state);
        ::core::hash::Hash::hash(&self.name, state);
        ::core::hash::Hash::hash(&self.procedure_template_id, state);
        ::core::hash::Hash::hash(&self.based_on_id, state);
        ::core::hash::Hash::hash(&self.asset_model_id, state)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for ProcedureTemplateAssetModel {
    #[inline]
    fn cmp(&self, other: &ProcedureTemplateAssetModel) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.id, &other.id) {
            ::core::cmp::Ordering::Equal => {
                match ::core::cmp::Ord::cmp(&self.name, &other.name) {
                    ::core::cmp::Ordering::Equal => {
                        match ::core::cmp::Ord::cmp(
                            &self.procedure_template_id,
                            &other.procedure_template_id,
                        ) {
                            ::core::cmp::Ordering::Equal => {
                                match ::core::cmp::Ord::cmp(
                                    &self.based_on_id,
                                    &other.based_on_id,
                                ) {
                                    ::core::cmp::Ordering::Equal => {
                                        ::core::cmp::Ord::cmp(
                                            &self.asset_model_id,
                                            &other.asset_model_id,
                                        )
                                    }
                                    cmp => cmp,
                                }
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for ProcedureTemplateAssetModel {
    #[inline]
    fn partial_cmp(
        &self,
        other: &ProcedureTemplateAssetModel,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.id, &other.id) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                match ::core::cmp::PartialOrd::partial_cmp(&self.name, &other.name) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(
                            &self.procedure_template_id,
                            &other.procedure_template_id,
                        ) {
                            ::core::option::Option::Some(
                                ::core::cmp::Ordering::Equal,
                            ) => {
                                match ::core::cmp::PartialOrd::partial_cmp(
                                    &self.based_on_id,
                                    &other.based_on_id,
                                ) {
                                    ::core::option::Option::Some(
                                        ::core::cmp::Ordering::Equal,
                                    ) => {
                                        ::core::cmp::PartialOrd::partial_cmp(
                                            &self.asset_model_id,
                                            &other.asset_model_id,
                                        )
                                    }
                                    cmp => cmp,
                                }
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for ProcedureTemplateAssetModel {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) {
        let _: ::core::cmp::AssertParamIsEq<::rosetta_uuid::Uuid>;
        let _: ::core::cmp::AssertParamIsEq<String>;
        let _: ::core::cmp::AssertParamIsEq<::rosetta_uuid::Uuid>;
        let _: ::core::cmp::AssertParamIsEq<Option<::rosetta_uuid::Uuid>>;
        let _: ::core::cmp::AssertParamIsEq<::rosetta_uuid::Uuid>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ProcedureTemplateAssetModel {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ProcedureTemplateAssetModel {
    #[inline]
    fn eq(&self, other: &ProcedureTemplateAssetModel) -> bool {
        self.id == other.id && self.name == other.name
            && self.procedure_template_id == other.procedure_template_id
            && self.based_on_id == other.based_on_id
            && self.asset_model_id == other.asset_model_id
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for ProcedureTemplateAssetModel {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "ProcedureTemplateAssetModel",
                false as usize + 1 + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "id",
                &self.id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "procedure_template_id",
                &self.procedure_template_id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "based_on_id",
                &self.based_on_id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "asset_model_id",
                &self.asset_model_id,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ProcedureTemplateAssetModel {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        2u64 => _serde::__private228::Ok(__Field::__field2),
                        3u64 => _serde::__private228::Ok(__Field::__field3),
                        4u64 => _serde::__private228::Ok(__Field::__field4),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => _serde::__private228::Ok(__Field::__field0),
                        "name" => _serde::__private228::Ok(__Field::__field1),
                        "procedure_template_id" => {
                            _serde::__private228::Ok(__Field::__field2)
                        }
                        "based_on_id" => _serde::__private228::Ok(__Field::__field3),
                        "asset_model_id" => _serde::__private228::Ok(__Field::__field4),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => _serde::__private228::Ok(__Field::__field0),
                        b"name" => _serde::__private228::Ok(__Field::__field1),
                        b"procedure_template_id" => {
                            _serde::__private228::Ok(__Field::__field2)
                        }
                        b"based_on_id" => _serde::__private228::Ok(__Field::__field3),
                        b"asset_model_id" => _serde::__private228::Ok(__Field::__field4),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<ProcedureTemplateAssetModel>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ProcedureTemplateAssetModel;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct ProcedureTemplateAssetModel",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        ::rosetta_uuid::Uuid,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ProcedureTemplateAssetModel with 5 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct ProcedureTemplateAssetModel with 5 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        ::rosetta_uuid::Uuid,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct ProcedureTemplateAssetModel with 5 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        Option<::rosetta_uuid::Uuid>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct ProcedureTemplateAssetModel with 5 elements",
                                ),
                            );
                        }
                    };
                    let __field4 = match _serde::de::SeqAccess::next_element::<
                        ::rosetta_uuid::Uuid,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct ProcedureTemplateAssetModel with 5 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(ProcedureTemplateAssetModel {
                        id: __field0,
                        name: __field1,
                        procedure_template_id: __field2,
                        based_on_id: __field3,
                        asset_model_id: __field4,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<
                        ::rosetta_uuid::Uuid,
                    > = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<String> = _serde::__private228::None;
                    let mut __field2: _serde::__private228::Option<
                        ::rosetta_uuid::Uuid,
                    > = _serde::__private228::None;
                    let mut __field3: _serde::__private228::Option<
                        Option<::rosetta_uuid::Uuid>,
                    > = _serde::__private228::None;
                    let mut __field4: _serde::__private228::Option<
                        ::rosetta_uuid::Uuid,
                    > = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        ::rosetta_uuid::Uuid,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private228::Option::is_some(&__field2) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "procedure_template_id",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        ::rosetta_uuid::Uuid,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private228::Option::is_some(&__field3) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "based_on_id",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<::rosetta_uuid::Uuid>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field4 => {
                                if _serde::__private228::Option::is_some(&__field4) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "asset_model_id",
                                        ),
                                    );
                                }
                                __field4 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        ::rosetta_uuid::Uuid,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("id")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("name")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private228::Some(__field2) => __field2,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field(
                                "procedure_template_id",
                            )?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private228::Some(__field3) => __field3,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("based_on_id")?
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::__private228::Some(__field4) => __field4,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("asset_model_id")?
                        }
                    };
                    _serde::__private228::Ok(ProcedureTemplateAssetModel {
                        id: __field0,
                        name: __field1,
                        procedure_template_id: __field2,
                        based_on_id: __field3,
                        asset_model_id: __field4,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "id",
                "name",
                "procedure_template_id",
                "based_on_id",
                "asset_model_id",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ProcedureTemplateAssetModel",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<
                        ProcedureTemplateAssetModel,
                    >,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    use diesel;
    use diesel::row::{Row as _, Field as _};
    impl<
        __DB: diesel::backend::Backend,
        __ST0,
        __ST1,
        __ST2,
        __ST3,
        __ST4,
    > diesel::deserialize::Queryable<(__ST0, __ST1, __ST2, __ST3, __ST4), __DB>
    for ProcedureTemplateAssetModel
    where
        (
            ::rosetta_uuid::Uuid,
            String,
            ::rosetta_uuid::Uuid,
            Option<::rosetta_uuid::Uuid>,
            ::rosetta_uuid::Uuid,
        ): diesel::deserialize::FromStaticSqlRow<
            (__ST0, __ST1, __ST2, __ST3, __ST4),
            __DB,
        >,
    {
        type Row = (
            ::rosetta_uuid::Uuid,
            String,
            ::rosetta_uuid::Uuid,
            Option<::rosetta_uuid::Uuid>,
            ::rosetta_uuid::Uuid,
        );
        fn build(
            row: (
                ::rosetta_uuid::Uuid,
                String,
                ::rosetta_uuid::Uuid,
                Option<::rosetta_uuid::Uuid>,
                ::rosetta_uuid::Uuid,
            ),
        ) -> diesel::deserialize::Result<Self> {
            use std::convert::TryInto;
            diesel::deserialize::Result::Ok(Self {
                id: row.0.try_into()?,
                name: row.1.try_into()?,
                procedure_template_id: row.2.try_into()?,
                based_on_id: row.3.try_into()?,
                asset_model_id: row.4.try_into()?,
            })
        }
    }
};
const _: () = {
    use diesel;
    use diesel::expression::Selectable;
    impl<__DB: diesel::backend::Backend> Selectable<__DB>
    for ProcedureTemplateAssetModel {
        type SelectExpression = (
            procedure_template_asset_models::id,
            procedure_template_asset_models::name,
            procedure_template_asset_models::procedure_template_id,
            procedure_template_asset_models::based_on_id,
            procedure_template_asset_models::asset_model_id,
        );
        fn construct_selection() -> Self::SelectExpression {
            (
                procedure_template_asset_models::id,
                procedure_template_asset_models::name,
                procedure_template_asset_models::procedure_template_id,
                procedure_template_asset_models::based_on_id,
                procedure_template_asset_models::asset_model_id,
            )
        }
    }
};
const _: () = {
    use diesel;
    impl diesel::associations::HasTable for ProcedureTemplateAssetModel {
        type Table = procedure_template_asset_models::table;
        fn table() -> <Self as diesel::associations::HasTable>::Table {
            procedure_template_asset_models::table
        }
    }
    impl<'ident> diesel::associations::Identifiable
    for &'ident ProcedureTemplateAssetModel {
        type Id = (&'ident ::rosetta_uuid::Uuid);
        fn id(self) -> <Self as diesel::associations::Identifiable>::Id {
            (&self.id)
        }
    }
    impl<'ident> diesel::associations::Identifiable
    for &'_ &'ident ProcedureTemplateAssetModel {
        type Id = (&'ident ::rosetta_uuid::Uuid);
        fn id(self) -> <Self as diesel::associations::Identifiable>::Id {
            (&self.id)
        }
    }
};
const _: () = {
    use diesel;
    impl<
        __FK,
    > diesel::associations::BelongsTo<aps_procedure_templates::ProcedureTemplate>
    for ProcedureTemplateAssetModel
    where
        __FK: std::hash::Hash + std::cmp::Eq,
        for<'__a> &'__a ::rosetta_uuid::Uuid: std::convert::Into<
            ::std::option::Option<&'__a __FK>,
        >,
        for<'__a> &'__a aps_procedure_templates::ProcedureTemplate: diesel::associations::Identifiable<
            Id = &'__a __FK,
        >,
    {
        type ForeignKey = __FK;
        type ForeignKeyColumn = procedure_template_asset_models::procedure_template_id;
        fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
            std::convert::Into::into(&self.procedure_template_id)
        }
        fn foreign_key_column() -> Self::ForeignKeyColumn {
            procedure_template_asset_models::procedure_template_id
        }
    }
    impl<
        __FK,
    > diesel::associations::BelongsTo<&'_ aps_procedure_templates::ProcedureTemplate>
    for ProcedureTemplateAssetModel
    where
        __FK: std::hash::Hash + std::cmp::Eq,
        for<'__a> &'__a ::rosetta_uuid::Uuid: std::convert::Into<
            ::std::option::Option<&'__a __FK>,
        >,
        for<'__a> &'__a aps_procedure_templates::ProcedureTemplate: diesel::associations::Identifiable<
            Id = &'__a __FK,
        >,
    {
        type ForeignKey = __FK;
        type ForeignKeyColumn = procedure_template_asset_models::procedure_template_id;
        fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
            std::convert::Into::into(&self.procedure_template_id)
        }
        fn foreign_key_column() -> Self::ForeignKeyColumn {
            procedure_template_asset_models::procedure_template_id
        }
    }
    impl<__FK> diesel::associations::BelongsTo<aps_asset_models::AssetModel>
    for ProcedureTemplateAssetModel
    where
        __FK: std::hash::Hash + std::cmp::Eq,
        for<'__a> &'__a ::rosetta_uuid::Uuid: std::convert::Into<
            ::std::option::Option<&'__a __FK>,
        >,
        for<'__a> &'__a aps_asset_models::AssetModel: diesel::associations::Identifiable<
            Id = &'__a __FK,
        >,
    {
        type ForeignKey = __FK;
        type ForeignKeyColumn = procedure_template_asset_models::asset_model_id;
        fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
            std::convert::Into::into(&self.asset_model_id)
        }
        fn foreign_key_column() -> Self::ForeignKeyColumn {
            procedure_template_asset_models::asset_model_id
        }
    }
    impl<__FK> diesel::associations::BelongsTo<&'_ aps_asset_models::AssetModel>
    for ProcedureTemplateAssetModel
    where
        __FK: std::hash::Hash + std::cmp::Eq,
        for<'__a> &'__a ::rosetta_uuid::Uuid: std::convert::Into<
            ::std::option::Option<&'__a __FK>,
        >,
        for<'__a> &'__a aps_asset_models::AssetModel: diesel::associations::Identifiable<
            Id = &'__a __FK,
        >,
    {
        type ForeignKey = __FK;
        type ForeignKeyColumn = procedure_template_asset_models::asset_model_id;
        fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
            std::convert::Into::into(&self.asset_model_id)
        }
        fn foreign_key_column() -> Self::ForeignKeyColumn {
            procedure_template_asset_models::asset_model_id
        }
    }
};
/// Table to store procedure template asset models
#[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
pub mod procedure_template_asset_models {
    use ::diesel;
    pub use self::columns::*;
    use diesel::sql_types::*;
    ///Re-exports all of the columns of this table, as well as the
    ///table struct renamed to the module name. This is meant to be
    ///glob imported for functions which only deal with one table.
    pub mod dsl {
        pub use super::columns::id;
        pub use super::columns::name;
        pub use super::columns::procedure_template_id;
        pub use super::columns::based_on_id;
        pub use super::columns::asset_model_id;
        pub use super::table as procedure_template_asset_models;
    }
    #[allow(non_upper_case_globals, dead_code)]
    ///A tuple of all of the columns on thistable
    pub const all_columns: (
        id,
        name,
        procedure_template_id,
        based_on_id,
        asset_model_id,
    ) = (id, name, procedure_template_id, based_on_id, asset_model_id);
    #[allow(non_camel_case_types)]
    ///The actual table struct
    ///
    /// This is the type which provides the base methods of the query
    /// builder, such as `.select` and `.filter`.
    pub struct table;
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::fmt::Debug for table {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "table")
        }
    }
    #[automatically_derived]
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    unsafe impl ::core::clone::TrivialClone for table {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for table {
        #[inline]
        fn clone(&self) -> table {
            *self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for table {}
    const _: () = {
        use diesel;
        #[allow(non_camel_case_types)]
        impl diesel::query_builder::QueryId for table {
            type QueryId = table;
            const HAS_STATIC_QUERY_ID: bool = true;
            const IS_WINDOW_FUNCTION: bool = false;
        }
    };
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::default::Default for table {
        #[inline]
        fn default() -> table {
            table {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for table {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for table {
        #[inline]
        fn eq(&self, other: &table) -> bool {
            true
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Eq for table {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) {}
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialOrd for table {
        #[inline]
        fn partial_cmp(
            &self,
            other: &table,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Ord for table {
        #[inline]
        fn cmp(&self, other: &table) -> ::core::cmp::Ordering {
            ::core::cmp::Ordering::Equal
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::hash::Hash for table {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {}
    }
    impl table {
        #[allow(dead_code)]
        ///Represents `table_name.*`, which is sometimes necessary
        /// for efficient count queries. It cannot be used in place of
        /// `all_columns`
        pub fn star(&self) -> star {
            star
        }
    }
    ///The SQL type of all of the columns on this table
    pub type SqlType = (
        ::rosetta_uuid::diesel_impls::Uuid,
        diesel::sql_types::Text,
        ::rosetta_uuid::diesel_impls::Uuid,
        ::diesel::sql_types::Nullable<::rosetta_uuid::diesel_impls::Uuid>,
        ::rosetta_uuid::diesel_impls::Uuid,
    );
    ///Helper type for representing a boxed query from this table
    pub type BoxedQuery<'a, DB, ST = SqlType> = diesel::internal::table_macro::BoxedSelectStatement<
        'a,
        ST,
        diesel::internal::table_macro::FromClause<table>,
        DB,
    >;
    impl diesel::QuerySource for table {
        type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<
            table,
        >;
        type DefaultSelection = <Self as diesel::query_source::QueryRelation>::AllColumns;
        fn from_clause(&self) -> Self::FromClause {
            diesel::internal::table_macro::StaticQueryFragmentInstance::new()
        }
        fn default_selection(&self) -> Self::DefaultSelection {
            <Self as diesel::query_source::QueryRelation>::all_columns()
        }
    }
    impl diesel::internal::table_macro::PlainQuerySource for table {}
    impl<DB> diesel::query_builder::QueryFragment<DB> for table
    where
        DB: diesel::backend::Backend,
        <Self as diesel::internal::table_macro::StaticQueryFragment>::Component: diesel::query_builder::QueryFragment<
            DB,
        >,
    {
        fn walk_ast<'b>(
            &'b self,
            __diesel_internal_pass: diesel::query_builder::AstPass<'_, 'b, DB>,
        ) -> diesel::result::QueryResult<()> {
            <Self as diesel::internal::table_macro::StaticQueryFragment>::STATIC_COMPONENT
                .walk_ast(__diesel_internal_pass)
        }
    }
    impl diesel::internal::table_macro::StaticQueryFragment for table {
        type Component = diesel::internal::table_macro::Identifier<'static>;
        const STATIC_COMPONENT: &'static Self::Component = &diesel::internal::table_macro::Identifier(
            "procedure_template_asset_models",
        );
    }
    impl diesel::query_builder::AsQuery for table {
        type SqlType = SqlType;
        type Query = diesel::internal::table_macro::SelectStatement<
            diesel::internal::table_macro::FromClause<Self>,
        >;
        fn as_query(self) -> Self::Query {
            diesel::internal::table_macro::SelectStatement::simple(self)
        }
    }
    impl diesel::Table for table {
        type PrimaryKey = id;
        type AllColumns = (id, name, procedure_template_id, based_on_id, asset_model_id);
        fn primary_key(&self) -> Self::PrimaryKey {
            id
        }
        fn all_columns() -> Self::AllColumns {
            (id, name, procedure_template_id, based_on_id, asset_model_id)
        }
    }
    impl diesel::associations::HasTable for table {
        type Table = Self;
        fn table() -> Self::Table {
            table
        }
    }
    impl diesel::query_builder::IntoUpdateTarget for table {
        type WhereClause = <<Self as diesel::query_builder::AsQuery>::Query as diesel::query_builder::IntoUpdateTarget>::WhereClause;
        fn into_update_target(
            self,
        ) -> diesel::query_builder::UpdateTarget<Self::Table, Self::WhereClause> {
            use diesel::query_builder::AsQuery;
            let q: diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<table>,
            > = self.as_query();
            q.into_update_target()
        }
    }
    impl<T> diesel::insertable::Insertable<T> for table
    where
        <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<
            T,
        >,
    {
        type Values = <<table as diesel::query_builder::AsQuery>::Query as diesel::insertable::Insertable<
            T,
        >>::Values;
        fn values(self) -> Self::Values {
            use diesel::query_builder::AsQuery;
            self.as_query().values()
        }
    }
    impl<'a, T> diesel::insertable::Insertable<T> for &'a table
    where
        table: diesel::insertable::Insertable<T>,
    {
        type Values = <table as diesel::insertable::Insertable<T>>::Values;
        fn values(self) -> Self::Values {
            (*self).values()
        }
    }
    impl diesel::query_source::AppearsInFromClause<Self> for table {
        type Count = diesel::query_source::Once;
    }
    impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, Self> for table
    where
        S: diesel::query_source::AliasSource<Target = Self>,
    {
        type Count = diesel::query_source::Never;
    }
    impl<
        S1,
        S2,
    > diesel::internal::table_macro::AliasAliasAppearsInFromClause<Self, S2, S1>
    for table
    where
        S1: diesel::query_source::AliasSource<Target = Self>,
        S2: diesel::query_source::AliasSource<Target = Self>,
        S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
            S2,
            Self,
        >,
    {
        type Count = <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
            S2,
            Self,
        >>::Count;
    }
    impl<S> diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>>
    for table
    where
        S: diesel::query_source::AliasSource,
    {
        type Count = diesel::query_source::Never;
    }
    impl<
        S,
        C,
    > diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<
        Self,
        S,
        C,
    > for table
    where
        S: diesel::query_source::AliasSource<Target = Self> + ::std::clone::Clone,
        C: diesel::query_source::QueryRelationField<QueryRelation = Self>,
    {
        type Out = diesel::query_source::AliasedField<S, C>;
        fn map(
            __diesel_internal_column: C,
            __diesel_internal_alias: &diesel::query_source::Alias<S>,
        ) -> Self::Out {
            __diesel_internal_alias.field(__diesel_internal_column)
        }
    }
    impl diesel::query_source::AppearsInFromClause<table>
    for diesel::internal::table_macro::NoFromClause {
        type Count = diesel::query_source::Never;
    }
    impl<
        Left,
        Right,
        Kind,
    > diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>> for table
    where
        diesel::internal::table_macro::Join<Left, Right, Kind>: diesel::JoinTo<Self>,
        Left: diesel::query_source::QuerySource,
        Right: diesel::query_source::QuerySource,
    {
        type FromClause = diesel::internal::table_macro::Join<Left, Right, Kind>;
        type OnClause = <diesel::internal::table_macro::Join<
            Left,
            Right,
            Kind,
        > as diesel::JoinTo<Self>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::internal::table_macro::Join<Left, Right, Kind>,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::Join::join_target(
                Self,
            );
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl<Join, On> diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>>
    for table
    where
        diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<Self>,
    {
        type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
        type OnClause = <diesel::internal::table_macro::JoinOn<
            Join,
            On,
        > as diesel::JoinTo<Self>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::internal::table_macro::JoinOn<Join, On>,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::JoinOn::join_target(
                Self,
            );
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl<
        F,
        S,
        D,
        W,
        O,
        L,
        Of,
        G,
    > diesel::JoinTo<
        diesel::internal::table_macro::SelectStatement<
            diesel::internal::table_macro::FromClause<F>,
            S,
            D,
            W,
            O,
            L,
            Of,
            G,
        >,
    > for table
    where
        diesel::internal::table_macro::SelectStatement<
            diesel::internal::table_macro::FromClause<F>,
            S,
            D,
            W,
            O,
            L,
            Of,
            G,
        >: diesel::JoinTo<Self>,
        F: diesel::query_source::QuerySource,
    {
        type FromClause = diesel::internal::table_macro::SelectStatement<
            diesel::internal::table_macro::FromClause<F>,
            S,
            D,
            W,
            O,
            L,
            Of,
            G,
        >;
        type OnClause = <diesel::internal::table_macro::SelectStatement<
            diesel::internal::table_macro::FromClause<F>,
            S,
            D,
            W,
            O,
            L,
            Of,
            G,
        > as diesel::JoinTo<Self>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<F>,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            >,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::SelectStatement::join_target(
                Self,
            );
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl<
        'a,
        QS,
        ST,
        DB,
    > diesel::JoinTo<
        diesel::internal::table_macro::BoxedSelectStatement<
            'a,
            diesel::internal::table_macro::FromClause<QS>,
            ST,
            DB,
        >,
    > for table
    where
        diesel::internal::table_macro::BoxedSelectStatement<
            'a,
            diesel::internal::table_macro::FromClause<QS>,
            ST,
            DB,
        >: diesel::JoinTo<Self>,
        QS: diesel::query_source::QuerySource,
    {
        type FromClause = diesel::internal::table_macro::BoxedSelectStatement<
            'a,
            diesel::internal::table_macro::FromClause<QS>,
            ST,
            DB,
        >;
        type OnClause = <diesel::internal::table_macro::BoxedSelectStatement<
            'a,
            diesel::internal::table_macro::FromClause<QS>,
            ST,
            DB,
        > as diesel::JoinTo<Self>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                diesel::internal::table_macro::FromClause<QS>,
                ST,
                DB,
            >,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::BoxedSelectStatement::join_target(
                Self,
            );
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
    where
        diesel::query_source::Alias<S>: diesel::JoinTo<Self>,
    {
        type FromClause = diesel::query_source::Alias<S>;
        type OnClause = <diesel::query_source::Alias<
            S,
        > as diesel::JoinTo<Self>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::query_source::Alias<S>,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) = diesel::query_source::Alias::<
                S,
            >::join_target(Self);
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    ///Contains all of the columns of this table
    pub mod columns {
        use ::diesel;
        use super::table;
        use diesel::sql_types::*;
        #[allow(non_camel_case_types, dead_code)]
        ///Represents `table_name.*`, which is sometimes needed for
        /// efficient count queries. It cannot be used in place of
        /// `all_columns`, and has a `SqlType` of `()` to prevent it
        /// being used that way
        pub struct star;
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::fmt::Debug for star {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "star")
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        #[allow(non_camel_case_types, dead_code)]
        unsafe impl ::core::clone::TrivialClone for star {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::clone::Clone for star {
            #[inline]
            fn clone(&self) -> star {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::Copy for star {}
        const _: () = {
            use diesel;
            #[allow(non_camel_case_types)]
            impl diesel::query_builder::QueryId for star {
                type QueryId = star;
                const HAS_STATIC_QUERY_ID: bool = true;
                const IS_WINDOW_FUNCTION: bool = false;
            }
        };
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::StructuralPartialEq for star {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialEq for star {
            #[inline]
            fn eq(&self, other: &star) -> bool {
                true
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Eq for star {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {}
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialOrd for star {
            #[inline]
            fn partial_cmp(
                &self,
                other: &star,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Ord for star {
            #[inline]
            fn cmp(&self, other: &star) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::hash::Hash for star {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {}
        }
        impl<__GB> diesel::expression::ValidGrouping<__GB> for star
        where
            (
                id,
                name,
                procedure_template_id,
                based_on_id,
                asset_model_id,
            ): diesel::expression::ValidGrouping<__GB>,
        {
            type IsAggregate = <(
                id,
                name,
                procedure_template_id,
                based_on_id,
                asset_model_id,
            ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
        }
        impl diesel::Expression for star {
            type SqlType = diesel::expression::expression_types::NotSelectable;
        }
        impl<DB: diesel::backend::Backend> diesel::query_builder::QueryFragment<DB>
        for star
        where
            <table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<
                DB,
            >,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                use diesel::QuerySource;
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_sql("*");
                Ok(())
            }
        }
        impl diesel::SelectableExpression<table> for star {}
        impl diesel::AppearsOnTable<table> for star {}
        /// Identifier of the procedure template asset model
        #[allow(non_camel_case_types, dead_code)]
        pub struct id;
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::fmt::Debug for id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "id")
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        #[allow(non_camel_case_types, dead_code)]
        unsafe impl ::core::clone::TrivialClone for id {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::clone::Clone for id {
            #[inline]
            fn clone(&self) -> id {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::Copy for id {}
        const _: () = {
            use diesel;
            #[allow(non_camel_case_types)]
            impl diesel::query_builder::QueryId for id {
                type QueryId = id;
                const HAS_STATIC_QUERY_ID: bool = true;
                const IS_WINDOW_FUNCTION: bool = false;
            }
        };
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::default::Default for id {
            #[inline]
            fn default() -> id {
                id {}
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::StructuralPartialEq for id {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialEq for id {
            #[inline]
            fn eq(&self, other: &id) -> bool {
                true
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Eq for id {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {}
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialOrd for id {
            #[inline]
            fn partial_cmp(
                &self,
                other: &id,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Ord for id {
            #[inline]
            fn cmp(&self, other: &id) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::hash::Hash for id {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {}
        }
        impl diesel::expression::Expression for id {
            type SqlType = ::rosetta_uuid::diesel_impls::Uuid;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for id
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<
                table,
            >: diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("id")
            }
        }
        impl diesel::SelectableExpression<super::table> for id {}
        impl<QS> diesel::AppearsOnTable<QS> for id
        where
            QS: diesel::query_source::AppearsInFromClause<
                super::table,
                Count = diesel::query_source::Once,
            >,
        {}
        impl<
            Left,
            Right,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::Join<
                Left,
                Right,
                diesel::internal::table_macro::LeftOuter,
            >,
        > for id
        where
            id: diesel::AppearsOnTable<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {}
        impl<
            Left,
            Right,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::Join<
                Left,
                Right,
                diesel::internal::table_macro::Inner,
            >,
        > for id
        where
            id: diesel::AppearsOnTable<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (
                Left::Count,
                Right::Count,
            ): diesel::internal::table_macro::Pick<Left, Right>,
            Self: diesel::SelectableExpression<
                <(
                    Left::Count,
                    Right::Count,
                ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
            >,
        {}
        impl<
            Join,
            On,
        > diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
        for id
        where
            id: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                >,
        {}
        impl<
            From,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<From>,
            >,
        > for id
        where
            From: diesel::query_source::QuerySource,
            id: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {}
        impl<__GB> diesel::expression::ValidGrouping<__GB> for id
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                id,
                Output = diesel::expression::is_contained_in_group_by::Yes,
            >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for id {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl<T> diesel::EqAll<T> for id
        where
            T: diesel::expression::AsExpression<::rosetta_uuid::diesel_impls::Uuid>,
            diesel::dsl::Eq<
                id,
                T::Expression,
            >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl diesel::query_source::Column for id {
            type Table = super::table;
            const NAME: &'static str = "id";
        }
        /// The name of the procedure template asset model
        #[allow(non_camel_case_types, dead_code)]
        pub struct name;
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::fmt::Debug for name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "name")
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        #[allow(non_camel_case_types, dead_code)]
        unsafe impl ::core::clone::TrivialClone for name {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::clone::Clone for name {
            #[inline]
            fn clone(&self) -> name {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::Copy for name {}
        const _: () = {
            use diesel;
            #[allow(non_camel_case_types)]
            impl diesel::query_builder::QueryId for name {
                type QueryId = name;
                const HAS_STATIC_QUERY_ID: bool = true;
                const IS_WINDOW_FUNCTION: bool = false;
            }
        };
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::default::Default for name {
            #[inline]
            fn default() -> name {
                name {}
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::StructuralPartialEq for name {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialEq for name {
            #[inline]
            fn eq(&self, other: &name) -> bool {
                true
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Eq for name {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {}
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialOrd for name {
            #[inline]
            fn partial_cmp(
                &self,
                other: &name,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Ord for name {
            #[inline]
            fn cmp(&self, other: &name) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::hash::Hash for name {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {}
        }
        impl diesel::expression::Expression for name {
            type SqlType = diesel::sql_types::Text;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for name
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<
                table,
            >: diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("name")
            }
        }
        impl diesel::SelectableExpression<super::table> for name {}
        impl<QS> diesel::AppearsOnTable<QS> for name
        where
            QS: diesel::query_source::AppearsInFromClause<
                super::table,
                Count = diesel::query_source::Once,
            >,
        {}
        impl<
            Left,
            Right,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::Join<
                Left,
                Right,
                diesel::internal::table_macro::LeftOuter,
            >,
        > for name
        where
            name: diesel::AppearsOnTable<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {}
        impl<
            Left,
            Right,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::Join<
                Left,
                Right,
                diesel::internal::table_macro::Inner,
            >,
        > for name
        where
            name: diesel::AppearsOnTable<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (
                Left::Count,
                Right::Count,
            ): diesel::internal::table_macro::Pick<Left, Right>,
            Self: diesel::SelectableExpression<
                <(
                    Left::Count,
                    Right::Count,
                ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
            >,
        {}
        impl<
            Join,
            On,
        > diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
        for name
        where
            name: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                >,
        {}
        impl<
            From,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<From>,
            >,
        > for name
        where
            From: diesel::query_source::QuerySource,
            name: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {}
        impl<__GB> diesel::expression::ValidGrouping<__GB> for name
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                name,
                Output = diesel::expression::is_contained_in_group_by::Yes,
            >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for name {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for name {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl<T> diesel::EqAll<T> for name
        where
            T: diesel::expression::AsExpression<diesel::sql_types::Text>,
            diesel::dsl::Eq<
                name,
                T::Expression,
            >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl diesel::query_source::Column for name {
            type Table = super::table;
            const NAME: &'static str = "name";
        }
        /// Procedure template this asset model is associated with
        #[allow(non_camel_case_types, dead_code)]
        pub struct procedure_template_id;
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::fmt::Debug for procedure_template_id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "procedure_template_id")
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        #[allow(non_camel_case_types, dead_code)]
        unsafe impl ::core::clone::TrivialClone for procedure_template_id {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::clone::Clone for procedure_template_id {
            #[inline]
            fn clone(&self) -> procedure_template_id {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::Copy for procedure_template_id {}
        const _: () = {
            use diesel;
            #[allow(non_camel_case_types)]
            impl diesel::query_builder::QueryId for procedure_template_id {
                type QueryId = procedure_template_id;
                const HAS_STATIC_QUERY_ID: bool = true;
                const IS_WINDOW_FUNCTION: bool = false;
            }
        };
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::default::Default for procedure_template_id {
            #[inline]
            fn default() -> procedure_template_id {
                procedure_template_id {}
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::StructuralPartialEq for procedure_template_id {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialEq for procedure_template_id {
            #[inline]
            fn eq(&self, other: &procedure_template_id) -> bool {
                true
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Eq for procedure_template_id {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {}
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialOrd for procedure_template_id {
            #[inline]
            fn partial_cmp(
                &self,
                other: &procedure_template_id,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Ord for procedure_template_id {
            #[inline]
            fn cmp(&self, other: &procedure_template_id) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::hash::Hash for procedure_template_id {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {}
        }
        impl diesel::expression::Expression for procedure_template_id {
            type SqlType = ::rosetta_uuid::diesel_impls::Uuid;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for procedure_template_id
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<
                table,
            >: diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("procedure_template_id")
            }
        }
        impl diesel::SelectableExpression<super::table> for procedure_template_id {}
        impl<QS> diesel::AppearsOnTable<QS> for procedure_template_id
        where
            QS: diesel::query_source::AppearsInFromClause<
                super::table,
                Count = diesel::query_source::Once,
            >,
        {}
        impl<
            Left,
            Right,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::Join<
                Left,
                Right,
                diesel::internal::table_macro::LeftOuter,
            >,
        > for procedure_template_id
        where
            procedure_template_id: diesel::AppearsOnTable<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {}
        impl<
            Left,
            Right,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::Join<
                Left,
                Right,
                diesel::internal::table_macro::Inner,
            >,
        > for procedure_template_id
        where
            procedure_template_id: diesel::AppearsOnTable<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (
                Left::Count,
                Right::Count,
            ): diesel::internal::table_macro::Pick<Left, Right>,
            Self: diesel::SelectableExpression<
                <(
                    Left::Count,
                    Right::Count,
                ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
            >,
        {}
        impl<
            Join,
            On,
        > diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
        for procedure_template_id
        where
            procedure_template_id: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                >,
        {}
        impl<
            From,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<From>,
            >,
        > for procedure_template_id
        where
            From: diesel::query_source::QuerySource,
            procedure_template_id: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {}
        impl<__GB> diesel::expression::ValidGrouping<__GB> for procedure_template_id
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                procedure_template_id,
                Output = diesel::expression::is_contained_in_group_by::Yes,
            >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for procedure_template_id {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<procedure_template_id>
        for procedure_template_id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl<T> diesel::EqAll<T> for procedure_template_id
        where
            T: diesel::expression::AsExpression<::rosetta_uuid::diesel_impls::Uuid>,
            diesel::dsl::Eq<
                procedure_template_id,
                T::Expression,
            >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl diesel::query_source::Column for procedure_template_id {
            type Table = super::table;
            const NAME: &'static str = "procedure_template_id";
        }
        /// which this procedure template asset model is based on
        #[allow(non_camel_case_types, dead_code)]
        pub struct based_on_id;
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::fmt::Debug for based_on_id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "based_on_id")
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        #[allow(non_camel_case_types, dead_code)]
        unsafe impl ::core::clone::TrivialClone for based_on_id {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::clone::Clone for based_on_id {
            #[inline]
            fn clone(&self) -> based_on_id {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::Copy for based_on_id {}
        const _: () = {
            use diesel;
            #[allow(non_camel_case_types)]
            impl diesel::query_builder::QueryId for based_on_id {
                type QueryId = based_on_id;
                const HAS_STATIC_QUERY_ID: bool = true;
                const IS_WINDOW_FUNCTION: bool = false;
            }
        };
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::default::Default for based_on_id {
            #[inline]
            fn default() -> based_on_id {
                based_on_id {}
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::StructuralPartialEq for based_on_id {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialEq for based_on_id {
            #[inline]
            fn eq(&self, other: &based_on_id) -> bool {
                true
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Eq for based_on_id {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {}
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialOrd for based_on_id {
            #[inline]
            fn partial_cmp(
                &self,
                other: &based_on_id,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Ord for based_on_id {
            #[inline]
            fn cmp(&self, other: &based_on_id) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::hash::Hash for based_on_id {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {}
        }
        impl diesel::expression::Expression for based_on_id {
            type SqlType = ::diesel::sql_types::Nullable<
                ::rosetta_uuid::diesel_impls::Uuid,
            >;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for based_on_id
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<
                table,
            >: diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("based_on_id")
            }
        }
        impl diesel::SelectableExpression<super::table> for based_on_id {}
        impl<QS> diesel::AppearsOnTable<QS> for based_on_id
        where
            QS: diesel::query_source::AppearsInFromClause<
                super::table,
                Count = diesel::query_source::Once,
            >,
        {}
        impl<
            Left,
            Right,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::Join<
                Left,
                Right,
                diesel::internal::table_macro::LeftOuter,
            >,
        > for based_on_id
        where
            based_on_id: diesel::AppearsOnTable<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {}
        impl<
            Left,
            Right,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::Join<
                Left,
                Right,
                diesel::internal::table_macro::Inner,
            >,
        > for based_on_id
        where
            based_on_id: diesel::AppearsOnTable<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (
                Left::Count,
                Right::Count,
            ): diesel::internal::table_macro::Pick<Left, Right>,
            Self: diesel::SelectableExpression<
                <(
                    Left::Count,
                    Right::Count,
                ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
            >,
        {}
        impl<
            Join,
            On,
        > diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
        for based_on_id
        where
            based_on_id: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                >,
        {}
        impl<
            From,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<From>,
            >,
        > for based_on_id
        where
            From: diesel::query_source::QuerySource,
            based_on_id: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {}
        impl<__GB> diesel::expression::ValidGrouping<__GB> for based_on_id
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                based_on_id,
                Output = diesel::expression::is_contained_in_group_by::Yes,
            >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for based_on_id {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<based_on_id> for based_on_id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl<T> diesel::EqAll<T> for based_on_id
        where
            T: diesel::expression::AsExpression<
                ::diesel::sql_types::Nullable<::rosetta_uuid::diesel_impls::Uuid>,
            >,
            diesel::dsl::Eq<
                based_on_id,
                T::Expression,
            >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl diesel::query_source::Column for based_on_id {
            type Table = super::table;
            const NAME: &'static str = "based_on_id";
        }
        /// The asset model this procedure template asset model is associated with
        #[allow(non_camel_case_types, dead_code)]
        pub struct asset_model_id;
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::fmt::Debug for asset_model_id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "asset_model_id")
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        #[allow(non_camel_case_types, dead_code)]
        unsafe impl ::core::clone::TrivialClone for asset_model_id {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::clone::Clone for asset_model_id {
            #[inline]
            fn clone(&self) -> asset_model_id {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::Copy for asset_model_id {}
        const _: () = {
            use diesel;
            #[allow(non_camel_case_types)]
            impl diesel::query_builder::QueryId for asset_model_id {
                type QueryId = asset_model_id;
                const HAS_STATIC_QUERY_ID: bool = true;
                const IS_WINDOW_FUNCTION: bool = false;
            }
        };
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::default::Default for asset_model_id {
            #[inline]
            fn default() -> asset_model_id {
                asset_model_id {}
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::StructuralPartialEq for asset_model_id {}
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialEq for asset_model_id {
            #[inline]
            fn eq(&self, other: &asset_model_id) -> bool {
                true
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Eq for asset_model_id {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {}
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::PartialOrd for asset_model_id {
            #[inline]
            fn partial_cmp(
                &self,
                other: &asset_model_id,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::cmp::Ord for asset_model_id {
            #[inline]
            fn cmp(&self, other: &asset_model_id) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::hash::Hash for asset_model_id {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {}
        }
        impl diesel::expression::Expression for asset_model_id {
            type SqlType = ::rosetta_uuid::diesel_impls::Uuid;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for asset_model_id
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<
                table,
            >: diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("asset_model_id")
            }
        }
        impl diesel::SelectableExpression<super::table> for asset_model_id {}
        impl<QS> diesel::AppearsOnTable<QS> for asset_model_id
        where
            QS: diesel::query_source::AppearsInFromClause<
                super::table,
                Count = diesel::query_source::Once,
            >,
        {}
        impl<
            Left,
            Right,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::Join<
                Left,
                Right,
                diesel::internal::table_macro::LeftOuter,
            >,
        > for asset_model_id
        where
            asset_model_id: diesel::AppearsOnTable<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {}
        impl<
            Left,
            Right,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::Join<
                Left,
                Right,
                diesel::internal::table_macro::Inner,
            >,
        > for asset_model_id
        where
            asset_model_id: diesel::AppearsOnTable<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (
                Left::Count,
                Right::Count,
            ): diesel::internal::table_macro::Pick<Left, Right>,
            Self: diesel::SelectableExpression<
                <(
                    Left::Count,
                    Right::Count,
                ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
            >,
        {}
        impl<
            Join,
            On,
        > diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
        for asset_model_id
        where
            asset_model_id: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                >,
        {}
        impl<
            From,
        > diesel::SelectableExpression<
            diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<From>,
            >,
        > for asset_model_id
        where
            From: diesel::query_source::QuerySource,
            asset_model_id: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {}
        impl<__GB> diesel::expression::ValidGrouping<__GB> for asset_model_id
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                asset_model_id,
                Output = diesel::expression::is_contained_in_group_by::Yes,
            >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for asset_model_id {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<asset_model_id>
        for asset_model_id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl<T> diesel::EqAll<T> for asset_model_id
        where
            T: diesel::expression::AsExpression<::rosetta_uuid::diesel_impls::Uuid>,
            diesel::dsl::Eq<
                asset_model_id,
                T::Expression,
            >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl diesel::query_source::Column for asset_model_id {
            type Table = super::table;
            const NAME: &'static str = "asset_model_id";
        }
        impl diesel::expression::IsContainedInGroupBy<id> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for procedure_template_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<procedure_template_id> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for based_on_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<based_on_id> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for asset_model_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<asset_model_id> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for procedure_template_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<procedure_template_id> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for based_on_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<based_on_id> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for asset_model_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<asset_model_id> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<procedure_template_id>
        for based_on_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<based_on_id>
        for procedure_template_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<procedure_template_id>
        for asset_model_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<asset_model_id>
        for procedure_template_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<based_on_id> for asset_model_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<asset_model_id> for based_on_id {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
    }
}
///Trait to set the `id` column on a [`procedure_template_asset_models`] table builder.
pub trait SetProcedureTemplateAssetModelId: diesel_builders::SetColumn<
        procedure_template_asset_models::id,
    > + Sized {
    #[inline]
    ///Sets the `id` column on a [`procedure_template_asset_models`] table builder by reference.
    fn id_ref(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> &mut Self {
        use diesel_builders::SetColumnExt;
        self.set_column_ref::<procedure_template_asset_models::id>(value)
    }
    #[inline]
    #[must_use]
    ///Sets the `id` column on a [`procedure_template_asset_models`] table builder.
    fn id(
        self,
        value: impl Into<
            <procedure_template_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> Self {
        use diesel_builders::SetColumnExt;
        self.set_column::<procedure_template_asset_models::id>(value)
    }
}
impl<T> SetProcedureTemplateAssetModelId for T
where
    T: diesel_builders::SetColumn<procedure_template_asset_models::id>,
{}
///Trait to try to set the `id` column on a table builder.
pub trait TrySetProcedureTemplateAssetModelId: diesel_builders::TrySetColumn<
        procedure_template_asset_models::id,
    > + Sized {
    #[inline]
    ///Tries to set the `id` column on a table builder by reference.
    ///
    /// # Errors
    ///
    ///Returns an error if the column check constraints are not respected.
    fn try_id_ref(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType,
        > + Clone,
    ) -> Result<&mut Self, Self::Error> {
        use diesel_builders::TrySetColumnExt;
        self.try_set_column_ref::<procedure_template_asset_models::id>(value)
    }
    #[inline]
    ///Tries to set the `id` column on a table builder.
    ///
    /// # Errors
    ///
    ///Returns an error if the value cannot be converted to the column type.
    fn try_id(
        self,
        value: impl Into<
            <procedure_template_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType,
        > + Clone,
    ) -> Result<Self, Self::Error> {
        use diesel_builders::TrySetColumnExt;
        self.try_set_column::<procedure_template_asset_models::id>(value)
    }
}
impl<T> TrySetProcedureTemplateAssetModelId for T
where
    T: diesel_builders::TrySetColumn<procedure_template_asset_models::id>,
{}
impl ::diesel_builders::ValueTyped for procedure_template_asset_models::id {
    type ValueType = ::rosetta_uuid::Uuid;
}
impl ::diesel_builders::ColumnTyped for procedure_template_asset_models::id {
    type ColumnType = ::rosetta_uuid::Uuid;
}
///Trait to get the `name` column from a `procedure_template_asset_models` table model.
pub trait GetProcedureTemplateAssetModelName: ::diesel_builders::GetColumn<
        procedure_template_asset_models::name,
    > {
    #[inline]
    ///Gets the value of the `name` column from a `procedure_template_asset_models` table model.
    fn name(
        &self,
    ) -> &<procedure_template_asset_models::name as ::diesel_builders::ColumnTyped>::ColumnType {
        self.get_column_ref()
    }
}
impl<T> GetProcedureTemplateAssetModelName for T
where
    T: ::diesel_builders::GetColumn<procedure_template_asset_models::name>,
{}
///Trait to set the `name` column on a [`procedure_template_asset_models`] table builder.
pub trait SetProcedureTemplateAssetModelName: diesel_builders::SetColumn<
        procedure_template_asset_models::name,
    > + Sized {
    #[inline]
    ///Sets the `name` column on a [`procedure_template_asset_models`] table builder by reference.
    fn name_ref(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::name as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> &mut Self {
        use diesel_builders::SetColumnExt;
        self.set_column_ref::<procedure_template_asset_models::name>(value)
    }
    #[inline]
    #[must_use]
    ///Sets the `name` column on a [`procedure_template_asset_models`] table builder.
    fn name(
        self,
        value: impl Into<
            <procedure_template_asset_models::name as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> Self {
        use diesel_builders::SetColumnExt;
        self.set_column::<procedure_template_asset_models::name>(value)
    }
}
impl<T> SetProcedureTemplateAssetModelName for T
where
    T: diesel_builders::SetColumn<procedure_template_asset_models::name>,
{}
///Trait to try to set the `name` column on a table builder.
pub trait TrySetProcedureTemplateAssetModelName: diesel_builders::TrySetColumn<
        procedure_template_asset_models::name,
    > + Sized {
    #[inline]
    ///Tries to set the `name` column on a table builder by reference.
    ///
    /// # Errors
    ///
    ///Returns an error if the column check constraints are not respected.
    fn try_name_ref(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::name as ::diesel_builders::ColumnTyped>::ColumnType,
        > + Clone,
    ) -> Result<&mut Self, Self::Error> {
        use diesel_builders::TrySetColumnExt;
        self.try_set_column_ref::<procedure_template_asset_models::name>(value)
    }
    #[inline]
    ///Tries to set the `name` column on a table builder.
    ///
    /// # Errors
    ///
    ///Returns an error if the value cannot be converted to the column type.
    fn try_name(
        self,
        value: impl Into<
            <procedure_template_asset_models::name as ::diesel_builders::ColumnTyped>::ColumnType,
        > + Clone,
    ) -> Result<Self, Self::Error> {
        use diesel_builders::TrySetColumnExt;
        self.try_set_column::<procedure_template_asset_models::name>(value)
    }
}
impl<T> TrySetProcedureTemplateAssetModelName for T
where
    T: diesel_builders::TrySetColumn<procedure_template_asset_models::name>,
{}
impl ::diesel_builders::ValueTyped for procedure_template_asset_models::name {
    type ValueType = String;
}
impl ::diesel_builders::ColumnTyped for procedure_template_asset_models::name {
    type ColumnType = String;
}
///Trait to get the `procedure_template_id` column from a `procedure_template_asset_models` table model.
pub trait GetProcedureTemplateAssetModelProcedureTemplateId: ::diesel_builders::GetColumn<
        procedure_template_asset_models::procedure_template_id,
    > {
    #[inline]
    ///Gets the value of the `procedure_template_id` column from a `procedure_template_asset_models` table model.
    fn procedure_template_id(
        &self,
    ) -> &<procedure_template_asset_models::procedure_template_id as ::diesel_builders::ColumnTyped>::ColumnType {
        self.get_column_ref()
    }
}
impl<T> GetProcedureTemplateAssetModelProcedureTemplateId for T
where
    T: ::diesel_builders::GetColumn<
        procedure_template_asset_models::procedure_template_id,
    >,
{}
///Trait to set the `procedure_template_id` column on a [`procedure_template_asset_models`] table builder.
pub trait SetProcedureTemplateAssetModelProcedureTemplateId: diesel_builders::SetColumn<
        procedure_template_asset_models::procedure_template_id,
    > + Sized {
    #[inline]
    ///Sets the `procedure_template_id` column on a [`procedure_template_asset_models`] table builder by reference.
    fn procedure_template_id_ref(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::procedure_template_id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> &mut Self {
        use diesel_builders::SetColumnExt;
        self.set_column_ref::<
                procedure_template_asset_models::procedure_template_id,
            >(value)
    }
    #[inline]
    #[must_use]
    ///Sets the `procedure_template_id` column on a [`procedure_template_asset_models`] table builder.
    fn procedure_template_id(
        self,
        value: impl Into<
            <procedure_template_asset_models::procedure_template_id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> Self {
        use diesel_builders::SetColumnExt;
        self.set_column::<procedure_template_asset_models::procedure_template_id>(value)
    }
}
impl<T> SetProcedureTemplateAssetModelProcedureTemplateId for T
where
    T: diesel_builders::SetColumn<
        procedure_template_asset_models::procedure_template_id,
    >,
{}
///Trait to try to set the `procedure_template_id` column on a table builder.
pub trait TrySetProcedureTemplateAssetModelProcedureTemplateId: diesel_builders::TrySetColumn<
        procedure_template_asset_models::procedure_template_id,
    > + Sized {
    #[inline]
    ///Tries to set the `procedure_template_id` column on a table builder by reference.
    ///
    /// # Errors
    ///
    ///Returns an error if the column check constraints are not respected.
    fn try_procedure_template_id_ref(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::procedure_template_id as ::diesel_builders::ColumnTyped>::ColumnType,
        > + Clone,
    ) -> Result<&mut Self, Self::Error> {
        use diesel_builders::TrySetColumnExt;
        self.try_set_column_ref::<
                procedure_template_asset_models::procedure_template_id,
            >(value)
    }
    #[inline]
    ///Tries to set the `procedure_template_id` column on a table builder.
    ///
    /// # Errors
    ///
    ///Returns an error if the value cannot be converted to the column type.
    fn try_procedure_template_id(
        self,
        value: impl Into<
            <procedure_template_asset_models::procedure_template_id as ::diesel_builders::ColumnTyped>::ColumnType,
        > + Clone,
    ) -> Result<Self, Self::Error> {
        use diesel_builders::TrySetColumnExt;
        self.try_set_column::<
                procedure_template_asset_models::procedure_template_id,
            >(value)
    }
}
impl<T> TrySetProcedureTemplateAssetModelProcedureTemplateId for T
where
    T: diesel_builders::TrySetColumn<
        procedure_template_asset_models::procedure_template_id,
    >,
{}
impl ::diesel_builders::ValueTyped
for procedure_template_asset_models::procedure_template_id {
    type ValueType = ::rosetta_uuid::Uuid;
}
impl ::diesel_builders::ColumnTyped
for procedure_template_asset_models::procedure_template_id {
    type ColumnType = ::rosetta_uuid::Uuid;
}
///Trait to get the `based_on_id` column from a `procedure_template_asset_models` table model.
pub trait GetProcedureTemplateAssetModelBasedOnId: ::diesel_builders::GetColumn<
        procedure_template_asset_models::based_on_id,
    > {
    #[inline]
    ///Gets the value of the `based_on_id` column from a `procedure_template_asset_models` table model.
    fn based_on_id(
        &self,
    ) -> &<procedure_template_asset_models::based_on_id as ::diesel_builders::ColumnTyped>::ColumnType {
        self.get_column_ref()
    }
}
impl<T> GetProcedureTemplateAssetModelBasedOnId for T
where
    T: ::diesel_builders::GetColumn<procedure_template_asset_models::based_on_id>,
{}
///Trait to set the `based_on_id` column on a [`procedure_template_asset_models`] table builder.
pub trait SetProcedureTemplateAssetModelBasedOnId: diesel_builders::SetColumn<
        procedure_template_asset_models::based_on_id,
    > + Sized {
    #[inline]
    ///Sets the `based_on_id` column on a [`procedure_template_asset_models`] table builder by reference.
    fn based_on_id_ref(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::based_on_id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> &mut Self {
        use diesel_builders::SetColumnExt;
        self.set_column_ref::<procedure_template_asset_models::based_on_id>(value)
    }
    #[inline]
    #[must_use]
    ///Sets the `based_on_id` column on a [`procedure_template_asset_models`] table builder.
    fn based_on_id(
        self,
        value: impl Into<
            <procedure_template_asset_models::based_on_id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> Self {
        use diesel_builders::SetColumnExt;
        self.set_column::<procedure_template_asset_models::based_on_id>(value)
    }
}
impl<T> SetProcedureTemplateAssetModelBasedOnId for T
where
    T: diesel_builders::SetColumn<procedure_template_asset_models::based_on_id>,
{}
///Trait to try to set the `based_on_id` column on a table builder.
pub trait TrySetProcedureTemplateAssetModelBasedOnId: diesel_builders::TrySetColumn<
        procedure_template_asset_models::based_on_id,
    > + Sized {
    #[inline]
    ///Tries to set the `based_on_id` column on a table builder by reference.
    ///
    /// # Errors
    ///
    ///Returns an error if the column check constraints are not respected.
    fn try_based_on_id_ref(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::based_on_id as ::diesel_builders::ColumnTyped>::ColumnType,
        > + Clone,
    ) -> Result<&mut Self, Self::Error> {
        use diesel_builders::TrySetColumnExt;
        self.try_set_column_ref::<procedure_template_asset_models::based_on_id>(value)
    }
    #[inline]
    ///Tries to set the `based_on_id` column on a table builder.
    ///
    /// # Errors
    ///
    ///Returns an error if the value cannot be converted to the column type.
    fn try_based_on_id(
        self,
        value: impl Into<
            <procedure_template_asset_models::based_on_id as ::diesel_builders::ColumnTyped>::ColumnType,
        > + Clone,
    ) -> Result<Self, Self::Error> {
        use diesel_builders::TrySetColumnExt;
        self.try_set_column::<procedure_template_asset_models::based_on_id>(value)
    }
}
impl<T> TrySetProcedureTemplateAssetModelBasedOnId for T
where
    T: diesel_builders::TrySetColumn<procedure_template_asset_models::based_on_id>,
{}
impl ::diesel_builders::ValueTyped for procedure_template_asset_models::based_on_id {
    type ValueType = ::rosetta_uuid::Uuid;
}
impl ::diesel_builders::ColumnTyped for procedure_template_asset_models::based_on_id {
    type ColumnType = Option<::rosetta_uuid::Uuid>;
}
///Trait to get the `asset_model_id` column from a `procedure_template_asset_models` table model.
pub trait GetProcedureTemplateAssetModelAssetModelId: ::diesel_builders::GetColumn<
        procedure_template_asset_models::asset_model_id,
    > {
    #[inline]
    ///Gets the value of the `asset_model_id` column from a `procedure_template_asset_models` table model.
    fn asset_model_id(
        &self,
    ) -> &<procedure_template_asset_models::asset_model_id as ::diesel_builders::ColumnTyped>::ColumnType {
        self.get_column_ref()
    }
}
impl<T> GetProcedureTemplateAssetModelAssetModelId for T
where
    T: ::diesel_builders::GetColumn<procedure_template_asset_models::asset_model_id>,
{}
///Trait to set the `asset_model_id` column on a [`procedure_template_asset_models`] table builder.
pub trait SetProcedureTemplateAssetModelAssetModelId: diesel_builders::SetColumn<
        procedure_template_asset_models::asset_model_id,
    > + Sized {
    #[inline]
    ///Sets the `asset_model_id` column on a [`procedure_template_asset_models`] table builder by reference.
    fn asset_model_id_ref(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::asset_model_id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> &mut Self {
        use diesel_builders::SetColumnExt;
        self.set_column_ref::<procedure_template_asset_models::asset_model_id>(value)
    }
    #[inline]
    #[must_use]
    ///Sets the `asset_model_id` column on a [`procedure_template_asset_models`] table builder.
    fn asset_model_id(
        self,
        value: impl Into<
            <procedure_template_asset_models::asset_model_id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> Self {
        use diesel_builders::SetColumnExt;
        self.set_column::<procedure_template_asset_models::asset_model_id>(value)
    }
}
impl<T> SetProcedureTemplateAssetModelAssetModelId for T
where
    T: diesel_builders::SetColumn<procedure_template_asset_models::asset_model_id>,
{}
///Trait to try to set the `asset_model_id` column on a table builder.
pub trait TrySetProcedureTemplateAssetModelAssetModelId: diesel_builders::TrySetColumn<
        procedure_template_asset_models::asset_model_id,
    > + Sized {
    #[inline]
    ///Tries to set the `asset_model_id` column on a table builder by reference.
    ///
    /// # Errors
    ///
    ///Returns an error if the column check constraints are not respected.
    fn try_asset_model_id_ref(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::asset_model_id as ::diesel_builders::ColumnTyped>::ColumnType,
        > + Clone,
    ) -> Result<&mut Self, Self::Error> {
        use diesel_builders::TrySetColumnExt;
        self.try_set_column_ref::<procedure_template_asset_models::asset_model_id>(value)
    }
    #[inline]
    ///Tries to set the `asset_model_id` column on a table builder.
    ///
    /// # Errors
    ///
    ///Returns an error if the value cannot be converted to the column type.
    fn try_asset_model_id(
        self,
        value: impl Into<
            <procedure_template_asset_models::asset_model_id as ::diesel_builders::ColumnTyped>::ColumnType,
        > + Clone,
    ) -> Result<Self, Self::Error> {
        use diesel_builders::TrySetColumnExt;
        self.try_set_column::<procedure_template_asset_models::asset_model_id>(value)
    }
}
impl<T> TrySetProcedureTemplateAssetModelAssetModelId for T
where
    T: diesel_builders::TrySetColumn<procedure_template_asset_models::asset_model_id>,
{}
impl ::diesel_builders::ValueTyped for procedure_template_asset_models::asset_model_id {
    type ValueType = ::rosetta_uuid::Uuid;
}
impl ::diesel_builders::ColumnTyped for procedure_template_asset_models::asset_model_id {
    type ColumnType = ::rosetta_uuid::Uuid;
}
impl ::diesel_builders::GetColumn<procedure_template_asset_models::id>
for ProcedureTemplateAssetModel {
    fn get_column_ref(
        &self,
    ) -> &<procedure_template_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<procedure_template_asset_models::name>
for ProcedureTemplateAssetModel {
    fn get_column_ref(
        &self,
    ) -> &<procedure_template_asset_models::name as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.name
    }
}
impl ::diesel_builders::GetColumn<procedure_template_asset_models::procedure_template_id>
for ProcedureTemplateAssetModel {
    fn get_column_ref(
        &self,
    ) -> &<procedure_template_asset_models::procedure_template_id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.procedure_template_id
    }
}
impl ::diesel_builders::GetColumn<procedure_template_asset_models::based_on_id>
for ProcedureTemplateAssetModel {
    fn get_column_ref(
        &self,
    ) -> &<procedure_template_asset_models::based_on_id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.based_on_id
    }
}
impl ::diesel_builders::GetColumn<procedure_template_asset_models::asset_model_id>
for ProcedureTemplateAssetModel {
    fn get_column_ref(
        &self,
    ) -> &<procedure_template_asset_models::asset_model_id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.asset_model_id
    }
}
/**Aggregated trait ensuring a builder can set all columns for [`ProcedureTemplateAssetModel`].

This trait aggregates [`SetColumn`](::diesel_builders::SetColumn) (or [`TrySetColumn`](::diesel_builders::TrySetColumn)) bounds for every column in the table
(excluding surrogate primary keys).

It is automatically implemented for any builder that satisfies these bounds.*/
pub trait ProcedureTemplateAssetModelTableBuilder: ::diesel_builders::SetColumn<
        procedure_template_asset_models::id,
    > + ::diesel_builders::TrySetColumn<
        procedure_template_asset_models::name,
    > + ::diesel_builders::SetColumn<
        procedure_template_asset_models::procedure_template_id,
    > + ::diesel_builders::SetColumn<
        procedure_template_asset_models::based_on_id,
    > + ::diesel_builders::SetColumn<procedure_template_asset_models::asset_model_id> {}
impl<T> ProcedureTemplateAssetModelTableBuilder for T
where
    T: ::diesel_builders::SetColumn<procedure_template_asset_models::id>
        + ::diesel_builders::TrySetColumn<procedure_template_asset_models::name>
        + ::diesel_builders::SetColumn<
            procedure_template_asset_models::procedure_template_id,
        > + ::diesel_builders::SetColumn<procedure_template_asset_models::based_on_id>
        + ::diesel_builders::SetColumn<procedure_template_asset_models::asset_model_id>,
{}
/**Aggregated trait ensuring access to all columns of [`ProcedureTemplateAssetModel`].

This trait aggregates [`GetColumn`](::diesel_builders::GetColumn) bounds for every column in the table.

It is automatically implemented for any type (struct, generated model, or tuple)
that allows retrieving these columns.*/
pub trait ProcedureTemplateAssetModelTableModel: ::diesel_builders::GetColumn<
        procedure_template_asset_models::id,
    > + ::diesel_builders::GetColumn<
        procedure_template_asset_models::name,
    > + ::diesel_builders::GetColumn<
        procedure_template_asset_models::procedure_template_id,
    > + ::diesel_builders::GetColumn<
        procedure_template_asset_models::based_on_id,
    > + ::diesel_builders::GetColumn<procedure_template_asset_models::asset_model_id> {}
impl<T> ProcedureTemplateAssetModelTableModel for T
where
    T: ::diesel_builders::GetColumn<procedure_template_asset_models::id>
        + ::diesel_builders::GetColumn<procedure_template_asset_models::name>
        + ::diesel_builders::GetColumn<
            procedure_template_asset_models::procedure_template_id,
        > + ::diesel_builders::GetColumn<procedure_template_asset_models::based_on_id>
        + ::diesel_builders::GetColumn<procedure_template_asset_models::asset_model_id>,
{}
impl ::diesel_builders::UniquelyIndexedColumn<
    ::diesel_builders::typenum::U0,
    (procedure_template_asset_models::id,),
> for procedure_template_asset_models::id {}
impl ::diesel_builders::MayGetColumn<procedure_template_asset_models::id>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    fn may_get_column_ref(
        &self,
    ) -> Option<
        &<procedure_template_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType,
    > {
        use ::diesel_builders::tuplities::NestedTupleIndex;
        <Self as NestedTupleIndex<::diesel_builders::typenum::U0>>::nested_index(self)
            .as_ref()
    }
}
impl ::diesel_builders::MayGetColumn<procedure_template_asset_models::name>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    fn may_get_column_ref(
        &self,
    ) -> Option<
        &<procedure_template_asset_models::name as ::diesel_builders::ColumnTyped>::ColumnType,
    > {
        use ::diesel_builders::tuplities::NestedTupleIndex;
        <Self as NestedTupleIndex<::diesel_builders::typenum::U1>>::nested_index(self)
            .as_ref()
    }
}
impl ::diesel_builders::MayGetColumn<
    procedure_template_asset_models::procedure_template_id,
>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    fn may_get_column_ref(
        &self,
    ) -> Option<
        &<procedure_template_asset_models::procedure_template_id as ::diesel_builders::ColumnTyped>::ColumnType,
    > {
        use ::diesel_builders::tuplities::NestedTupleIndex;
        <Self as NestedTupleIndex<::diesel_builders::typenum::U2>>::nested_index(self)
            .as_ref()
    }
}
impl ::diesel_builders::MayGetColumn<procedure_template_asset_models::based_on_id>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    fn may_get_column_ref(
        &self,
    ) -> Option<
        &<procedure_template_asset_models::based_on_id as ::diesel_builders::ColumnTyped>::ColumnType,
    > {
        use ::diesel_builders::tuplities::NestedTupleIndex;
        <Self as NestedTupleIndex<::diesel_builders::typenum::U3>>::nested_index(self)
            .as_ref()
    }
}
impl ::diesel_builders::MayGetColumn<procedure_template_asset_models::asset_model_id>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    fn may_get_column_ref(
        &self,
    ) -> Option<
        &<procedure_template_asset_models::asset_model_id as ::diesel_builders::ColumnTyped>::ColumnType,
    > {
        use ::diesel_builders::tuplities::NestedTupleIndex;
        <Self as NestedTupleIndex<::diesel_builders::typenum::U4>>::nested_index(self)
            .as_ref()
    }
}
impl ::diesel_builders::SetColumn<procedure_template_asset_models::id>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    #[inline]
    fn set_column(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> &mut Self {
        use ::diesel_builders::tuplities::NestedTupleIndexMut;
        *<Self as NestedTupleIndexMut<
            ::diesel_builders::typenum::U0,
        >>::nested_index_mut(self) = Some(value.into());
        self
    }
}
impl ::diesel_builders::SetColumn<procedure_template_asset_models::name>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    #[inline]
    fn set_column(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::name as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> &mut Self {
        use ::diesel_builders::tuplities::NestedTupleIndexMut;
        *<Self as NestedTupleIndexMut<
            ::diesel_builders::typenum::U1,
        >>::nested_index_mut(self) = Some(value.into());
        self
    }
}
impl ::diesel_builders::SetColumn<procedure_template_asset_models::procedure_template_id>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    #[inline]
    fn set_column(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::procedure_template_id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> &mut Self {
        use ::diesel_builders::tuplities::NestedTupleIndexMut;
        *<Self as NestedTupleIndexMut<
            ::diesel_builders::typenum::U2,
        >>::nested_index_mut(self) = Some(value.into());
        self
    }
}
impl ::diesel_builders::SetColumn<procedure_template_asset_models::based_on_id>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    #[inline]
    fn set_column(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::based_on_id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> &mut Self {
        use ::diesel_builders::tuplities::NestedTupleIndexMut;
        *<Self as NestedTupleIndexMut<
            ::diesel_builders::typenum::U3,
        >>::nested_index_mut(self) = Some(value.into());
        self
    }
}
impl ::diesel_builders::SetColumn<procedure_template_asset_models::asset_model_id>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    #[inline]
    fn set_column(
        &mut self,
        value: impl Into<
            <procedure_template_asset_models::asset_model_id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
    ) -> &mut Self {
        use ::diesel_builders::tuplities::NestedTupleIndexMut;
        *<Self as NestedTupleIndexMut<
            ::diesel_builders::typenum::U4,
        >>::nested_index_mut(self) = Some(value.into());
        self
    }
}
impl ::diesel_builders::ValidateColumn<procedure_template_asset_models::id>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    type Error = core::convert::Infallible;
}
impl ::diesel_builders::ValidateColumn<
    procedure_template_asset_models::procedure_template_id,
>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    type Error = core::convert::Infallible;
}
impl ::diesel_builders::ValidateColumn<procedure_template_asset_models::based_on_id>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    type Error = core::convert::Infallible;
}
impl ::diesel_builders::ValidateColumn<procedure_template_asset_models::asset_model_id>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    type Error = core::convert::Infallible;
}
impl ::diesel_builders::Root for procedure_template_asset_models::table {}
impl ::diesel_builders::Descendant for procedure_template_asset_models::table {
    type Ancestors = ();
    type Root = Self;
}
impl diesel_builders::AncestorOfIndex<procedure_template_asset_models::table>
for procedure_template_asset_models::table {
    type Idx = diesel_builders::typenum::U0;
}
impl ::diesel_builders::BundlableTable for procedure_template_asset_models::table {
    type MandatoryTriangularColumns = ();
    type DiscretionaryTriangularColumns = ();
}
impl ::diesel_builders::BuildableTable for procedure_template_asset_models::table {
    type NestedAncestorBuilders = <<procedure_template_asset_models::table as ::diesel_builders::DescendantWithSelf>::NestedAncestorsWithSelf as ::diesel_builders::NestedBundlableTables>::NestedBundleBuilders;
    type NestedCompletedAncestorBuilders = <<procedure_template_asset_models::table as ::diesel_builders::DescendantWithSelf>::NestedAncestorsWithSelf as ::diesel_builders::NestedBundlableTables>::NestedCompletedBundleBuilders;
    fn default_bundles() -> Self::NestedAncestorBuilders {
        #[allow(unused_mut)]
        let mut bundles = <Self::NestedAncestorBuilders as Default>::default();
        let mut builder = ::diesel_builders::TableBuilder::<Self>::from_bundles(bundles);
        builder.into_bundles()
    }
}
impl ::diesel_builders::HorizontalSameAsGroup for procedure_template_asset_models::id {
    type Idx = ::diesel_builders::typenum::U0;
    type MandatoryHorizontalKeys = ();
    type DiscretionaryHorizontalKeys = ();
}
impl ::diesel_builders::HorizontalSameAsGroup for procedure_template_asset_models::name {
    type Idx = ::diesel_builders::typenum::U0;
    type MandatoryHorizontalKeys = ();
    type DiscretionaryHorizontalKeys = ();
}
impl ::diesel_builders::HorizontalSameAsGroup
for procedure_template_asset_models::procedure_template_id {
    type Idx = ::diesel_builders::typenum::U0;
    type MandatoryHorizontalKeys = ();
    type DiscretionaryHorizontalKeys = ();
}
impl ::diesel_builders::HorizontalSameAsGroup
for procedure_template_asset_models::based_on_id {
    type Idx = ::diesel_builders::typenum::U0;
    type MandatoryHorizontalKeys = ();
    type DiscretionaryHorizontalKeys = ();
}
impl ::diesel_builders::HorizontalSameAsGroup
for procedure_template_asset_models::asset_model_id {
    type Idx = ::diesel_builders::typenum::U0;
    type MandatoryHorizontalKeys = ();
    type DiscretionaryHorizontalKeys = ();
}
impl diesel_builders::VerticalSameAsGroup for procedure_template_asset_models::id {
    type VerticalSameAsNestedColumns = ();
}
impl diesel_builders::VerticalSameAsGroup for procedure_template_asset_models::name {
    type VerticalSameAsNestedColumns = ();
}
impl diesel_builders::VerticalSameAsGroup
for procedure_template_asset_models::procedure_template_id {
    type VerticalSameAsNestedColumns = ();
}
impl diesel_builders::VerticalSameAsGroup
for procedure_template_asset_models::based_on_id {
    type VerticalSameAsNestedColumns = ();
}
impl diesel_builders::VerticalSameAsGroup
for procedure_template_asset_models::asset_model_id {
    type VerticalSameAsNestedColumns = ();
}
impl ::diesel::query_source::TableNotEqual<
    ::aps_procedure_templates::procedure_templates::table,
> for procedure_template_asset_models::table {}
impl ::diesel::query_source::TableNotEqual<procedure_template_asset_models::table>
for ::aps_procedure_templates::procedure_templates::table {}
impl ::diesel::query_source::TableNotEqual<::aps_asset_models::asset_models::table>
for procedure_template_asset_models::table {}
impl ::diesel::query_source::TableNotEqual<procedure_template_asset_models::table>
for ::aps_asset_models::asset_models::table {}
impl ::diesel_builders::HostColumn<
    ::diesel_builders::typenum::U0,
    (
        procedure_template_asset_models::based_on_id,
        procedure_template_asset_models::asset_model_id,
    ),
    (
        procedure_template_asset_models::id,
        procedure_template_asset_models::asset_model_id,
    ),
> for procedure_template_asset_models::based_on_id {}
impl ::diesel_builders::HostColumn<
    ::diesel_builders::typenum::U1,
    (
        procedure_template_asset_models::based_on_id,
        procedure_template_asset_models::asset_model_id,
    ),
    (
        procedure_template_asset_models::id,
        procedure_template_asset_models::asset_model_id,
    ),
> for procedure_template_asset_models::asset_model_id {}
impl ::diesel_builders::ForeignPrimaryKey
for procedure_template_asset_models::procedure_template_id {
    type ReferencedTable = ::aps_procedure_templates::procedure_templates::table;
}
///Trait to get the foreign record referenced by `procedure_template_id`.
pub trait FKProcedureTemplateAssetModelsProcedureTemplateId<
    Conn,
>: ::diesel_builders::GetForeign<
        Conn,
        (procedure_template_asset_models::procedure_template_id,),
        (
            <::aps_procedure_templates::procedure_templates::table as ::diesel::Table>::PrimaryKey,
        ),
    > {
    ///Fetches the foreign `procedure_templates` record referenced by this `procedure_template_id`.
    ///
    ///# Arguments
    ///
    ///* `conn` - A mutable reference to the database connection.
    ///
    ///# Errors
    ///Returns a `diesel::QueryResult` error if the query fails or no matching record is found.
    #[inline]
    fn procedure_template(
        &self,
        conn: &mut Conn,
    ) -> ::diesel::QueryResult<
        <::aps_procedure_templates::procedure_templates::table as ::diesel_builders::TableExt>::Model,
    > {
        <Self as ::diesel_builders::GetForeign<
            Conn,
            (procedure_template_asset_models::procedure_template_id,),
            (
                <::aps_procedure_templates::procedure_templates::table as ::diesel::Table>::PrimaryKey,
            ),
        >>::foreign(self, conn)
    }
}
impl<T, Conn> FKProcedureTemplateAssetModelsProcedureTemplateId<Conn> for T
where
    T: ::diesel_builders::GetForeign<
        Conn,
        (procedure_template_asset_models::procedure_template_id,),
        (
            <::aps_procedure_templates::procedure_templates::table as ::diesel::Table>::PrimaryKey,
        ),
    >,
{}
impl ::diesel_builders::ForeignPrimaryKey
for procedure_template_asset_models::based_on_id {
    type ReferencedTable = procedure_template_asset_models::table;
}
///Trait to get the foreign record referenced by `based_on_id`.
pub trait FKProcedureTemplateAssetModelsBasedOnId<
    Conn,
>: ::diesel_builders::GetForeign<
        Conn,
        (procedure_template_asset_models::based_on_id,),
        (<procedure_template_asset_models::table as ::diesel::Table>::PrimaryKey,),
    > {
    ///Fetches the foreign `procedure_template_asset_models` record referenced by this `based_on_id`.
    ///
    ///# Arguments
    ///
    ///* `conn` - A mutable reference to the database connection.
    ///
    ///# Errors
    ///Returns a `diesel::QueryResult` error if the query fails or no matching record is found.
    #[inline]
    fn based_on(
        &self,
        conn: &mut Conn,
    ) -> ::diesel::QueryResult<
        <procedure_template_asset_models::table as ::diesel_builders::TableExt>::Model,
    > {
        <Self as ::diesel_builders::GetForeign<
            Conn,
            (procedure_template_asset_models::based_on_id,),
            (<procedure_template_asset_models::table as ::diesel::Table>::PrimaryKey,),
        >>::foreign(self, conn)
    }
}
impl<T, Conn> FKProcedureTemplateAssetModelsBasedOnId<Conn> for T
where
    T: ::diesel_builders::GetForeign<
        Conn,
        (procedure_template_asset_models::based_on_id,),
        (<procedure_template_asset_models::table as ::diesel::Table>::PrimaryKey,),
    >,
{}
impl ::diesel_builders::ForeignPrimaryKey
for procedure_template_asset_models::asset_model_id {
    type ReferencedTable = ::aps_asset_models::asset_models::table;
}
///Trait to get the foreign record referenced by `asset_model_id`.
pub trait FKProcedureTemplateAssetModelsAssetModelId<
    Conn,
>: ::diesel_builders::GetForeign<
        Conn,
        (procedure_template_asset_models::asset_model_id,),
        (<::aps_asset_models::asset_models::table as ::diesel::Table>::PrimaryKey,),
    > {
    ///Fetches the foreign `asset_models` record referenced by this `asset_model_id`.
    ///
    ///# Arguments
    ///
    ///* `conn` - A mutable reference to the database connection.
    ///
    ///# Errors
    ///Returns a `diesel::QueryResult` error if the query fails or no matching record is found.
    #[inline]
    fn asset_model(
        &self,
        conn: &mut Conn,
    ) -> ::diesel::QueryResult<
        <::aps_asset_models::asset_models::table as ::diesel_builders::TableExt>::Model,
    > {
        <Self as ::diesel_builders::GetForeign<
            Conn,
            (procedure_template_asset_models::asset_model_id,),
            (<::aps_asset_models::asset_models::table as ::diesel::Table>::PrimaryKey,),
        >>::foreign(self, conn)
    }
}
impl<T, Conn> FKProcedureTemplateAssetModelsAssetModelId<Conn> for T
where
    T: ::diesel_builders::GetForeign<
        Conn,
        (procedure_template_asset_models::asset_model_id,),
        (<::aps_asset_models::asset_models::table as ::diesel::Table>::PrimaryKey,),
    >,
{}
impl ::diesel_builders::IterForeignKeys<(procedure_template_asset_models::id,)>
for ProcedureTemplateAssetModel {
    #[inline]
    fn iter_foreign_key_columns() -> impl Iterator<
        Item = <(
            procedure_template_asset_models::id,
        ) as ::diesel_builders::HasNestedDynColumns>::NestedDynColumns,
    > {
        [
            (
                ::diesel_builders::DynColumn::<
                    ::rosetta_uuid::Uuid,
                >::from(procedure_template_asset_models::based_on_id),
            ),
        ]
            .into_iter()
    }
    #[inline]
    fn iter_match_simple<'a>(
        &'a self,
    ) -> impl Iterator<Item = (::std::option::Option<&'a ::rosetta_uuid::Uuid>,)>
    where
        (procedure_template_asset_models::id,): 'a,
    {
        ::std::iter::empty()
            .chain(
                ::std::iter::once((
                    ::diesel_builders::GetColumn::<
                        procedure_template_asset_models::based_on_id,
                    >::get_column_ref(self)
                        .as_ref(),
                )),
            )
    }
}
impl ::diesel_builders::IterForeignKeys<
    (::aps_procedure_templates::procedure_templates::id,),
> for ProcedureTemplateAssetModel {
    #[inline]
    fn iter_foreign_key_columns() -> impl Iterator<
        Item = <(
            ::aps_procedure_templates::procedure_templates::id,
        ) as ::diesel_builders::HasNestedDynColumns>::NestedDynColumns,
    > {
        [
            (
                ::diesel_builders::DynColumn::<
                    ::rosetta_uuid::Uuid,
                >::from(procedure_template_asset_models::procedure_template_id),
            ),
        ]
            .into_iter()
    }
    #[inline]
    fn iter_match_simple<'a>(
        &'a self,
    ) -> impl Iterator<Item = (::std::option::Option<&'a ::rosetta_uuid::Uuid>,)>
    where
        (::aps_procedure_templates::procedure_templates::id,): 'a,
    {
        ::std::iter::empty()
            .chain(
                ::std::iter::once((
                    ::std::option::Option::Some(
                        ::diesel_builders::GetColumn::<
                            procedure_template_asset_models::procedure_template_id,
                        >::get_column_ref(self),
                    ),
                )),
            )
    }
}
impl ::diesel_builders::IterForeignKeys<(::aps_asset_models::asset_models::id,)>
for ProcedureTemplateAssetModel {
    #[inline]
    fn iter_foreign_key_columns() -> impl Iterator<
        Item = <(
            ::aps_asset_models::asset_models::id,
        ) as ::diesel_builders::HasNestedDynColumns>::NestedDynColumns,
    > {
        [
            (
                ::diesel_builders::DynColumn::<
                    ::rosetta_uuid::Uuid,
                >::from(procedure_template_asset_models::asset_model_id),
            ),
        ]
            .into_iter()
    }
    #[inline]
    fn iter_match_simple<'a>(
        &'a self,
    ) -> impl Iterator<Item = (::std::option::Option<&'a ::rosetta_uuid::Uuid>,)>
    where
        (::aps_asset_models::asset_models::id,): 'a,
    {
        ::std::iter::empty()
            .chain(
                ::std::iter::once((
                    ::std::option::Option::Some(
                        ::diesel_builders::GetColumn::<
                            procedure_template_asset_models::asset_model_id,
                        >::get_column_ref(self),
                    ),
                )),
            )
    }
}
impl ::diesel_builders::IterForeignKeys<
    (
        procedure_template_asset_models::id,
        (procedure_template_asset_models::asset_model_id,),
    ),
> for ProcedureTemplateAssetModel {
    #[inline]
    fn iter_foreign_key_columns() -> impl Iterator<
        Item = <(
            procedure_template_asset_models::id,
            (procedure_template_asset_models::asset_model_id,),
        ) as ::diesel_builders::HasNestedDynColumns>::NestedDynColumns,
    > {
        [
            (
                ::diesel_builders::DynColumn::<
                    ::rosetta_uuid::Uuid,
                >::from(procedure_template_asset_models::based_on_id),
                (
                    ::diesel_builders::DynColumn::<
                        ::rosetta_uuid::Uuid,
                    >::from(procedure_template_asset_models::asset_model_id),
                ),
            ),
        ]
            .into_iter()
    }
    #[inline]
    fn iter_match_simple<'a>(
        &'a self,
    ) -> impl Iterator<
        Item = (
            ::std::option::Option<&'a ::rosetta_uuid::Uuid>,
            (::std::option::Option<&'a ::rosetta_uuid::Uuid>,),
        ),
    >
    where
        (
            procedure_template_asset_models::id,
            (procedure_template_asset_models::asset_model_id,),
        ): 'a,
    {
        ::std::iter::empty()
            .chain(
                ::std::iter::once((
                    ::diesel_builders::GetColumn::<
                        procedure_template_asset_models::based_on_id,
                    >::get_column_ref(self)
                        .as_ref(),
                    (
                        ::std::option::Option::Some(
                            ::diesel_builders::GetColumn::<
                                procedure_template_asset_models::asset_model_id,
                            >::get_column_ref(self),
                        ),
                    ),
                )),
            )
    }
}
impl<DynIdx> ::diesel_builders::IterDynForeignKeys<DynIdx>
for ProcedureTemplateAssetModel
where
    DynIdx: ::diesel_builders::NestedDynColumns + ::diesel_builders::TypedNestedTuple
        + 'static,
    DynIdx::NestedTupleValueType: 'static,
{
    fn iter_foreign_key_dyn_columns(index: DynIdx) -> impl Iterator<Item = DynIdx> {
        {
            if ::core::any::TypeId::of::<DynIdx>()
                == ::core::any::TypeId::of::<
                    (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                >()
            {
                let index_any = &index as &dyn ::std::any::Any;
                if let Some(concrete_index) = index_any
                    .downcast_ref::<
                        (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                    >()
                {
                    if <(
                        ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                    ) as ::diesel_builders::NestedDynColumns>::nested_dyn_column_names(
                        concrete_index,
                    )
                        == <(
                            procedure_template_asset_models::id,
                        ) as ::diesel_builders::NestedColumns>::NESTED_COLUMN_NAMES
                        && <(
                            ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                        ) as ::diesel_builders::NestedDynColumns>::nested_dyn_column_table_names(
                            concrete_index,
                        )
                            == <(
                                procedure_template_asset_models::id,
                            ) as ::diesel_builders::NestedColumns>::NESTED_TABLE_NAMES
                    {
                        let iterator = <Self as ::diesel_builders::IterForeignKeys<
                            (procedure_template_asset_models::id,),
                        >>::iter_foreign_key_columns();
                        let dyn_keys: ::std::vec::Vec<
                            (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                        > = iterator.collect();
                        let boxed: ::std::boxed::Box<dyn ::std::any::Any> = ::std::boxed::Box::new(
                            dyn_keys,
                        );
                        if let Ok(vec) = boxed.downcast::<::std::vec::Vec<DynIdx>>() {
                            return vec.into_iter();
                        }
                    }
                }
            }
        }
        {
            if ::core::any::TypeId::of::<DynIdx>()
                == ::core::any::TypeId::of::<
                    (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                >()
            {
                let index_any = &index as &dyn ::std::any::Any;
                if let Some(concrete_index) = index_any
                    .downcast_ref::<
                        (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                    >()
                {
                    if <(
                        ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                    ) as ::diesel_builders::NestedDynColumns>::nested_dyn_column_names(
                        concrete_index,
                    )
                        == <(
                            ::aps_procedure_templates::procedure_templates::id,
                        ) as ::diesel_builders::NestedColumns>::NESTED_COLUMN_NAMES
                        && <(
                            ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                        ) as ::diesel_builders::NestedDynColumns>::nested_dyn_column_table_names(
                            concrete_index,
                        )
                            == <(
                                ::aps_procedure_templates::procedure_templates::id,
                            ) as ::diesel_builders::NestedColumns>::NESTED_TABLE_NAMES
                    {
                        let iterator = <Self as ::diesel_builders::IterForeignKeys<
                            (::aps_procedure_templates::procedure_templates::id,),
                        >>::iter_foreign_key_columns();
                        let dyn_keys: ::std::vec::Vec<
                            (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                        > = iterator.collect();
                        let boxed: ::std::boxed::Box<dyn ::std::any::Any> = ::std::boxed::Box::new(
                            dyn_keys,
                        );
                        if let Ok(vec) = boxed.downcast::<::std::vec::Vec<DynIdx>>() {
                            return vec.into_iter();
                        }
                    }
                }
            }
        }
        {
            if ::core::any::TypeId::of::<DynIdx>()
                == ::core::any::TypeId::of::<
                    (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                >()
            {
                let index_any = &index as &dyn ::std::any::Any;
                if let Some(concrete_index) = index_any
                    .downcast_ref::<
                        (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                    >()
                {
                    if <(
                        ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                    ) as ::diesel_builders::NestedDynColumns>::nested_dyn_column_names(
                        concrete_index,
                    )
                        == <(
                            ::aps_asset_models::asset_models::id,
                        ) as ::diesel_builders::NestedColumns>::NESTED_COLUMN_NAMES
                        && <(
                            ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                        ) as ::diesel_builders::NestedDynColumns>::nested_dyn_column_table_names(
                            concrete_index,
                        )
                            == <(
                                ::aps_asset_models::asset_models::id,
                            ) as ::diesel_builders::NestedColumns>::NESTED_TABLE_NAMES
                    {
                        let iterator = <Self as ::diesel_builders::IterForeignKeys<
                            (::aps_asset_models::asset_models::id,),
                        >>::iter_foreign_key_columns();
                        let dyn_keys: ::std::vec::Vec<
                            (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                        > = iterator.collect();
                        let boxed: ::std::boxed::Box<dyn ::std::any::Any> = ::std::boxed::Box::new(
                            dyn_keys,
                        );
                        if let Ok(vec) = boxed.downcast::<::std::vec::Vec<DynIdx>>() {
                            return vec.into_iter();
                        }
                    }
                }
            }
        }
        {
            if ::core::any::TypeId::of::<DynIdx>()
                == ::core::any::TypeId::of::<
                    (
                        ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                        (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                    ),
                >()
            {
                let index_any = &index as &dyn ::std::any::Any;
                if let Some(concrete_index) = index_any
                    .downcast_ref::<
                        (
                            ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                            (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                        ),
                    >()
                {
                    if <(
                        ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                        (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                    ) as ::diesel_builders::NestedDynColumns>::nested_dyn_column_names(
                        concrete_index,
                    )
                        == <(
                            procedure_template_asset_models::id,
                            (procedure_template_asset_models::asset_model_id,),
                        ) as ::diesel_builders::NestedColumns>::NESTED_COLUMN_NAMES
                        && <(
                            ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                            (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                        ) as ::diesel_builders::NestedDynColumns>::nested_dyn_column_table_names(
                            concrete_index,
                        )
                            == <(
                                procedure_template_asset_models::id,
                                (procedure_template_asset_models::asset_model_id,),
                            ) as ::diesel_builders::NestedColumns>::NESTED_TABLE_NAMES
                    {
                        let iterator = <Self as ::diesel_builders::IterForeignKeys<
                            (
                                procedure_template_asset_models::id,
                                (procedure_template_asset_models::asset_model_id,),
                            ),
                        >>::iter_foreign_key_columns();
                        let dyn_keys: ::std::vec::Vec<
                            (
                                ::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,
                                (::diesel_builders::DynColumn<::rosetta_uuid::Uuid>,),
                            ),
                        > = iterator.collect();
                        let boxed: ::std::boxed::Box<dyn ::std::any::Any> = ::std::boxed::Box::new(
                            dyn_keys,
                        );
                        if let Ok(vec) = boxed.downcast::<::std::vec::Vec<DynIdx>>() {
                            return vec.into_iter();
                        }
                    }
                }
            }
        }
        ::std::vec::Vec::new().into_iter()
    }
}
impl ::diesel_builders::TableExt for procedure_template_asset_models::table {
    const TABLE_NAME: &'static str = "procedure_template_asset_models";
    type NewRecord = (
        procedure_template_asset_models::id,
        (
            procedure_template_asset_models::name,
            (
                procedure_template_asset_models::procedure_template_id,
                (
                    procedure_template_asset_models::based_on_id,
                    (procedure_template_asset_models::asset_model_id,),
                ),
            ),
        ),
    );
    type NewValues = (
        Option<
            <procedure_template_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType,
        >,
        (
            Option<
                <procedure_template_asset_models::name as ::diesel_builders::ColumnTyped>::ColumnType,
            >,
            (
                Option<
                    <procedure_template_asset_models::procedure_template_id as ::diesel_builders::ColumnTyped>::ColumnType,
                >,
                (
                    Option<
                        <procedure_template_asset_models::based_on_id as ::diesel_builders::ColumnTyped>::ColumnType,
                    >,
                    (
                        Option<
                            <procedure_template_asset_models::asset_model_id as ::diesel_builders::ColumnTyped>::ColumnType,
                        >,
                    ),
                ),
            ),
        ),
    );
    type Model = ProcedureTemplateAssetModel;
    type NestedPrimaryKeyColumns = (procedure_template_asset_models::id,);
    type Error = ::validation_errors::ValidationError;
    fn default_new_values() -> Self::NewValues {
        (
            Some((::rosetta_uuid::Uuid::new_v4()).to_owned().into()),
            (None, (None, (Some(None), (None,)))),
        )
    }
}
impl diesel_builders::UniquelyIndexedColumn<
    diesel_builders::typenum::U0,
    (
        procedure_template_asset_models::procedure_template_id,
        procedure_template_asset_models::name,
    ),
> for procedure_template_asset_models::procedure_template_id {}
impl diesel_builders::UniquelyIndexedColumn<
    diesel_builders::typenum::U1,
    (
        procedure_template_asset_models::procedure_template_id,
        procedure_template_asset_models::name,
    ),
> for procedure_template_asset_models::name {}
impl diesel_builders::UniquelyIndexedColumn<
    diesel_builders::typenum::U0,
    (
        procedure_template_asset_models::id,
        procedure_template_asset_models::procedure_template_id,
    ),
> for procedure_template_asset_models::id {}
impl diesel_builders::UniquelyIndexedColumn<
    diesel_builders::typenum::U1,
    (
        procedure_template_asset_models::id,
        procedure_template_asset_models::procedure_template_id,
    ),
> for procedure_template_asset_models::procedure_template_id {}
impl diesel_builders::UniquelyIndexedColumn<
    diesel_builders::typenum::U0,
    (
        procedure_template_asset_models::id,
        procedure_template_asset_models::asset_model_id,
    ),
> for procedure_template_asset_models::id {}
impl diesel_builders::UniquelyIndexedColumn<
    diesel_builders::typenum::U1,
    (
        procedure_template_asset_models::id,
        procedure_template_asset_models::asset_model_id,
    ),
> for procedure_template_asset_models::asset_model_id {}
impl ::diesel_builders::ValidateColumn<procedure_template_asset_models::name>
for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues {
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(
                ::validation_errors::ValidationError::empty(
                    "procedure_template_asset_models",
                    crate::procedure_template_asset_models::name::NAME,
                ),
            );
        }
        Ok(())
    }
}
