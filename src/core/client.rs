use core::micro_client::*;

pub struct TSnap7Client {
    connection_type:u16,
    job_start: u32,
    job: TSnap7Job
}