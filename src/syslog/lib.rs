#[crate_type = "lib"];
#[desc = "Syslog client"];
#[license = "MIT"];

extern crate extra = "extra";

use std::io;
use std::str;
use std::libc::getpid;
use extra::time;


pub type Priority = uint;

pub enum Severity {
  LOG_EMERG,
  LOG_ALERT,
  LOG_CRIT,
  LOG_ERR,
  LOG_WARNING,
  LOG_NOTICE,
  LOG_INFO,
  LOG_DEBUG
}

pub enum Facility {
  LOG_KERN     = 0  << 3,
  LOG_USER     = 1  << 3,
  LOG_MAIL     = 2  << 3,
  LOG_DAEMON   = 3  << 3,
  LOG_AUTH     = 4  << 3,
  LOG_SYSLOG   = 5  << 3,
  LOG_LPR      = 6  << 3,
  LOG_NEWS     = 7  << 3,
  LOG_UUCP     = 8  << 3,
  LOG_CRON     = 9  << 3,
  LOG_AUTHPRIV = 10 << 3,
  LOG_FTP      = 11 << 3,
  LOG_LOCAL0   = 16 << 3,
  LOG_LOCAL1   = 17 << 3,
  LOG_LOCAL2   = 18 << 3,
  LOG_LOCAL3   = 19 << 3,
  LOG_LOCAL4   = 20 << 3,
  LOG_LOCAL5   = 21 << 3,
  LOG_LOCAL6   = 22 << 3,
  LOG_LOCAL7   = 23 << 3
}

pub struct Writer {
  severity: Severity,
  facility: Facility,
  tag:      ~str,
  hostname: ~str,
  network:  ~str,
  raddr:    ~str
}

pub fn init(address: ~str, severity: Severity, facility: Facility, tag: ~str) -> ~Writer {
  ~Writer {
    severity: severity,
    facility: facility,
    tag:      tag,
    hostname: ~"",
    network:  ~"",
    raddr:    address
  }
}

impl Writer {
  pub fn format(&self, message: ~str) -> ~str {
    let pid = unsafe { getpid() };
    let f =  format!("<{:u}> {:d} {:s} {:s} {:s} {:d} {:s}",
      self.encode_priority(), 1/*version*/, time::now_utc().rfc3339(), self.hostname, self.tag, pid, message);
    println!("formatted: {}", f);
    return f;
  }

  fn encode_priority(&self) -> Priority {
    return self.facility as uint | self.severity as uint
  }
}
