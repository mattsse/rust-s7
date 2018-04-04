pub const ERR_CLI_MASK: u32 = 0xFFF0_0000;
pub const ERR_CLI_BASE: u32 = 0x000F_FFFF;

pub const ERR_CLI_INVALID_PARAMS: u32 = 0x0020_0000;
pub const ERR_CLI_JOB_PENDING: u32 = 0x0030_0000;
pub const ERR_CLI_TOO_MANY_ITEMS: u32 = 0x0040_0000;
pub const ERR_CLI_INVALID_WORD_LEN: u32 = 0x0050_0000;
pub const ERR_CLI_PARTIAL_DATA_WRITTEN: u32 = 0x0060_0000;
pub const ERR_CLI_SIZE_OVER_PDU: u32 = 0x0070_0000;
pub const ERR_CLI_INVALID_PLC_ANSWER: u32 = 0x0080_0000;
pub const ERR_CLI_ADDRESS_OUT_OF_RANGE: u32 = 0x0090_0000;
pub const ERR_CLI_INVALID_TRANSPORT_SIZE: u32 = 0x00A0_0000;
pub const ERR_CLI_WRITE_DATA_SIZE_MISMATCH: u32 = 0x00B0_0000;
pub const ERR_CLI_ITEM_NOT_AVAILABLE: u32 = 0x00C0_0000;
pub const ERR_C1LI_INVALID_VALUE: u32 = 0x00D0_0000;
pub const ERR_CLI_CANNOT_START_PLC: u32 = 0x00E0_0000;
pub const ERR_CLI_ALREADY_RUN: u32 = 0x00F0_0000;
pub const ERR_CLI_CANNOT_STOP_PLC: u32 = 0x0100_0000;
pub const ERR_CLI_CANNOT_COPY_RAM_TO_ROM: u32 = 0x0110_0000;
pub const ERR_CLI_CANNOT_COMPRESS: u32 = 0x0120_0000;
pub const ERR_CLI_ALREADY_STOP: u32 = 0x0130_0000;
pub const ERR_CLI_FUN_NOT_AVAILABLE: u32 = 0x0140_0000;
pub const ERR_CLI_UPLOAD_SEQUENCE_FAILED: u32 = 0x0150_0000;
pub const ERR_CLI_INVALID_DATA_SIZE_RECVD: u32 = 0x0160_0000;
pub const ERR_CLI_INVALID_BLOCK_TYPE: u32 = 0x0170_0000;
pub const ERR_CLI_INVALID_BLOCK_NUMBER: u32 = 0x0180_0000;
pub const ERR_CLI_INVALID_BLOCK_SIZE: u32 = 0x0190_0000;
pub const ERR_CLI_DOWNLOAD_SEQUENCE_FAILED: u32 = 0x01A0_0000;
pub const ERR_CLI_INSERT_REFUSED: u32 = 0x01B0_0000;
pub const ERR_CLI_DELETE_REFUSED: u32 = 0x01C0_0000;
pub const ERR_CLI_NEED_PASSWORD: u32 = 0x01D0_0000;
pub const ERR_CLI_INVALID_PASSWORD: u32 = 0x01E0_0000;
pub const ERR_CLI_NO_PASSWORD_TO_SET_OR_CLEAR: u32 = 0x01F0_0000;
pub const ERR_CLI_JOB_TIMEOUT: u32 = 0x0200_0000;
pub const ERR_CLI_PARTIAL_DATA_READ: u32 = 0x0210_0000;
pub const ERR_CLI_BUFFER_TOO_SMALL: u32 = 0x0220_0000;
pub const ERR_CLI_FUNCTION_REFUSED: u32 = 0x0230_0000;
pub const ERR_CLI_DESTROYING: u32 = 0x0240_0000;
pub const ERR_CLI_INVALID_PARAM_NUMBER: u32 = 0x0250_0000;
pub const ERR_CLI_CANNOT_CHANGE_PARAM: u32 = 0x0260_0000;

// Seconds between 1970/1/1 (C time base) and 1984/1/1 (Siemens base)
pub const DELTA_SECS: i64 = 441_763_200;

pub struct TS7DataItem {
    area: i32,
    word_len: i32,
    result: i32,
    dbnumber: i32,
    start: i32,
    amount: i32,
    // TODO was  void  *pdata;
    pdata: Vec<u8>,
}

pub struct TS7BlocksList {
    ob_count: i32,
    fb_count: i32,
    fc_count: i32,
    sfb_count: i32,
    sfc_count: i32,
    db_count: i32,
    sdb_count: i32,
}

pub struct TS7BlockInfo {
    blk_type: i32,
    blk_number: i32,
    blk_lang: i32,
    blk_flags: i32,
    // The real size in bytes
    mc7_size: i32,
    load_size: i32,
    local_data: i32,
    sbb_length: i32,
    check_sum: i32,
    version: i32,
    // Chars info
    code_date: [char; 11],
    intf_date: [char; 11],
    author: [char; 9],
    family: [char; 9],
    header: [char; 9],
}

pub struct TS7OrderCode {
    // Order code
    code: [char; 21],
    // Version v1.V2.V3
    v1: u8,
    v2: u8,
    v3: u8,
}

pub struct TS7CpuInfo {
    module_type_name: [char; 33],
    serial_number: [char; 25],
    as_name: [char; 25],
    copyright: [char; 27],
    module_name: [char; 25],
}

pub struct TS7CpInfo {
    max_pdu_length: i32,
    max_connections: i32,
    max_mpi_rate: i32,
    max_bus_rate: i32,
}

pub struct SzlHeader {
    length_r: u16,
    n_dr: u16,
}

pub struct TS7SZL {
    header: SzlHeader,
    data: [u8; 0x4000 - 4],
}

pub struct TS7Protection {
    sch_schal: u16,
    sch_par: u16,
    sch_rel: u16,
    bart_sch: u16,
    anl_sch: u16,
}

/// Low level : change them to experiment new connections, their defaults normally work well
pub const PC_ISO_SEND_TIMEOUT: i32 = 6;
pub const PC_ISO_RECV_TIMEOUT: i32 = 7;
pub const PC_ISO_CONN_TIMEOUT: i32 = 8;
pub const PC_ISO_SRC_REF: i32 = 1;
pub const PC_ISO_DST_REF: i32 = 2;
pub const PC_ISO_SRC_TSAP: i32 = 3;
pub const PC_ISO_DST_TSAP: i32 = 4;
pub const PC_ISO_ISO_PDU_SIZE: i32 = 5;

/// Client Connection Type
pub const CONNTYPE_PG: u16 = 0x01; // Connect to the PLC as a PG
pub const CONNTYPE_OP: u16 = 0x02; // Connect to the PLC as an OP
pub const CONNTYPE_BASIC: u16 = 0x03; // Basic connection

/// Internal struct for operations
/// Commands are not executed directly in the function such as "DBRead(...",
/// but this struct is filled and then PerformOperation() is called.
/// This allow us to implement async function very easily.

pub struct TSnap7Job {
    // Operation Code
    op: i32,
    // Operation result
    result: i32,
    // A Job is pending
    pending: bool,
    // Job Execution time
    time: u32,
    // Read/Write
    // Also used for Block type and Block of type
    area: i32,
    // Used for DB number, Block number
    number: i32,
    // Offset start
    start: i32,
    // Word length
    word_len: i32,
    // SZL
    // SZL id
    id: i32,
    // SZL index
    index: i32,
    // ptr info
    // TODO was void * p_data
    // User data pointer
    p_data: Vec<u8>,
    // Items amount/Size in input
    amount: i32,
    // Items amount/Size in output
    p_amount: i32,
    // Generic
    // Used for full upload and CopyRamToRom extended timeout
    i_param: i32,
}

pub struct TSnap7MicroClient {
    // TODO
}