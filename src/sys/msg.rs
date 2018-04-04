pub const ICMP_ECHORP: u8 = 0; // ECHO Reply
pub const ICMP_ECHORQ: u8 = 8; // ECHO Request

pub struct TIPHeader {
    ip_hl_v: u8,
    ip_tos: u8,
    ip_len: u16,
    ip_id: u16,
    ip_off: u16,
    ip_ttl: u8,
    ip_p: u8,
    ip_sum: u16,
    ip_src: u32,
    ip_dst: u32,
}

pub struct TIcmpHeader {
    // Type of message
    ic_type: u8,
    // code
    ic_code: u8,
    // 16 bit checksum
    ic_cksum: u16,
    // id (ic1 : ipv4)
    ic_id: u16,
    // Sequence
    ic_seq: u16,
}

pub struct TIcmpPacket {
    header: TIcmpHeader,
    // use the well known default
    data: [u8; 32],
}

pub struct TIcmpReply {
    iph: TIPHeader,
    icmp_reply: TIcmpPacket,
}

pub struct TRawSocketPinger {}

pub struct TPinger {}
