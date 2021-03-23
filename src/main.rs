/*
 * @Author: dotwoo
 * @Date: 2021-03-23 10:28:53
 * @LastEditTime: 2021-03-23 16:01:10
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /rust_unit_app/src/main.rs
 */

use gflags;
use rust_unit_app::atoi;
use rust_unit_app::intptr_t;
use rust_unit_app::memset;
use rust_unit_app::nxt_unit_buf_send;
use rust_unit_app::nxt_unit_buf_t;
use rust_unit_app::nxt_unit_callbacks_t;
use rust_unit_app::nxt_unit_ctx_alloc;
use rust_unit_app::nxt_unit_ctx_t;
use rust_unit_app::nxt_unit_done;
use rust_unit_app::nxt_unit_field_t;
use rust_unit_app::nxt_unit_free;
use rust_unit_app::nxt_unit_init;
use rust_unit_app::nxt_unit_init_t;
use rust_unit_app::nxt_unit_is_main_ctx;
use rust_unit_app::nxt_unit_log;
use rust_unit_app::nxt_unit_malloc;
use rust_unit_app::nxt_unit_port_id_t;
use rust_unit_app::nxt_unit_port_t;
use rust_unit_app::nxt_unit_request_done;
use rust_unit_app::nxt_unit_request_info_t;
use rust_unit_app::nxt_unit_request_read;
use rust_unit_app::nxt_unit_request_t;
use rust_unit_app::nxt_unit_response_add_content;
use rust_unit_app::nxt_unit_response_add_field;
use rust_unit_app::nxt_unit_response_buf_alloc;
use rust_unit_app::nxt_unit_response_init;
use rust_unit_app::nxt_unit_response_send;
use rust_unit_app::nxt_unit_run;
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
unsafe fn uxt_main(threads: i32) {
    let mut ctx: *mut nxt_unit_ctx_t = 0 as *mut nxt_unit_ctx_t;
    let mut init: nxt_unit_init_t = unsafe { mem::zeroed() };
    init.callbacks.request_handler = Some(greeting_app_request_handler);

    ctx = nxt_unit_init(&mut init);
    if ctx.is_null() {
        std::process::exit(1);
    }
    nxt_unit_run(ctx);

    nxt_unit_done(ctx);
}

unsafe extern "C" fn greeting_app_request_handler(mut req: *mut nxt_unit_request_info_t) {
    let mut rc: libc::c_int = 0;
    rc = nxt_unit_response_init(
        req,
        200 as uint16_t,
        1 as uint32_t,
        (CONTENTTYPE.len() + TEXTPLAIN.len() + RETURNCONTENT.len()) as uint32_t,
    );
    rc = nxt_unit_response_add_field(
        req,
        CONTENTTYPE.as_ptr() as *const libc::c_char,
        CONTENTTYPE.len() as uint8_t,
        TEXTPLAIN.as_ptr() as *const libc::c_char,
        TEXTPLAIN.len() as uint32_t,
    );
    rc = nxt_unit_response_add_content(
        req,
        RETURNCONTENT.as_ptr() as *const libc::c_void,
        RETURNCONTENT.len() as uint32_t,
    );
    rc = nxt_unit_response_send(req);
    nxt_unit_request_done(req, rc);
}
