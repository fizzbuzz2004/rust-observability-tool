#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::{bpf_get_current_comm, bpf_get_current_pid_tgid},
    macros::{kprobe, map},
    maps::PerfEventArray,
    programs::ProbeContext,
};
use my_observability_tool_common::ExecEvent;

#[map]
static EVENTS: PerfEventArray<ExecEvent> = PerfEventArray::new(0);

#[kprobe]
pub fn execve(ctx: ProbeContext) -> u32 {
    match try_execve(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[inline(always)]
fn try_execve(ctx: ProbeContext) -> Result<u32, u32> {
    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as u32;

    let comm = match bpf_get_current_comm() {
        Ok(c) => c,
        Err(_) => [0u8; 16],
    };

    let event = ExecEvent {
        pid,
        ppid: 0,
        comm,
    };

    EVENTS.output(&ctx, &event, 0);

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
