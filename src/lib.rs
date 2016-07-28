extern crate libc;
use libc::{c_char, c_short, pid_t, uid_t, timeval, passwd};
#[cfg(target_os = "linux")]
use libc::c_int;

pub const EMPTY: c_short = 0;
pub const RUN_LVL: c_short = 1;
pub const BOOT_TIME: c_short = 2;
pub const OLD_TIME: c_short = 3;
pub const NEW_TIME: c_short = 4;
pub const INIT_PROCESS: c_short = 5;
pub const LOGIN_PROCESS: c_short = 6;
pub const USER_PROCESS: c_short = 7;
pub const DEAD_PROCESS: c_short = 8;

#[cfg(target_os = "linux")]
#[repr(C)]
pub struct utmpx {
  pub ut_type: c_short,
  pub ut_pid: pid_t,
  pub ut_line: [c_char; 32],
  pub ut_id: [c_char; 4],
  pub ut_user: [c_char; 32],
  pub ut_host: [c_char; 256],
  pub ut_exit: exit_status,
  pub ut_tv: timeval,
  pub ut_session: c_int,
}

#[cfg(target_os = "macos")]
#[repr(C)]
pub struct utmpx {
  pub ut_user: [c_char; 256],
  pub ut_id: [c_char; 4],
  pub ut_line: [c_char; 32],
  pub ut_pid: pid_t,
  pub ut_type: c_short,
  pub ut_tv: timeval,
  pub ut_host: [c_char; 256],
}

#[repr(C)]
pub struct exit_status {
  pub e_termination: c_short,
  pub e_exit: c_short
}

extern {
  pub fn getutxent() -> *const utmpx;

  pub fn getpwuid(uid: uid_t) -> *const passwd;
}
