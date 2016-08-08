extern crate mosh_detached;
#[macro_use]
extern crate error_chain;
extern crate libc;

mod error;
use error::*;

use mosh_detached::*;
use std::ffi::CStr;
use libc::geteuid;
use std::io::Write;

macro_rules! println_stderr {
  ($fmt:expr) => { { writeln!(std::io::stderr(), $fmt).expect("error writing to stderr"); } };
  ($fmt:expr, $($arg:tt)*) => { { writeln!(std::io::stderr(), $fmt, $($arg)*).expect("error writing to stderr"); } };
}

fn get_username() -> Result<String> {
  let pw = unsafe { getpwuid(geteuid()) };
  if pw.is_null() {
    return Err("getpwuid".into());
  }
  let pw = match unsafe { pw.as_ref() } {
    Some(x) => x,
    None => return Err("could not get passwd".into())
  };
  match unsafe { CStr::from_ptr(pw.pw_name) }.to_str() {
    Ok(x) => Ok(x.to_owned()),
    Err(e) => Err(format!("could not get username from passwd: {}", e).into())
  }
}

fn get_unattached_servers() -> Result<Vec<String>> {
  let username = try!(get_username());
  let mut unattached_servers: Vec<String> = Vec::new();
  loop {
    let utm = unsafe { getutxent() };
    if utm.is_null() {
      break;
    }
    let utm = match unsafe { utm.as_ref() } {
      Some(x) => x,
      None => return Err("could not get utmpx entity".into())
    };
    if utm.ut_type != USER_PROCESS {
      continue;
    }
    let user = utm.ut_user;
    let user = match unsafe { CStr::from_ptr(user.as_ptr()) }.to_str() {
      Ok(x) => x,
      Err(e) => return Err(format!("could not get user: {}", e).into())
    };
    if user != username {
      continue;
    }
    let text = match unsafe { CStr::from_ptr(utm.ut_host.as_ptr()) }.to_str() {
      Ok(x) => x,
      Err(e) => return Err(format!("could not get text: {}", e).into())
    };
    if text.len() < 5 || !text.starts_with("mosh ") || !text.ends_with(']') {
      continue;
    }
    unattached_servers.push(text.to_owned());
  }
  Ok(unattached_servers)
}

fn get_pids(servers: Vec<String>) -> Vec<isize> {
  servers
    .iter()
    .flat_map(|x| x.split('['))
    .filter(|x| x.len() > 1)
    .map(|x| &x[0..x.len() - 1])
    .map(|x| x.parse::<isize>())
    .flat_map(|x| x)
    .collect()
}

fn inner() -> i32 {
  let unattached_servers = match get_unattached_servers() {
    Ok(x) => x,
    Err(e) => {
      println_stderr!("could not get unattached servers: {}", e);
      return 1;
    }
  };
  for unattached_server in get_pids(unattached_servers) {
    println!("{}", unattached_server);
  }
  0
}

fn main() {
  let exit_code = inner();
  std::process::exit(exit_code);
}
