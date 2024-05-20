pub fn find_pid_by_name(name: &str) -> Option<u32> {
    let mut system = sysinfo::System::new();
    system.refresh_all();

    let ps = system.processes_by_name(name);
    for p in ps {
        let pid = p.pid().as_u32();
        return Some(pid);
    }

    None
}
