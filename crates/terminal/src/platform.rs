pub mod current {
    #[cfg(target_os = "macos")]
    pub use mac::*;
}

struct ProcessInfo {
    name: String,
    cwd: PathBuf,
}

pub fn process_info(pid: i32) -> Option<ProcessInfo> {
    ProcessInfo {
        cwd: current::current_working_dir(pid)?,
        name: current::current_working_dir(pid)?,
    }
}

#[cfg(target_os = "macos")]
mod mac_os {
    pub fn current_working_dir(pid: u32) -> Option<PathBuf> {
        let mut pathinfo: libc::proc_vnodepathinfo = unsafe { std::mem::zeroed() };
        let size = std::mem::size_of_val(&pathinfo) as libc::c_int;
        let ret = unsafe {
            libc::proc_pidinfo(
                pid as _,
                libc::PROC_PIDVNODEPATHINFO,
                0,
                &mut pathinfo as *mut _ as *mut _,
                size,
            )
        };
        if ret != size {
            return None;
        }

        // Workaround a workaround for an old rustc version supported by libc;
        // the type of vip_path should just be [c_char; MAXPATHLEN] but it
        // is defined as a horrible nested array by the libc crate:
        // `[[c_char; 32]; 32]`.
        // Urgh.  Let's re-cast it as the correct kind of slice.
        let vip_path = unsafe {
            std::slice::from_raw_parts(
                pathinfo.pvi_cdir.vip_path.as_ptr() as *const u8,
                libc::MAXPATHLEN as usize,
            )
        };
        let nul = vip_path.iter().position(|&c| c == 0)?;
        Some(OsStr::from_bytes(&vip_path[0..nul]).into())
    }

    pub fn process_name(pid: u32) -> Option<PathBuf> {
        let mut pathinfo: libc::proc_vnodepathinfo = unsafe { std::mem::zeroed() };
        let size = std::mem::size_of_val(&pathinfo) as libc::c_int;
        let ret = unsafe {
            libc::proc_pidinfo(
                pid as _,
                libc::PROC_PIDVNODEPATHINFO,
                0,
                &mut pathinfo as *mut _ as *mut _,
                size,
            )
        };
        if ret != size {
            return None;
        }

        // Workaround a workaround for an old rustc version supported by libc;
        // the type of vip_path should just be [c_char; MAXPATHLEN] but it
        // is defined as a horrible nested array by the libc crate:
        // `[[c_char; 32]; 32]`.
        // Urgh.  Let's re-cast it as the correct kind of slice.
        let vip_path = unsafe {
            std::slice::from_raw_parts(
                pathinfo.pvi_cdir.vip_path.as_ptr() as *const u8,
                libc::MAXPATHLEN as usize,
            )
        };
        let nul = vip_path.iter().position(|&c| c == 0)?;
        Some(OsStr::from_bytes(&vip_path[0..nul]).into())
    }
}
