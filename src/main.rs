use std::env;
use nix;
fn main() {
    let args : Vec<String> = env::args().collect();

    if (args.len() <2) {
        println!("Usage : {} <pid>",args[0]);
        return;
    }
    let pid_i32 : i32 = args[1].parse().unwrap();
    let attach_pid = nix::sys::ptrace::attach(nix::unistd::Pid::from_raw(pid_i32));
    attach_pid.unwrap(); // handle result trước

    nix::sys::wait::waitpid(
        nix::unistd::Pid::from_raw(pid_i32),
        None
    ).unwrap();

    let regs = nix::sys::ptrace::getregs(
        nix::unistd::Pid::from_raw(pid_i32)
    ).unwrap();

    println!("RIP: 0x{:x}", regs.rip);
    println!("RSP: 0x{:x}", regs.rsp);
    println!("RAX: 0x{:x}", regs.rax);
}
