const ISO_PAYLOAD_SIZE: usize = 4096;

const ERR_ISO_MASK: u32 = 0x000F_0000;
const ERR_ISO_BASE: u32 = 0x0000_FFFF;

const ERR_ISO_CONNECT: u32 = 0x0001_0000; // Connection error
const ERR_ISO_DISCONNECT: u32 = 0x0002_0000; // Disconnect error
const ERR_ISO_INVALID_PDU: u32 = 0x0003_0000; // Bad format
const ERR_ISO_INVALID_DATA_SIZE: u32 = 0x0004_0000; // Bad Datasize passed to send/recv : buffer is invalid
const ERR_ISO_NULL_POINTER: u32 = 0x0005_0000; // Null passed as pointer
const ERR_ISO_SHORT_PACKET: u32 = 0x0006_0000; // A short packet received
const ERR_ISO_TOO_MANY_FRAGMENTS: u32 = 0x0007_0000; // Too many packets without EoT flag
const ERR_ISO_PDU_OVERFLOW: u32 = 0x0008_0000; // The sum of fragments data exceded maximum packet size
const ERR_ISO_SEND_PACKET: u32 = 0x0009_0000; // An error occurred during send
const ERR_ISO_RECV_PACKET: u32 = 0x000A_0000; // An error occurred during recv
const ERR_ISO_INVALID_PARAMS: u32 = 0x000B_0000; // Invalid TSAP params
const ERR_ISO_RESVD_1: u32 = 0x000C_0000; // Unassigned
const ERR_ISO_RESVD_2: u32 = 0x000D_0000; // Unassigned
const ERR_ISO_RESVD_3: u32 = 0x000E_0000; // Unassigned
const ERR_ISO_RESVD_4: u32 = 0x000F_0000; // Unassigned

const ISO_OPT_TCP_NODELAY: u32 = 0x0000_0001; // Disable Nagle algorithm
const ISO_OPT_INSIDE_MTU: u32 = 0x0000_0002; // Max packet size < MTU ethernet card

/// TPKT header - ISO on TCP - RFC 1006 (4 bytes)
#[derive(Debug)]
pub struct TTPKT {
    version: u8,   // Always 3 for RFC 1006
    reserved: u8,  // 0
    hi_lenght: u8, // High part of packet lenght (entire frame, payload and TPDU included)
    lo_lenght: u8, // Low part of packet lenght (entire frame, payload and TPDU included)
}

pub struct TcoptParams {
    pdu_size_code: u8,
    pdu_size_len: u8,
    pdu_size_val: u8,
    tsap: [u8; 245], // We don't know in advance these fields....
}

pub struct TcotpCo {
    h_length: u8, // header length : initialized to 6 (length without params - 1)
    // descending classes that add values in params field must update it.
    pdut_ype: u8, // 0xE0 Connection request
    // 0xD0 Connection confirm
    // 0x80 Disconnect request
    // 0xDC Disconnect confirm
    dst_ref: u16, // Destination reference : Always 0x0000
    src_ref: u16, // Source reference : Always 0x0000
    co_r: u8,
    params: TcoptParams,
}

/// cotp header for DATA EXCHANGE
#[derive(Debug)]
pub struct TcotpDt {
    h_length: u8, // header length : 3 for this header
    pdu_type: u8, // 0xF0 for this header
    eot_num: u8,  // EOT (bit 7) + PDU number (bits 0..6)
                  // EOT = 1 -> End of Trasmission Packet (This packet is complete)
                  // PDU number : Always 0
}

// Info part of a PDU, only common parts. We use it to check the consistence
// of a telegram regardless of it's nature (control or data).
#[derive(Debug)]
pub struct TIsoHeaderInfo {
    ttpkt: TTPKT, // TPKT header
    // Common part of any cotp
    h_length: u8, // header length : 3 for this header
    pdu_type: u8, // Pdu type
}

/// PDU Type consts (code + Credit)
const PDU_TYPE_CR: u8 = 0xE0; // Connection request
const PDU_TYPE_CC: u8 = 0xD0; // Connection confirm
const PDU_TYPE_DR: u8 = 0x80; // Disconnect request
const PDU_TYPE_DC: u8 = 0xC0; // Disconnect confirm
const PDU_TYPE_DT: u8 = 0xF0; // Data transfer

const PDU_EO_T: u8 = 0x80; // End of Trasmission Packet (This packet is complete)

const DATA_HEADER_SIZE: usize = 4 + 3; // sizeof(ttpkt) + sizeof(TcotpDt)
const ISO_FRAME_SIZE: usize = ISO_PAYLOAD_SIZE + DATA_HEADER_SIZE;

#[derive(Debug)]
pub struct TIsoControlPDU {
    ttpkt: TTPKT,  // ttpkt header
    cotp: TcotpDt, // COPT header for CONNECTION stuffs
}

pub struct TIsoDataPDU {
    ttpkt: TTPKT,                    // TPKT header
    cotp: TcotpDt,                   // COPT header for DATA EXCHANGE
    payload: [u8; ISO_PAYLOAD_SIZE], // Payload
}

#[derive(Debug)]
pub enum TPDUKind {
    PkConnectionRequest,
    PkDisconnectRequest,
    PkEmptyFragment,
    PkInvalidPdu,
    PkUnrecognizedType,
    PkValidData,
}
