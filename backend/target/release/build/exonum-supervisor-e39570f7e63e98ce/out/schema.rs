// This file is generated by rust-protobuf 2.28.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `schema.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ConfigProposalWithHash {
    // message fields
    pub propose_hash: ::protobuf::SingularPtrField<super::types::Hash>,
    pub config_propose: ::protobuf::SingularPtrField<super::service::ConfigPropose>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ConfigProposalWithHash {
    fn default() -> &'a ConfigProposalWithHash {
        <ConfigProposalWithHash as ::protobuf::Message>::default_instance()
    }
}

impl ConfigProposalWithHash {
    pub fn new() -> ConfigProposalWithHash {
        ::std::default::Default::default()
    }

    // .exonum.crypto.Hash propose_hash = 1;


    pub fn get_propose_hash(&self) -> &super::types::Hash {
        self.propose_hash.as_ref().unwrap_or_else(|| <super::types::Hash as ::protobuf::Message>::default_instance())
    }
    pub fn clear_propose_hash(&mut self) {
        self.propose_hash.clear();
    }

    pub fn has_propose_hash(&self) -> bool {
        self.propose_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_propose_hash(&mut self, v: super::types::Hash) {
        self.propose_hash = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_propose_hash(&mut self) -> &mut super::types::Hash {
        if self.propose_hash.is_none() {
            self.propose_hash.set_default();
        }
        self.propose_hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_propose_hash(&mut self) -> super::types::Hash {
        self.propose_hash.take().unwrap_or_else(|| super::types::Hash::new())
    }

    // .exonum.supervisor.ConfigPropose config_propose = 2;


    pub fn get_config_propose(&self) -> &super::service::ConfigPropose {
        self.config_propose.as_ref().unwrap_or_else(|| <super::service::ConfigPropose as ::protobuf::Message>::default_instance())
    }
    pub fn clear_config_propose(&mut self) {
        self.config_propose.clear();
    }

    pub fn has_config_propose(&self) -> bool {
        self.config_propose.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config_propose(&mut self, v: super::service::ConfigPropose) {
        self.config_propose = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config_propose(&mut self) -> &mut super::service::ConfigPropose {
        if self.config_propose.is_none() {
            self.config_propose.set_default();
        }
        self.config_propose.as_mut().unwrap()
    }

    // Take field
    pub fn take_config_propose(&mut self) -> super::service::ConfigPropose {
        self.config_propose.take().unwrap_or_else(|| super::service::ConfigPropose::new())
    }
}

impl ::protobuf::Message for ConfigProposalWithHash {
    fn is_initialized(&self) -> bool {
        for v in &self.propose_hash {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.config_propose {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.propose_hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.config_propose)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.propose_hash.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.config_propose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.propose_hash.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.config_propose.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ConfigProposalWithHash {
        ConfigProposalWithHash::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::Hash>>(
                "propose_hash",
                |m: &ConfigProposalWithHash| { &m.propose_hash },
                |m: &mut ConfigProposalWithHash| { &mut m.propose_hash },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::service::ConfigPropose>>(
                "config_propose",
                |m: &ConfigProposalWithHash| { &m.config_propose },
                |m: &mut ConfigProposalWithHash| { &mut m.config_propose },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ConfigProposalWithHash>(
                "ConfigProposalWithHash",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ConfigProposalWithHash {
        static instance: ::protobuf::rt::LazyV2<ConfigProposalWithHash> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ConfigProposalWithHash::new)
    }
}

impl ::protobuf::Clear for ConfigProposalWithHash {
    fn clear(&mut self) {
        self.propose_hash.clear();
        self.config_propose.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfigProposalWithHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfigProposalWithHash {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ErrorInfo {
    // message fields
    pub height: u64,
    pub error: ::protobuf::SingularPtrField<super::errors::ExecutionError>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ErrorInfo {
    fn default() -> &'a ErrorInfo {
        <ErrorInfo as ::protobuf::Message>::default_instance()
    }
}

impl ErrorInfo {
    pub fn new() -> ErrorInfo {
        ::std::default::Default::default()
    }

    // uint64 height = 1;


    pub fn get_height(&self) -> u64 {
        self.height
    }
    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    // .exonum.runtime.ExecutionError error = 2;


    pub fn get_error(&self) -> &super::errors::ExecutionError {
        self.error.as_ref().unwrap_or_else(|| <super::errors::ExecutionError as ::protobuf::Message>::default_instance())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: super::errors::ExecutionError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut super::errors::ExecutionError {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> super::errors::ExecutionError {
        self.error.take().unwrap_or_else(|| super::errors::ExecutionError::new())
    }
}

impl ::protobuf::Message for ErrorInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.error {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_uint64(1, self.height)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ErrorInfo {
        ErrorInfo::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "height",
                |m: &ErrorInfo| { &m.height },
                |m: &mut ErrorInfo| { &mut m.height },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errors::ExecutionError>>(
                "error",
                |m: &ErrorInfo| { &m.error },
                |m: &mut ErrorInfo| { &mut m.error },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ErrorInfo>(
                "ErrorInfo",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ErrorInfo {
        static instance: ::protobuf::rt::LazyV2<ErrorInfo> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ErrorInfo::new)
    }
}

impl ::protobuf::Clear for ErrorInfo {
    fn clear(&mut self) {
        self.height = 0;
        self.error.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ErrorInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct AsyncEventState {
    // message fields
    pub state: AsyncEventState_Type,
    pub error: ::protobuf::SingularPtrField<ErrorInfo>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a AsyncEventState {
    fn default() -> &'a AsyncEventState {
        <AsyncEventState as ::protobuf::Message>::default_instance()
    }
}

impl AsyncEventState {
    pub fn new() -> AsyncEventState {
        ::std::default::Default::default()
    }

    // .exonum.supervisor.AsyncEventState.Type state = 1;


    pub fn get_state(&self) -> AsyncEventState_Type {
        self.state
    }
    pub fn clear_state(&mut self) {
        self.state = AsyncEventState_Type::PENDING;
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: AsyncEventState_Type) {
        self.state = v;
    }

    // .exonum.supervisor.ErrorInfo error = 2;


    pub fn get_error(&self) -> &ErrorInfo {
        self.error.as_ref().unwrap_or_else(|| <ErrorInfo as ::protobuf::Message>::default_instance())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ErrorInfo) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ErrorInfo {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ErrorInfo {
        self.error.take().unwrap_or_else(|| ErrorInfo::new())
    }
}

impl ::protobuf::Message for AsyncEventState {
    fn is_initialized(&self) -> bool {
        for v in &self.error {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.state, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.state != AsyncEventState_Type::PENDING {
            my_size += ::protobuf::rt::enum_size(1, self.state);
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.state != AsyncEventState_Type::PENDING {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.state))?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> AsyncEventState {
        AsyncEventState::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AsyncEventState_Type>>(
                "state",
                |m: &AsyncEventState| { &m.state },
                |m: &mut AsyncEventState| { &mut m.state },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ErrorInfo>>(
                "error",
                |m: &AsyncEventState| { &m.error },
                |m: &mut AsyncEventState| { &mut m.error },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<AsyncEventState>(
                "AsyncEventState",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static AsyncEventState {
        static instance: ::protobuf::rt::LazyV2<AsyncEventState> = ::protobuf::rt::LazyV2::INIT;
        instance.get(AsyncEventState::new)
    }
}

impl ::protobuf::Clear for AsyncEventState {
    fn clear(&mut self) {
        self.state = AsyncEventState_Type::PENDING;
        self.error.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AsyncEventState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AsyncEventState {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum AsyncEventState_Type {
    PENDING = 0,
    FAIL = 1,
    TIMEOUT = 2,
    SUCCESS = 3,
}

impl ::protobuf::ProtobufEnum for AsyncEventState_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AsyncEventState_Type> {
        match value {
            0 => ::std::option::Option::Some(AsyncEventState_Type::PENDING),
            1 => ::std::option::Option::Some(AsyncEventState_Type::FAIL),
            2 => ::std::option::Option::Some(AsyncEventState_Type::TIMEOUT),
            3 => ::std::option::Option::Some(AsyncEventState_Type::SUCCESS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AsyncEventState_Type] = &[
            AsyncEventState_Type::PENDING,
            AsyncEventState_Type::FAIL,
            AsyncEventState_Type::TIMEOUT,
            AsyncEventState_Type::SUCCESS,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<AsyncEventState_Type>("AsyncEventState.Type", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for AsyncEventState_Type {
}

impl ::std::default::Default for AsyncEventState_Type {
    fn default() -> Self {
        AsyncEventState_Type::PENDING
    }
}

impl ::protobuf::reflect::ProtobufValue for AsyncEventState_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct MigrationState {
    // message fields
    pub inner: ::protobuf::SingularPtrField<AsyncEventState>,
    pub version: ::std::string::String,
    pub reference_state_hash: ::protobuf::SingularPtrField<super::types::Hash>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MigrationState {
    fn default() -> &'a MigrationState {
        <MigrationState as ::protobuf::Message>::default_instance()
    }
}

impl MigrationState {
    pub fn new() -> MigrationState {
        ::std::default::Default::default()
    }

    // .exonum.supervisor.AsyncEventState inner = 1;


    pub fn get_inner(&self) -> &AsyncEventState {
        self.inner.as_ref().unwrap_or_else(|| <AsyncEventState as ::protobuf::Message>::default_instance())
    }
    pub fn clear_inner(&mut self) {
        self.inner.clear();
    }

    pub fn has_inner(&self) -> bool {
        self.inner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inner(&mut self, v: AsyncEventState) {
        self.inner = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inner(&mut self) -> &mut AsyncEventState {
        if self.inner.is_none() {
            self.inner.set_default();
        }
        self.inner.as_mut().unwrap()
    }

    // Take field
    pub fn take_inner(&mut self) -> AsyncEventState {
        self.inner.take().unwrap_or_else(|| AsyncEventState::new())
    }

    // string version = 2;


    pub fn get_version(&self) -> &str {
        &self.version
    }
    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    // .exonum.crypto.Hash reference_state_hash = 3;


    pub fn get_reference_state_hash(&self) -> &super::types::Hash {
        self.reference_state_hash.as_ref().unwrap_or_else(|| <super::types::Hash as ::protobuf::Message>::default_instance())
    }
    pub fn clear_reference_state_hash(&mut self) {
        self.reference_state_hash.clear();
    }

    pub fn has_reference_state_hash(&self) -> bool {
        self.reference_state_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reference_state_hash(&mut self, v: super::types::Hash) {
        self.reference_state_hash = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reference_state_hash(&mut self) -> &mut super::types::Hash {
        if self.reference_state_hash.is_none() {
            self.reference_state_hash.set_default();
        }
        self.reference_state_hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_reference_state_hash(&mut self) -> super::types::Hash {
        self.reference_state_hash.take().unwrap_or_else(|| super::types::Hash::new())
    }
}

impl ::protobuf::Message for MigrationState {
    fn is_initialized(&self) -> bool {
        for v in &self.inner {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.reference_state_hash {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.inner)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reference_state_hash)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.inner.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.version);
        }
        if let Some(ref v) = self.reference_state_hash.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.inner.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.version.is_empty() {
            os.write_string(2, &self.version)?;
        }
        if let Some(ref v) = self.reference_state_hash.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> MigrationState {
        MigrationState::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AsyncEventState>>(
                "inner",
                |m: &MigrationState| { &m.inner },
                |m: &mut MigrationState| { &mut m.inner },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "version",
                |m: &MigrationState| { &m.version },
                |m: &mut MigrationState| { &mut m.version },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::Hash>>(
                "reference_state_hash",
                |m: &MigrationState| { &m.reference_state_hash },
                |m: &mut MigrationState| { &mut m.reference_state_hash },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MigrationState>(
                "MigrationState",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MigrationState {
        static instance: ::protobuf::rt::LazyV2<MigrationState> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MigrationState::new)
    }
}

impl ::protobuf::Clear for MigrationState {
    fn clear(&mut self) {
        self.inner.clear();
        self.version.clear();
        self.reference_state_hash.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MigrationState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MigrationState {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cschema.proto\x12\x11exonum.supervisor\x1a\x19exonum/crypto/types.p\
    roto\x1a\x1bexonum/runtime/errors.proto\x1a\rservice.proto\"\x99\x01\n\
    \x16ConfigProposalWithHash\x126\n\x0cpropose_hash\x18\x01\x20\x01(\x0b2\
    \x13.exonum.crypto.HashR\x0bproposeHash\x12G\n\x0econfig_propose\x18\x02\
    \x20\x01(\x0b2\x20.exonum.supervisor.ConfigProposeR\rconfigPropose\"Y\n\
    \tErrorInfo\x12\x16\n\x06height\x18\x01\x20\x01(\x04R\x06height\x124\n\
    \x05error\x18\x02\x20\x01(\x0b2\x1e.exonum.runtime.ExecutionErrorR\x05er\
    ror\"\xbd\x01\n\x0fAsyncEventState\x12=\n\x05state\x18\x01\x20\x01(\x0e2\
    '.exonum.supervisor.AsyncEventState.TypeR\x05state\x122\n\x05error\x18\
    \x02\x20\x01(\x0b2\x1c.exonum.supervisor.ErrorInfoR\x05error\"7\n\x04Typ\
    e\x12\x0b\n\x07PENDING\x10\0\x12\x08\n\x04FAIL\x10\x01\x12\x0b\n\x07TIME\
    OUT\x10\x02\x12\x0b\n\x07SUCCESS\x10\x03\"\xab\x01\n\x0eMigrationState\
    \x128\n\x05inner\x18\x01\x20\x01(\x0b2\".exonum.supervisor.AsyncEventSta\
    teR\x05inner\x12\x18\n\x07version\x18\x02\x20\x01(\tR\x07version\x12E\n\
    \x14reference_state_hash\x18\x03\x20\x01(\x0b2\x13.exonum.crypto.HashR\
    \x12referenceStateHashb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
