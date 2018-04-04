/// area id
const S7_AREA_PE: u8 = 0x81;
const S7_AREA_PA: u8 = 0x82;
const S7_AREA_MK: u8 = 0x83;
const S7_AREA_DB: u8 = 0x84;
const S7_AREA_CT: u8 = 0x1C;
const S7_AREA_TM: u8 = 0x1D;

const MAX_VARS: i32 = 20;

const S7_WLBIT: i32 = 0x01;
const S7_WLBYTE: i32 = 0x02;
const S7_WLCHAR: i32 = 0x03;
const S7_WLWORD: i32 = 0x04;
const S7_WLINT: i32 = 0x05;
const S7_WLDWORD: i32 = 0x06;
const S7_WLDINT: i32 = 0x07;
const S7_WLREAL: i32 = 0x08;
const S7_WLCOUNTER: i32 = 0x1C;
const S7_WLTIMER: i32 = 0x1D;

/// Block type
const BLOCK_OB: u8 = 0x38;
const BLOCK_DB: u8 = 0x41;
const BLOCK_SDB: u8 = 0x42;
const BLOCK_FC: u8 = 0x43;
const BLOCK_SFC: u8 = 0x44;
const BLOCK_FB: u8 = 0x45;
const BLOCK_SFB: u8 = 0x46;

/// Sub Block Type
const SUB_BLK_OB: u8 = 0x08;
const SUB_BLK_DB: u8 = 0x0A;
const SUB_BLK_SDB: u8 = 0x0B;
const SUB_BLK_FC: u8 = 0x0C;
const SUB_BLK_SFC: u8 = 0x0D;
const SUB_BLK_FB: u8 = 0x0E;
const SUB_BLK_SFB: u8 = 0x0F;

/// Block languages
const BLOCK_LANG_AWL: u8 = 0x01;
const BLOCK_LANG_KOP: u8 = 0x02;
const BLOCK_LANG_FUP: u8 = 0x03;
const BLOCK_LANG_SCL: u8 = 0x04;
const BLOCK_LANG_DB: u8 = 0x05;
const BLOCK_LANG_GRAPH: u8 = 0x06;

/// CPU status
const S7_CPU_STATUS_UNKNOWN: u8 = 0x00;
const S7_CPU_STATUS_RUN: u8 = 0x08;
const S7_CPU_STATUS_STOP: u8 = 0x04;

const EVC_SNAP7_BASE: u32 = 0x0000_8000;
/// S7 Server Event code
const EVC_PDU_INCOMING: u32 = 0x0001_0000;
const EVC_DATA_READ: u32 = 0x0002_0000;
const EVC_DATA_WRITE: u32 = 0x0004_0000;
const EVC_NEGOTIATE_PDU: u32 = 0x0008_0000;
const EVC_READ_SZL: u32 = 0x0010_0000;
const EVC_CLOCK: u32 = 0x0020_0000;
const EVC_UPLOAD: u32 = 0x0040_0000;
const EVC_DOWNLOAD: u32 = 0x0080_0000;
const EVC_DIRECTORY: u32 = 0x0100_0000;
const EVC_SECURITY: u32 = 0x0200_0000;
const EVC_CONTROL: u32 = 0x0400_0000;
const EVC_RESERVED_08000000: u32 = 0x0800_0000;
const EVC_RESERVED_10000000: u32 = 0x1000_0000;
const EVC_RESERVED_20000000: u32 = 0x2000_0000;
const EVC_RESERVED_40000000: u32 = 0x4000_0000;
const EVC_RESERVED_80000000: u32 = 0x8000_0000;
/// Event SubCodes
const EVS_UNKNOWN: u16 = 0x0000;
const EVS_START_UPLOAD: u16 = 0x0001;
const EVS_START_DOWNLOAD: u16 = 0x0001;
const EVS_GET_BLOCK_LIST: u16 = 0x0001;
const EVS_START_LIST_BO_T: u16 = 0x0002;
const EVS_LIST_BO_T: u16 = 0x0003;
const EVS_GET_BLOCK_INFO: u16 = 0x0004;
const EVS_GET_CLOCK: u16 = 0x0001;
const EVS_SET_CLOCK: u16 = 0x0002;
const EVS_SET_PASSWORD: u16 = 0x0001;
const EVS_CLR_PASSWORD: u16 = 0x0002;
// Event result
const EVR_NO_ERROR: u16 = 0;
const EVR_FRAGMENT_REJECTED: u16 = 0x0001;
const EVR_MALFORMED_PDU: u16 = 0x0002;
const EVR_SPARSE_BYTES: u16 = 0x0003;
const EVR_CANNOT_HANDLE_PDU: u16 = 0x0004;
const EVR_NOT_IMPLEMENTED: u16 = 0x0005;
const EVR_ERR_EXCEPTION: u16 = 0x0006;
const EVR_ERR_AREA_NOT_FOUND: u16 = 0x0007;
const EVR_ERR_OUT_OF_RANGE: u16 = 0x0008;
const EVR_ERR_OVER_PDU: u16 = 0x0009;
const EVR_ERR_TRANSPORT_SIZE: u16 = 0x000A;
const EVR_INVALID_GROUP_UDATA: u16 = 0x000B;
const EVR_INVALID_SZL: u16 = 0x000C;
const EVR_DATA_SIZE_MISMATCH: u16 = 0x000D;
const EVR_CANNOT_UPLOAD: u16 = 0x000E;
const EVR_CANNOT_DOWNLOAD: u16 = 0x000F;
const EVR_UPLOAD_INVALID_ID: u16 = 0x0010;
const EVR_RES_NOT_FOUND: u16 = 0x0011;

/// Async mode
const AM_POLLING: i32 = 0;
const AM_EVENT: i32 = 1;
const AM_CALL_BACK: i32 = 2;

const P_U16_LOCAL_PORT: i32 = 1;
const P_U16_REMOTE_PORT: i32 = 2;
const P_I32_PING_TIMEOUT: i32 = 3;
const P_I32_SEND_TIMEOUT: i32 = 4;
const P_I32_RECV_TIMEOUT: i32 = 5;
const P_I32_WORK_INTERVAL: i32 = 6;
const P_U16_SRC_REF: i32 = 7;
const P_U16_DST_REF: i32 = 8;
const P_U16_SRC_TSAP: i32 = 9;
const P_I32_PDUREQUEST: i32 = 10;
const P_I32_MAX_CLIENTS: i32 = 11;
const P_I32_BSEND_TIMEOUT: i32 = 12;
const P_I32_BRECV_TIMEOUT: i32 = 13;
const P_U32_RECOVERY_TIME: i32 = 14;
const P_U32_KEEP_ALIVE_TIME: i32 = 15;

// Bool param is passed as i32 : 0->false, 1->true
// String param (only set) is passed as pointer
//-----------------------------------------------------------------------------
//                               INTERNALS CONSTANTS
//------------------------------------------------------------------------------

const DB_MAX_NAME: u16 = 0xFFFF; // max number (name) of DB

const ERR_S7_MASK: u32 = 0xFFF0_0000;
const ERR_S7_BASE: u32 = 0x000F_FFFF;
const ERR_S7_NOT_CONNECTED: u32 = ERR_S7_BASE + 0x0001;
/// Client not connected
const ERR_S7_INVALID_MODE: u32 = ERR_S7_BASE + 0x0002;
/// Requested a connection to...
const ERR_S7_INVALID_PDUIN: u32 = ERR_S7_BASE + 0x0003; // Malformed input PDU

/// S7 outcoming Error code
const CODE_7OK: u16 = 0x0000;
const CODE_7ADDRESS_OUT_OF_RANGE: u16 = 0x0005;
const CODE_7INVALID_TRANSPORT_SIZE: u16 = 0x0006;
const CODE_7WRITE_DATA_SIZE_MISMATCH: u16 = 0x0007;
const CODE_7RES_ITEM_NOT_AVAILABLE: u16 = 0x000A;
const CODE_7RES_ITEM_NOT_AVAILABLE1: u16 = 0xD209;
const CODE_7INVALID_VALUE: u16 = 0xDC01;
const CODE_7NEED_PASSWORD: u16 = 0xD241;
const CODE_7INVALID_PASSWORD: u16 = 0xD602;
const CODE_7NO_PASSWORD_TO_CLEAR: u16 = 0xD604;
const CODE_7NO_PASSWORD_TO_SET: u16 = 0xD605;
const CODE_7FUN_NOT_AVAILABLE: u16 = 0x8104;
const CODE_7DATA_OVER_PDU: u16 = 0x8500;

/// result transport size
const TS_RES_BIT: u8 = 0x03;
const TS_RES_BYTE: u8 = 0x04;
const TS_RES_INT: u8 = 0x05;
const TS_RES_REAL: u8 = 0x07;
const TS_RES_OCTET: u8 = 0x09;

/// Client Job status (lib internals, not S7)
const JOB_COMPLETE: i32 = 0;
const JOB_PENDING: i32 = 1;

/// Control codes
const CODE_CONTROL_UNKNOWN: u16 = 0;
const CODE_CONTROL_COLD_START: u16 = 1;
// Cold start
const CODE_CONTROL_WARM_START: u16 = 2;
// Warm start
const CODE_CONTROL_STOP: u16 = 3;
// Stop
const CODE_CONTROL_COMPRESS: u16 = 4;
// Compress
const CODE_CONTROL_CPY_RAM_ROM: u16 = 5;
// Copy Ram to Rom
const CODE_CONTROL_INS_DEL: u16 = 6;
// Insert in working ram the block downloaded
/// Delete from working ram the block selected
/// PDU Type
const PDU_TYPE_REQUEST: u8 = 1;
// family request
const PDU_TYPE_RESPONSE: u8 = 3;
// family response
const PDU_TYPE_USERDATA: u8 = 7; // family user data

/// PDU Functions
const PDU_RESPONSE: u8 = 0x02;
// Response (when error)
const PDU_FUNC_READ: u8 = 0x04;
// Read area
const PDU_FUNC_WRITE: u8 = 0x05;
// Write area
const PDU_NEGOTIATE: u8 = 0xF0;
// Negotiate PDU length
const PDU_START: u8 = 0x28;
// CPU start
const PDU_STOP: u8 = 0x29;
// CPU stop
const PDU_START_UPLOAD: u8 = 0x1D;
// start Upload
const PDU_UPLOAD: u8 = 0x1E;
// Upload
const PDU_END_UPLOAD: u8 = 0x1F;
// EndUpload
const PDU_REQ_DOWNLOAD: u8 = 0x1A;
// start Download request
const PDU_DOWNLOAD: u8 = 0x1B;
// Download request
const PDU_DOWNLOAD_ENDED: u8 = 0x1C;
// Download end request
const PDU_CONTROL: u8 = 0x28; // Control (insert/delete..)

/// PDU SubFunctions
const SFUN_LIST_ALL: u8 = 0x01;
/// List all blocks
const SFUN_LIST_BO_T: u8 = 0x02;
/// List Blocks of type
const SFUN_BLK_INFO: u8 = 0x03;
/// Get Block info
const SFUN_READ_SZL: u8 = 0x01;
/// Read SZL
const SFUN_READ_CLOCK: u8 = 0x01;
/// Read Clock (Date and time)
const SFUN_SET_CLOCK: u8 = 0x02;
/// Set Clock (Date and time)
const SFUN_ENTER_PWD: u8 = 0x01;
/// Enter password    for this session
const SFUN_CANCEL_PWD: u8 = 0x02;
/// Cancel password    for this session
const SFUN_INSERT: u8 = 0x50;
/// Insert block
const SFUN_DELETE: u8 = 0x42; // Delete block
