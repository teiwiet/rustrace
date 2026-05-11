use std::env;
use nix;
use std;
fn main(){
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage : {} <pid>",args[0]);
        return;
    }
    let pid = nix::unistd::Pid::from_raw(args[1].parse().unwrap());
    nix::sys::ptrace::attach(pid).unwrap();

    nix::sys::wait::waitpid(pid,None).unwrap();
    let regs = nix::sys::ptrace::getregs(pid).unwrap();

    println!("RIP : 0x{:x}",regs.rip);
    println!("RSP : 0x{:x}",regs.rsp);
    println!("RBP : 0x{:x}",regs.rbp);


    println!("RAX : 0x{:x}",regs.rax);
    println!("RBX : 0x{:x}",regs.rbx);
    println!("RCX : 0x{:x}",regs.rcx);
    println!("RDX : 0x{:x}",regs.rdx);


    println!("RDI : 0x{:x}",regs.rdi);
    println!("RSI : 0x{:x}",regs.rsi);

    let rip_addr = regs.rip as *mut std::ffi::c_void;
    let rip_value = nix::sys::ptrace::read(pid,rip_addr).unwrap();

    println!("Word at RIP : 0x{:x}",rip_value);

    let patched = rip_value & !(0xFF) | 0xCC;

    unsafe {
        nix::sys::ptrace::write(pid,rip_addr,patched as i64).unwrap();
    }

    let veritfy = nix::sys::ptrace::read(pid,rip_addr).unwrap();
    println!("Rip before : {:x}",veritfy);
    println!("Is first byte 0xCC ? : {}",(veritfy & 0xFF) == 0xCC);
}

