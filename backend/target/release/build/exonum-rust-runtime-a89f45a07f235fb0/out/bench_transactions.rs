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
//! Generated file from `bench_transactions.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct CurrencyTx {
    // message fields
    pub to: ::protobuf::SingularPtrField<super::types::PublicKey>,
    pub seed: u32,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CurrencyTx {
    fn default() -> &'a CurrencyTx {
        <CurrencyTx as ::protobuf::Message>::default_instance()
    }
}

impl CurrencyTx {
    pub fn new() -> CurrencyTx {
        ::std::default::Default::default()
    }

    // .exonum.crypto.PublicKey to = 1;


    pub fn get_to(&self) -> &super::types::PublicKey {
        self.to.as_ref().unwrap_or_else(|| <super::types::PublicKey as ::protobuf::Message>::default_instance())
    }
    pub fn clear_to(&mut self) {
        self.to.clear();
    }

    pub fn has_to(&self) -> bool {
        self.to.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to(&mut self, v: super::types::PublicKey) {
        self.to = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to(&mut self) -> &mut super::types::PublicKey {
        if self.to.is_none() {
            self.to.set_default();
        }
        self.to.as_mut().unwrap()
    }

    // Take field
    pub fn take_to(&mut self) -> super::types::PublicKey {
        self.to.take().unwrap_or_else(|| super::types::PublicKey::new())
    }

    // uint32 seed = 2;


    pub fn get_seed(&self) -> u32 {
        self.seed
    }
    pub fn clear_seed(&mut self) {
        self.seed = 0;
    }

    // Param is passed by value, moved
    pub fn set_seed(&mut self, v: u32) {
        self.seed = v;
    }
}

impl ::protobuf::Message for CurrencyTx {
    fn is_initialized(&self) -> bool {
        for v in &self.to {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.to)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seed = tmp;
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
        if let Some(ref v) = self.to.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.seed != 0 {
            my_size += ::protobuf::rt::value_size(2, self.seed, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.to.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.seed != 0 {
            os.write_uint32(2, self.seed)?;
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

    fn new() -> CurrencyTx {
        CurrencyTx::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::PublicKey>>(
                "to",
                |m: &CurrencyTx| { &m.to },
                |m: &mut CurrencyTx| { &mut m.to },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "seed",
                |m: &CurrencyTx| { &m.seed },
                |m: &mut CurrencyTx| { &mut m.seed },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CurrencyTx>(
                "CurrencyTx",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CurrencyTx {
        static instance: ::protobuf::rt::LazyV2<CurrencyTx> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CurrencyTx::new)
    }
}

impl ::protobuf::Clear for CurrencyTx {
    fn clear(&mut self) {
        self.to.clear();
        self.seed = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CurrencyTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CurrencyTx {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18bench_transactions.proto\x12\x11exonum.benchmarks\x1a\x19exonum/cr\
    ypto/types.proto\"J\n\nCurrencyTx\x12(\n\x02to\x18\x01\x20\x01(\x0b2\x18\
    .exonum.crypto.PublicKeyR\x02to\x12\x12\n\x04seed\x18\x02\x20\x01(\rR\
    \x04seedb\x06proto3\
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
