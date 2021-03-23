/*
 * @Author: dotwoo
 * @Date: 2021-03-23 10:28:53
 * @LastEditTime: 2021-03-23 17:07:07
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /rust_unit_app/src/main.rs
 */

#![feature(core_intrinsics)]

use gflags;
use rust_unit_app::nxt_unit_buf_send;
use rust_unit_app::nxt_unit_buf_t;
use rust_unit_app::nxt_unit_done;
use rust_unit_app::nxt_unit_init;
use rust_unit_app::nxt_unit_init_t;
use rust_unit_app::nxt_unit_request_done;
use rust_unit_app::nxt_unit_request_info_t;
use rust_unit_app::nxt_unit_request_read;
use rust_unit_app::nxt_unit_response_add_content;
use rust_unit_app::nxt_unit_response_add_field;
use rust_unit_app::nxt_unit_response_buf_alloc;
use rust_unit_app::nxt_unit_response_init;
use rust_unit_app::nxt_unit_response_send;
use rust_unit_app::nxt_unit_run;
use rust_unit_app::size_t;
use rust_unit_app::uint16_t;
use rust_unit_app::uint32_t;
use rust_unit_app::uint8_t;
use std::mem;

gflags::define! {
    /// the threads num setting
    -t, --threads <THREADS> :i32 = 1
}

gflags::define! {
    -h, --help = false
}

static RETURNCONTENT: &str = "Rust: Hello world!\n";
static CONTENTTYPE: &str = "Content-Type";
static TEXTPLAIN: &str = "text/plain";

fn main() {
    gflags::parse();

    if HELP.flag {
        gflags::print_help_and_exit(0);
    }
    unsafe { uxt_main(THREADS.flag) }
    return;
}
unsafe fn uxt_main(_threads: i32) {
    let mut init: nxt_unit_init_t = mem::zeroed();
    init.callbacks.request_handler = Some(greeting_app_request_handler);

    let ctx = nxt_unit_init(&mut init);
    if ctx.is_null() {
        std::process::exit(1);
    }
    nxt_unit_run(ctx);

    nxt_unit_done(ctx);
}

unsafe extern "C" fn greeting_app_request_handler(req: *mut nxt_unit_request_info_t) {
    let mut rc = nxt_unit_response_init(
        req,
        200 as uint16_t,
        1 as uint32_t,
        (CONTENTTYPE.len() + TEXTPLAIN.len() + RETURNCONTENT.len()) as uint32_t,
    );

    if std::intrinsics::unlikely(rc != rust_unit_app::NXT_UNIT_OK as i32) {
        nxt_unit_request_done(req, rc);
        return;
    }

    rc = nxt_unit_response_add_field(
        req,
        CONTENTTYPE.as_ptr() as *const libc::c_char,
        CONTENTTYPE.len() as uint8_t,
        TEXTPLAIN.as_ptr() as *const libc::c_char,
        TEXTPLAIN.len() as uint32_t,
    );

    if std::intrinsics::unlikely(rc != rust_unit_app::NXT_UNIT_OK as i32) {
        nxt_unit_request_done(req, rc);
        return;
    }

    rc = nxt_unit_response_add_content(
        req,
        RETURNCONTENT.as_ptr() as *const libc::c_void,
        RETURNCONTENT.len() as uint32_t,
    );

    if std::intrinsics::unlikely(rc != rust_unit_app::NXT_UNIT_OK as i32) {
        nxt_unit_request_done(req, rc);
        return;
    }

    rc = nxt_unit_response_send(req);

    if std::intrinsics::unlikely(rc != rust_unit_app::NXT_UNIT_OK as i32) {
        nxt_unit_request_done(req, rc);
        return;
    }

    let buf = nxt_unit_response_buf_alloc(
        req,
        (*(*req).request_buf)
            .end
            .offset_from((*(*req).request_buf).start) as u32,
    );

    if std::intrinsics::unlikely(buf == 0 as *mut libc::c_void as *mut nxt_unit_buf_t) {
        nxt_unit_request_done(req, rc);
        return;
    }

    let res = nxt_unit_request_read(
        req,
        (*buf).free as *mut libc::c_void,
        (*buf).end.offset_from((*buf).free) as libc::c_long as size_t,
    );
    (*buf).free = (*buf).free.offset(res as isize);

    rc = nxt_unit_buf_send(buf);
    nxt_unit_request_done(req, rc);
}
