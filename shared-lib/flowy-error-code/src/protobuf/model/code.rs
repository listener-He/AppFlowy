// This file is generated by rust-protobuf 2.25.2. Do not edit
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
//! Generated file from `code.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorCode {
    Internal = 0,
    UserUnauthorized = 2,
    RecordNotFound = 3,
    UserIdIsEmpty = 4,
    WorkspaceNameInvalid = 100,
    WorkspaceIdInvalid = 101,
    AppColorStyleInvalid = 102,
    WorkspaceDescTooLong = 103,
    WorkspaceNameTooLong = 104,
    AppIdInvalid = 110,
    AppNameInvalid = 111,
    ViewNameInvalid = 120,
    ViewThumbnailInvalid = 121,
    ViewIdInvalid = 122,
    ViewDescTooLong = 123,
    ViewDataInvalid = 124,
    ViewNameTooLong = 125,
    ConnectError = 200,
    EmailIsEmpty = 300,
    EmailFormatInvalid = 301,
    EmailAlreadyExists = 302,
    PasswordIsEmpty = 303,
    PasswordTooLong = 304,
    PasswordContainsForbidCharacters = 305,
    PasswordFormatInvalid = 306,
    PasswordNotMatch = 307,
    UserNameTooLong = 308,
    UserNameContainForbiddenCharacters = 309,
    UserNameIsEmpty = 310,
    UserIdInvalid = 311,
    UserNotExist = 312,
    TextTooLong = 400,
    GridIdIsEmpty = 410,
    BlockIdIsEmpty = 420,
    RowIdIsEmpty = 430,
    OptionIdIsEmpty = 431,
    FieldIdIsEmpty = 440,
    FieldDoesNotExist = 441,
    SelectOptionNameIsEmpty = 442,
    FieldNotExists = 443,
    FieldInvalidOperation = 444,
    TypeOptionDataIsEmpty = 450,
    InvalidDateTimeFormat = 500,
    InvalidData = 1000,
}

impl ::protobuf::ProtobufEnum for ErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorCode> {
        match value {
            0 => ::std::option::Option::Some(ErrorCode::Internal),
            2 => ::std::option::Option::Some(ErrorCode::UserUnauthorized),
            3 => ::std::option::Option::Some(ErrorCode::RecordNotFound),
            4 => ::std::option::Option::Some(ErrorCode::UserIdIsEmpty),
            100 => ::std::option::Option::Some(ErrorCode::WorkspaceNameInvalid),
            101 => ::std::option::Option::Some(ErrorCode::WorkspaceIdInvalid),
            102 => ::std::option::Option::Some(ErrorCode::AppColorStyleInvalid),
            103 => ::std::option::Option::Some(ErrorCode::WorkspaceDescTooLong),
            104 => ::std::option::Option::Some(ErrorCode::WorkspaceNameTooLong),
            110 => ::std::option::Option::Some(ErrorCode::AppIdInvalid),
            111 => ::std::option::Option::Some(ErrorCode::AppNameInvalid),
            120 => ::std::option::Option::Some(ErrorCode::ViewNameInvalid),
            121 => ::std::option::Option::Some(ErrorCode::ViewThumbnailInvalid),
            122 => ::std::option::Option::Some(ErrorCode::ViewIdInvalid),
            123 => ::std::option::Option::Some(ErrorCode::ViewDescTooLong),
            124 => ::std::option::Option::Some(ErrorCode::ViewDataInvalid),
            125 => ::std::option::Option::Some(ErrorCode::ViewNameTooLong),
            200 => ::std::option::Option::Some(ErrorCode::ConnectError),
            300 => ::std::option::Option::Some(ErrorCode::EmailIsEmpty),
            301 => ::std::option::Option::Some(ErrorCode::EmailFormatInvalid),
            302 => ::std::option::Option::Some(ErrorCode::EmailAlreadyExists),
            303 => ::std::option::Option::Some(ErrorCode::PasswordIsEmpty),
            304 => ::std::option::Option::Some(ErrorCode::PasswordTooLong),
            305 => ::std::option::Option::Some(ErrorCode::PasswordContainsForbidCharacters),
            306 => ::std::option::Option::Some(ErrorCode::PasswordFormatInvalid),
            307 => ::std::option::Option::Some(ErrorCode::PasswordNotMatch),
            308 => ::std::option::Option::Some(ErrorCode::UserNameTooLong),
            309 => ::std::option::Option::Some(ErrorCode::UserNameContainForbiddenCharacters),
            310 => ::std::option::Option::Some(ErrorCode::UserNameIsEmpty),
            311 => ::std::option::Option::Some(ErrorCode::UserIdInvalid),
            312 => ::std::option::Option::Some(ErrorCode::UserNotExist),
            400 => ::std::option::Option::Some(ErrorCode::TextTooLong),
            410 => ::std::option::Option::Some(ErrorCode::GridIdIsEmpty),
            420 => ::std::option::Option::Some(ErrorCode::BlockIdIsEmpty),
            430 => ::std::option::Option::Some(ErrorCode::RowIdIsEmpty),
            431 => ::std::option::Option::Some(ErrorCode::OptionIdIsEmpty),
            440 => ::std::option::Option::Some(ErrorCode::FieldIdIsEmpty),
            441 => ::std::option::Option::Some(ErrorCode::FieldDoesNotExist),
            442 => ::std::option::Option::Some(ErrorCode::SelectOptionNameIsEmpty),
            443 => ::std::option::Option::Some(ErrorCode::FieldNotExists),
            444 => ::std::option::Option::Some(ErrorCode::FieldInvalidOperation),
            450 => ::std::option::Option::Some(ErrorCode::TypeOptionDataIsEmpty),
            500 => ::std::option::Option::Some(ErrorCode::InvalidDateTimeFormat),
            1000 => ::std::option::Option::Some(ErrorCode::InvalidData),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorCode] = &[
            ErrorCode::Internal,
            ErrorCode::UserUnauthorized,
            ErrorCode::RecordNotFound,
            ErrorCode::UserIdIsEmpty,
            ErrorCode::WorkspaceNameInvalid,
            ErrorCode::WorkspaceIdInvalid,
            ErrorCode::AppColorStyleInvalid,
            ErrorCode::WorkspaceDescTooLong,
            ErrorCode::WorkspaceNameTooLong,
            ErrorCode::AppIdInvalid,
            ErrorCode::AppNameInvalid,
            ErrorCode::ViewNameInvalid,
            ErrorCode::ViewThumbnailInvalid,
            ErrorCode::ViewIdInvalid,
            ErrorCode::ViewDescTooLong,
            ErrorCode::ViewDataInvalid,
            ErrorCode::ViewNameTooLong,
            ErrorCode::ConnectError,
            ErrorCode::EmailIsEmpty,
            ErrorCode::EmailFormatInvalid,
            ErrorCode::EmailAlreadyExists,
            ErrorCode::PasswordIsEmpty,
            ErrorCode::PasswordTooLong,
            ErrorCode::PasswordContainsForbidCharacters,
            ErrorCode::PasswordFormatInvalid,
            ErrorCode::PasswordNotMatch,
            ErrorCode::UserNameTooLong,
            ErrorCode::UserNameContainForbiddenCharacters,
            ErrorCode::UserNameIsEmpty,
            ErrorCode::UserIdInvalid,
            ErrorCode::UserNotExist,
            ErrorCode::TextTooLong,
            ErrorCode::GridIdIsEmpty,
            ErrorCode::BlockIdIsEmpty,
            ErrorCode::RowIdIsEmpty,
            ErrorCode::OptionIdIsEmpty,
            ErrorCode::FieldIdIsEmpty,
            ErrorCode::FieldDoesNotExist,
            ErrorCode::SelectOptionNameIsEmpty,
            ErrorCode::FieldNotExists,
            ErrorCode::FieldInvalidOperation,
            ErrorCode::TypeOptionDataIsEmpty,
            ErrorCode::InvalidDateTimeFormat,
            ErrorCode::InvalidData,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ErrorCode>("ErrorCode", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ErrorCode {
}

impl ::std::default::Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::Internal
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\ncode.proto*\x81\x08\n\tErrorCode\x12\x0c\n\x08Internal\x10\0\x12\x14\
    \n\x10UserUnauthorized\x10\x02\x12\x12\n\x0eRecordNotFound\x10\x03\x12\
    \x11\n\rUserIdIsEmpty\x10\x04\x12\x18\n\x14WorkspaceNameInvalid\x10d\x12\
    \x16\n\x12WorkspaceIdInvalid\x10e\x12\x18\n\x14AppColorStyleInvalid\x10f\
    \x12\x18\n\x14WorkspaceDescTooLong\x10g\x12\x18\n\x14WorkspaceNameTooLon\
    g\x10h\x12\x10\n\x0cAppIdInvalid\x10n\x12\x12\n\x0eAppNameInvalid\x10o\
    \x12\x13\n\x0fViewNameInvalid\x10x\x12\x18\n\x14ViewThumbnailInvalid\x10\
    y\x12\x11\n\rViewIdInvalid\x10z\x12\x13\n\x0fViewDescTooLong\x10{\x12\
    \x13\n\x0fViewDataInvalid\x10|\x12\x13\n\x0fViewNameTooLong\x10}\x12\x11\
    \n\x0cConnectError\x10\xc8\x01\x12\x11\n\x0cEmailIsEmpty\x10\xac\x02\x12\
    \x17\n\x12EmailFormatInvalid\x10\xad\x02\x12\x17\n\x12EmailAlreadyExists\
    \x10\xae\x02\x12\x14\n\x0fPasswordIsEmpty\x10\xaf\x02\x12\x14\n\x0fPassw\
    ordTooLong\x10\xb0\x02\x12%\n\x20PasswordContainsForbidCharacters\x10\
    \xb1\x02\x12\x1a\n\x15PasswordFormatInvalid\x10\xb2\x02\x12\x15\n\x10Pas\
    swordNotMatch\x10\xb3\x02\x12\x14\n\x0fUserNameTooLong\x10\xb4\x02\x12'\
    \n\"UserNameContainForbiddenCharacters\x10\xb5\x02\x12\x14\n\x0fUserName\
    IsEmpty\x10\xb6\x02\x12\x12\n\rUserIdInvalid\x10\xb7\x02\x12\x11\n\x0cUs\
    erNotExist\x10\xb8\x02\x12\x10\n\x0bTextTooLong\x10\x90\x03\x12\x12\n\rG\
    ridIdIsEmpty\x10\x9a\x03\x12\x13\n\x0eBlockIdIsEmpty\x10\xa4\x03\x12\x11\
    \n\x0cRowIdIsEmpty\x10\xae\x03\x12\x14\n\x0fOptionIdIsEmpty\x10\xaf\x03\
    \x12\x13\n\x0eFieldIdIsEmpty\x10\xb8\x03\x12\x16\n\x11FieldDoesNotExist\
    \x10\xb9\x03\x12\x1c\n\x17SelectOptionNameIsEmpty\x10\xba\x03\x12\x13\n\
    \x0eFieldNotExists\x10\xbb\x03\x12\x1a\n\x15FieldInvalidOperation\x10\
    \xbc\x03\x12\x1a\n\x15TypeOptionDataIsEmpty\x10\xc2\x03\x12\x1a\n\x15Inv\
    alidDateTimeFormat\x10\xf4\x03\x12\x10\n\x0bInvalidData\x10\xe8\x07b\x06\
    proto3\
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