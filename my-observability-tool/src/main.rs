use aya::maps::AsyncPerfEventArray;
use aya::programs::KProbe;
use aya::util::online_cpus;
use aya::{include_bytes_aligned, Bpf};
use bytes::BytesMut;
use my_observability_tool_common::ExecEvent;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // 1. eBPF Bytecode einbinden
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/my-observability-tool"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/my-observability-tool"
    ))?;

    // 2. Kprobe laden und an den execve-Systemaufruf anhängen
    let program: &mut KProbe = bpf.program_mut("execve").unwrap().try_into()?;
    program.load()?;
    program.attach("__x64_sys_execve", 0)?;

    println!(">>> eBPF Observability Tool gestartet! Überwache Prozess-Executions...");

    // 3. Perf Ring Buffer auslesen
    let mut events = AsyncPerfEventArray::try_from(bpf.take_map("EVENTS").unwrap())?;

    for cpu_id in online_cpus()? {
        let mut buf = events.open(cpu_id, None)?;

        tokio::spawn(async move {
            let mut buffers = vec![BytesMut::with_capacity(1024); 10];
            loop {
                if let Ok(events) = buf.read_events(&mut buffers).await {
                    for i in 0..events.read {
                        let buf = &buffers[i];
                        if buf.len() >= std::mem::size_of::<ExecEvent>() {
                            let ptr = buf.as_ptr() as *const ExecEvent;
                            let data = unsafe { *ptr };

                            let comm_str = std::str::from_utf8(&data.comm)
                                .unwrap_or("unknown")
                                .trim_matches('\0');

                            println!(
                                "[EXEC LOG] CPU {:<2} | PID: {:<6} | Befehl: {}",
                                cpu_id, data.pid, comm_str
                            );
                        }
                    }
                }
            }
        });
    }

    println!("Drücke Strg+C zum Beenden...");
    signal::ctrl_c().await?;
    println!("\nBeende eBPF Observability Tool...");

    Ok(())
}
