use crate::{DpsProfile, NpcProp, OptionalReload, RearmMinion, SecZone, Spool};

// #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdSolAddFCtx {
    pub(in crate::cmd) sec_zone: Option<SecZone> = None,
    pub(in crate::cmd) default_incoming_dps: Option<DpsProfile> = None,
    pub(in crate::cmd) default_spool: Option<Spool> = None,
    pub(in crate::cmd) default_npc_prop: Option<NpcProp> = None,
    pub(in crate::cmd) default_optional_reloads: Option<OptionalReload> = None,
    pub(in crate::cmd) default_rearm_minions: Option<RearmMinion> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSolAddFCtx {
    pub(in crate::cmd) fn execute(self, core_src: &rc::Src) -> rc::SolarSystem {
        let mut core_sol = rc::SolarSystem::new(core_src);
        if let Some(sec_zone) = self.sec_zone {
            core_sol.set_sec_zone(sec_zone);
        }
        if let Some(incoming_dps) = self.default_incoming_dps {
            core_sol.set_default_incoming_dps(incoming_dps);
        }
        if let Some(spool) = self.default_spool {
            core_sol.set_default_spool(spool);
        }
        if let Some(npc_prop) = self.default_npc_prop {
            core_sol.set_default_npc_prop(npc_prop);
        }
        if let Some(optional_reloads) = self.default_optional_reloads {
            core_sol.set_default_optional_reloads(optional_reloads);
        }
        if let Some(rearm_minions) = self.default_rearm_minions {
            core_sol.set_default_rearm_minions(rearm_minions);
        }
        core_sol
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
// DFVTD: moved out for compatibility with default_field_values, replace with derive later
#[cfg(feature = "serde")]
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    _serde::__require_serde_not_serde_core!();
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ICmdSolAddFCtx {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error>
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
                __field5,
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
                    _serde::__private228::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        2u64 => _serde::__private228::Ok(__Field::__field2),
                        3u64 => _serde::__private228::Ok(__Field::__field3),
                        4u64 => _serde::__private228::Ok(__Field::__field4),
                        5u64 => _serde::__private228::Ok(__Field::__field5),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "sec_zone" => _serde::__private228::Ok(__Field::__field0),
                        "default_incoming_dps" => _serde::__private228::Ok(__Field::__field1),
                        "default_spool" => _serde::__private228::Ok(__Field::__field2),
                        "default_npc_prop" => _serde::__private228::Ok(__Field::__field3),
                        "default_optional_reloads" => _serde::__private228::Ok(__Field::__field4),
                        "default_rearm_minions" => _serde::__private228::Ok(__Field::__field5),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"sec_zone" => _serde::__private228::Ok(__Field::__field0),
                        b"default_incoming_dps" => _serde::__private228::Ok(__Field::__field1),
                        b"default_spool" => _serde::__private228::Ok(__Field::__field2),
                        b"default_npc_prop" => _serde::__private228::Ok(__Field::__field3),
                        b"default_optional_reloads" => _serde::__private228::Ok(__Field::__field4),
                        b"default_rearm_minions" => _serde::__private228::Ok(__Field::__field5),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<ICmdSolAddFCtx>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ICmdSolAddFCtx;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__formatter, "struct ICmdSolAddFCtx")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<Option<SecZone>>(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct ICmdSolAddFCtx with 6 elements",
                            ));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<Option<DpsProfile>>(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct ICmdSolAddFCtx with 6 elements",
                            ));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<Option<Spool>>(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct ICmdSolAddFCtx with 6 elements",
                            ));
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<Option<NpcProp>>(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(_serde::de::Error::invalid_length(
                                3usize,
                                &"struct ICmdSolAddFCtx with 6 elements",
                            ));
                        }
                    };
                    let __field4 = match _serde::de::SeqAccess::next_element::<Option<OptionalReload>>(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(_serde::de::Error::invalid_length(
                                4usize,
                                &"struct ICmdSolAddFCtx with 6 elements",
                            ));
                        }
                    };
                    let __field5 = match _serde::de::SeqAccess::next_element::<Option<RearmMinion>>(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(_serde::de::Error::invalid_length(
                                5usize,
                                &"struct ICmdSolAddFCtx with 6 elements",
                            ));
                        }
                    };
                    _serde::__private228::Ok(ICmdSolAddFCtx {
                        sec_zone: __field0,
                        default_incoming_dps: __field1,
                        default_spool: __field2,
                        default_npc_prop: __field3,
                        default_optional_reloads: __field4,
                        default_rearm_minions: __field5,
                    })
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<Option<SecZone>> = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<Option<DpsProfile>> = _serde::__private228::None;
                    let mut __field2: _serde::__private228::Option<Option<Spool>> = _serde::__private228::None;
                    let mut __field3: _serde::__private228::Option<Option<NpcProp>> = _serde::__private228::None;
                    let mut __field4: _serde::__private228::Option<Option<OptionalReload>> = _serde::__private228::None;
                    let mut __field5: _serde::__private228::Option<Option<RearmMinion>> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("sec_zone"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(_serde::de::MapAccess::next_value::<
                                    Option<SecZone>,
                                >(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("default_incoming_dps"),
                                    );
                                }
                                __field1 = _serde::__private228::Some(_serde::de::MapAccess::next_value::<
                                    Option<DpsProfile>,
                                >(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private228::Option::is_some(&__field2) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("default_spool"),
                                    );
                                }
                                __field2 = _serde::__private228::Some(_serde::de::MapAccess::next_value::<
                                    Option<Spool>,
                                >(&mut __map)?);
                            }
                            __Field::__field3 => {
                                if _serde::__private228::Option::is_some(&__field3) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("default_npc_prop"),
                                    );
                                }
                                __field3 = _serde::__private228::Some(_serde::de::MapAccess::next_value::<
                                    Option<NpcProp>,
                                >(&mut __map)?);
                            }
                            __Field::__field4 => {
                                if _serde::__private228::Option::is_some(&__field4) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("default_optional_reloads"),
                                    );
                                }
                                __field4 = _serde::__private228::Some(_serde::de::MapAccess::next_value::<
                                    Option<OptionalReload>,
                                >(&mut __map)?);
                            }
                            __Field::__field5 => {
                                if _serde::__private228::Option::is_some(&__field5) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("default_rearm_minions"),
                                    );
                                }
                                __field5 = _serde::__private228::Some(_serde::de::MapAccess::next_value::<
                                    Option<RearmMinion>,
                                >(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => _serde::__private228::de::missing_field("sec_zone")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => _serde::__private228::de::missing_field("default_incoming_dps")?,
                    };
                    let __field2 = match __field2 {
                        _serde::__private228::Some(__field2) => __field2,
                        _serde::__private228::None => _serde::__private228::de::missing_field("default_spool")?,
                    };
                    let __field3 = match __field3 {
                        _serde::__private228::Some(__field3) => __field3,
                        _serde::__private228::None => _serde::__private228::de::missing_field("default_npc_prop")?,
                    };
                    let __field4 = match __field4 {
                        _serde::__private228::Some(__field4) => __field4,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("default_optional_reloads")?
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::__private228::Some(__field5) => __field5,
                        _serde::__private228::None => _serde::__private228::de::missing_field("default_rearm_minions")?,
                    };
                    _serde::__private228::Ok(ICmdSolAddFCtx {
                        sec_zone: __field0,
                        default_incoming_dps: __field1,
                        default_spool: __field2,
                        default_npc_prop: __field3,
                        default_optional_reloads: __field4,
                        default_rearm_minions: __field5,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "sec_zone",
                "default_incoming_dps",
                "default_spool",
                "default_npc_prop",
                "default_optional_reloads",
                "default_rearm_minions",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ICmdSolAddFCtx",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<ICmdSolAddFCtx>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
