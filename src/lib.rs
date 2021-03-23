/*
 * @Author: dotwoo
 * @Date: 2021-03-23 15:14:16
 * @LastEditTime: 2021-03-23 15:17:20
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /rust_unit_app/src/lib.rs
 */


#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,  register_tool)]
#[macro_use]
extern crate c2rust_bitfields;

extern "C" {
    pub type nxt_unit_response_s;
    pub type nxt_unit_websocket_frame_s;
    #[no_mangle]
    pub fn nxt_unit_log(
        ctx: *mut nxt_unit_ctx_t,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    pub fn nxt_unit_free(ctx: *mut nxt_unit_ctx_t, p: *mut libc::c_void);
    #[no_mangle]
    pub fn nxt_unit_malloc(ctx: *mut nxt_unit_ctx_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    pub fn nxt_unit_request_done(req: *mut nxt_unit_request_info_t, rc: libc::c_int);
    #[no_mangle]
    pub fn nxt_unit_request_read(
        req: *mut nxt_unit_request_info_t,
        dst: *mut libc::c_void,
        size: size_t,
    ) -> ssize_t;
    #[no_mangle]
    pub fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    #[no_mangle]
    pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn nxt_unit_buf_send(buf: *mut nxt_unit_buf_t) -> libc::c_int;
    #[no_mangle]
    pub fn nxt_unit_init(_: *mut nxt_unit_init_t) -> *mut nxt_unit_ctx_t;
    #[no_mangle]
    pub fn nxt_unit_run(_: *mut nxt_unit_ctx_t) -> libc::c_int;
    #[no_mangle]
    pub fn nxt_unit_is_main_ctx(ctx: *mut nxt_unit_ctx_t) -> libc::c_int;
    #[no_mangle]
    pub fn nxt_unit_done(_: *mut nxt_unit_ctx_t);
    #[no_mangle]
    pub fn nxt_unit_ctx_alloc(_: *mut nxt_unit_ctx_t, _: *mut libc::c_void) -> *mut nxt_unit_ctx_t;
    #[no_mangle]
    pub fn nxt_unit_response_init(
        req: *mut nxt_unit_request_info_t,
        status: uint16_t,
        max_fields_count: uint32_t,
        max_fields_size: uint32_t,
    ) -> libc::c_int;
    #[no_mangle]
    pub fn nxt_unit_response_add_field(
        req: *mut nxt_unit_request_info_t,
        name: *const libc::c_char,
        name_length: uint8_t,
        value: *const libc::c_char,
        value_length: uint32_t,
    ) -> libc::c_int;
    #[no_mangle]
    pub fn nxt_unit_response_add_content(
        req: *mut nxt_unit_request_info_t,
        src: *const libc::c_void,
        size: uint32_t,
    ) -> libc::c_int;
    #[no_mangle]
    pub fn nxt_unit_response_send(req: *mut nxt_unit_request_info_t) -> libc::c_int;
    #[no_mangle]
    pub fn nxt_unit_response_buf_alloc(
        req: *mut nxt_unit_request_info_t,
        size: uint32_t,
    ) -> *mut nxt_unit_buf_t;
    #[no_mangle]
    pub fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    pub fn pthread_join(_: pthread_t, _: *mut *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    pub fn atoi(_: *const libc::c_char) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __darwin_intptr_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_pid_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub __arg: *mut libc::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_attr_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: libc::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [libc::c_char; 8176],
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type intptr_t = __darwin_intptr_t;
pub type pid_t = __darwin_pid_t;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
pub type pthread_attr_t = __darwin_pthread_attr_t;
pub type pthread_t = __darwin_pthread_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nxt_unit_s {
    pub data: *mut libc::c_void,
}
pub type nxt_unit_t = nxt_unit_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nxt_unit_ctx_s {
    pub data: *mut libc::c_void,
    pub unit: *mut nxt_unit_t,
}
pub type nxt_unit_ctx_t = nxt_unit_ctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nxt_unit_port_id_s {
    pub pid: pid_t,
    pub hash: uint32_t,
    pub id: uint16_t,
}
pub type nxt_unit_port_id_t = nxt_unit_port_id_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nxt_unit_port_s {
    pub id: nxt_unit_port_id_t,
    pub in_fd: libc::c_int,
    pub out_fd: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type nxt_unit_port_t = nxt_unit_port_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nxt_unit_buf_s {
    pub start: *mut libc::c_char,
    pub free: *mut libc::c_char,
    pub end: *mut libc::c_char,
}
pub type nxt_unit_buf_t = nxt_unit_buf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nxt_unit_request_info_s {
    pub unit: *mut nxt_unit_t,
    pub ctx: *mut nxt_unit_ctx_t,
    pub response_port: *mut nxt_unit_port_t,
    pub request: *mut nxt_unit_request_t,
    pub request_buf: *mut nxt_unit_buf_t,
    pub response: *mut nxt_unit_response_t,
    pub response_buf: *mut nxt_unit_buf_t,
    pub response_max_fields: uint32_t,
    pub content_buf: *mut nxt_unit_buf_t,
    pub content_length: uint64_t,
    pub content_fd: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type nxt_unit_response_t = nxt_unit_response_s;
pub type nxt_unit_request_t = nxt_unit_request_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nxt_unit_request_s {
    pub method_length: uint8_t,
    pub version_length: uint8_t,
    pub remote_length: uint8_t,
    pub local_length: uint8_t,
    pub tls: uint8_t,
    pub websocket_handshake: uint8_t,
    pub app_target: uint8_t,
    pub server_name_length: uint32_t,
    pub target_length: uint32_t,
    pub path_length: uint32_t,
    pub query_length: uint32_t,
    pub fields_count: uint32_t,
    pub content_length_field: uint32_t,
    pub content_type_field: uint32_t,
    pub cookie_field: uint32_t,
    pub authorization_field: uint32_t,
    pub content_length: uint64_t,
    pub method: nxt_unit_sptr_t,
    pub version: nxt_unit_sptr_t,
    pub remote: nxt_unit_sptr_t,
    pub local: nxt_unit_sptr_t,
    pub server_name: nxt_unit_sptr_t,
    pub target: nxt_unit_sptr_t,
    pub path: nxt_unit_sptr_t,
    pub query: nxt_unit_sptr_t,
    pub preread_content: nxt_unit_sptr_t,
    pub fields: [nxt_unit_field_t; 0],
}
pub type nxt_unit_field_t = nxt_unit_field_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct nxt_unit_field_s {
    pub hash: uint16_t,
    #[bitfield(name = "skip", ty = "uint8_t", bits = "0..=0")]
    #[bitfield(name = "hopbyhop", ty = "uint8_t", bits = "1..=1")]
    pub skip_hopbyhop: [u8; 1],
    pub name_length: uint8_t,
    pub value_length: uint32_t,
    pub name: nxt_unit_sptr_t,
    pub value: nxt_unit_sptr_t,
}
pub type nxt_unit_sptr_t = nxt_unit_sptr_u;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nxt_unit_sptr_u {
    pub base: [uint8_t; 1],
    pub offset: uint32_t,
}
pub type nxt_unit_request_info_t = nxt_unit_request_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nxt_unit_callbacks_s {
    pub request_handler: Option<unsafe extern "C" fn(_: *mut nxt_unit_request_info_t) -> ()>,
    pub data_handler: Option<unsafe extern "C" fn(_: *mut nxt_unit_request_info_t) -> ()>,
    pub websocket_handler: Option<unsafe extern "C" fn(_: *mut nxt_unit_websocket_frame_t) -> ()>,
    pub close_handler: Option<unsafe extern "C" fn(_: *mut nxt_unit_request_info_t) -> ()>,
    pub add_port: Option<
        unsafe extern "C" fn(_: *mut nxt_unit_ctx_t, _: *mut nxt_unit_port_t) -> libc::c_int,
    >,
    pub remove_port:
        Option<unsafe extern "C" fn(_: *mut nxt_unit_t, _: *mut nxt_unit_port_t) -> ()>,
    pub remove_pid: Option<unsafe extern "C" fn(_: *mut nxt_unit_t, _: pid_t) -> ()>,
    pub quit: Option<unsafe extern "C" fn(_: *mut nxt_unit_ctx_t) -> ()>,
    pub shm_ack_handler: Option<unsafe extern "C" fn(_: *mut nxt_unit_ctx_t) -> ()>,
    pub port_send: Option<
        unsafe extern "C" fn(
            _: *mut nxt_unit_ctx_t,
            _: *mut nxt_unit_port_t,
            _: *const libc::c_void,
            _: size_t,
            _: *const libc::c_void,
            _: size_t,
        ) -> ssize_t,
    >,
    pub port_recv: Option<
        unsafe extern "C" fn(
            _: *mut nxt_unit_ctx_t,
            _: *mut nxt_unit_port_t,
            _: *mut libc::c_void,
            _: size_t,
            _: *mut libc::c_void,
            _: size_t,
        ) -> ssize_t,
    >,
    pub ready_handler: Option<unsafe extern "C" fn(_: *mut nxt_unit_ctx_t) -> libc::c_int>,
}
pub type nxt_unit_websocket_frame_t = nxt_unit_websocket_frame_s;
pub type nxt_unit_callbacks_t = nxt_unit_callbacks_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nxt_unit_init_s {
    pub data: *mut libc::c_void,
    pub ctx_data: *mut libc::c_void,
    pub max_pending_requests: libc::c_int,
    pub request_data_size: uint32_t,
    pub shm_limit: uint32_t,
    pub callbacks: nxt_unit_callbacks_t,
    pub ready_port: nxt_unit_port_t,
    pub ready_stream: uint32_t,
    pub router_port: nxt_unit_port_t,
    pub read_port: nxt_unit_port_t,
    pub log_fd: libc::c_int,
}
pub type nxt_unit_init_t = nxt_unit_init_s;
pub type C2RustUnnamed = libc::c_uint;
pub const NXT_UNIT_CANCELLED: C2RustUnnamed = 3;
pub const NXT_UNIT_AGAIN: C2RustUnnamed = 2;
pub const NXT_UNIT_ERROR: C2RustUnnamed = 1;
pub const NXT_UNIT_OK: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NXT_UNIT_LOG_DEBUG: C2RustUnnamed_0 = 5;
pub const NXT_UNIT_LOG_INFO: C2RustUnnamed_0 = 4;
pub const NXT_UNIT_LOG_NOTICE: C2RustUnnamed_0 = 3;
pub const NXT_UNIT_LOG_WARN: C2RustUnnamed_0 = 2;
pub const NXT_UNIT_LOG_ERR: C2RustUnnamed_0 = 1;
pub const NXT_UNIT_LOG_ALERT: C2RustUnnamed_0 = 0;


#[inline]
pub unsafe extern "C"  fn copy(
    mut p: *mut libc::c_char,
    mut src: *const libc::c_void,
    mut len: uint32_t,
) -> *mut libc::c_char {
    memcpy(p as *mut libc::c_void, src, len as libc::c_ulong);
    return p.offset(len as isize);
}

#[inline]
pub unsafe extern "C"  fn nxt_unit_sptr_get(mut sptr: *mut nxt_unit_sptr_t) -> *mut libc::c_void {
    return (*sptr).base.as_mut_ptr().offset((*sptr).offset as isize) as *mut libc::c_void;
}