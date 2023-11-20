#![no_std]
#![no_main]

use aya_bpf::{
    macros::btf_tracepoint,
    programs::BtfTracePointContext,
};
use aya_log_ebpf::info;

#[btf_tracepoint(function="block_rq_insert")]
pub fn block_rq_insert(ctx: BtfTracePointContext) -> i32 {
    match try_block_rq_insert(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_block_rq_insert(ctx: BtfTracePointContext) -> Result<i32, i32> {
    info!(&ctx, "tracepoint block_rq_insert called");
    unsafe {
        let ptr: usize = ctx.arg(0);
        info!(&ctx, "rq insert {}", ptr);
    }
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
