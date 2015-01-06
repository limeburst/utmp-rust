extern crate time;

use time::Timespec;
use std::io::{File, SeekStyle};

#[derive(FromPrimitive)]
pub enum UtmpxRecordType {
    Empty,
    RunLevel,
    BootTime,
    OldTime,
    NewTime,
    InitProcess,
    LoginProcess,
    UserProcess,
    DeadProcess,
    Accounting,
    Signature,
    ShutdownTime,
}

impl Copy for UtmpxRecordType { }

pub struct UtmpxRecord {
    pub ut_user: Vec<u8>,
    pub ut_id: Vec<u8>,
    pub ut_line: Vec<u8>,
    pub ut_pid: i32,
    pub ut_type: UtmpxRecordType,
    pub timeval: Timespec,
    pub ut_host: Vec<u8>
}

pub fn getutmpx() -> Vec<UtmpxRecord> {
    let path = Path::new("/var/run/utmpx");
    let mut file = File::open(&path);
    let mut utmpx = Vec::new();
    loop {
        let ut_user = match file.read_exact(256) {
            Ok(x)   => x,
            Err(_)  => break
        };
        let ut_id = file.read_exact(4).unwrap();
        let ut_line = file.read_exact(32).unwrap();
        let ut_pid = file.read_le_i32().unwrap();
        let ut_type = FromPrimitive::from_i16(file.read_le_i16().unwrap()).unwrap();
        let _ = file.seek(2, SeekStyle::SeekCur);
        let sec = file.read_le_i32().unwrap();
        let nsec = file.read_le_i32().unwrap();
        let timeval = Timespec::new(sec as i64, nsec);
        let ut_host = file.read_exact(256).unwrap();
        let _ = file.seek(64, SeekStyle::SeekCur);
        let record = UtmpxRecord {
            ut_user: ut_user,
            ut_id: ut_id,
            ut_line: ut_line,
            ut_pid: ut_pid,
            ut_type: ut_type,
            timeval: timeval,
            ut_host: ut_host
        };
        utmpx.push(record);
    };
    utmpx
}
