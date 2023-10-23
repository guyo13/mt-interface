#[derive(Clone, Copy)]
pub enum MtUtilCommandId {
    UTIL_GET_DEVICE_INFO = 0x00,
    UTIL_GET_NV_INFO = 0x01,
    UTIL_SET_PANID = 0x02,
    UTIL_SET_CHANNELS = 0x03,
    UTIL_SET_SECLEVEL = 0x04,
    UTIL_SET_PRECFGKEY = 0x05,
    UTIL_CALLBACK_SUB_CMD = 0x06,
    UTIL_KEY_EVENT = 0x07,
    UTIL_TIME_ALIVE = 0x09,
    UTIL_LED_CONTROL = 0x0A,
    UTIL_LOOPBACK = 0x10,
    UTIL_DATA_REQ = 0x11,
    UTIL_SRC_MATCH_ENABLE = 0x20,
    UTIL_SRC_MATCH_ADD_ENTRY = 0x21,
    UTIL_SRC_MATCH_DEL_ENTRY = 0x22,
    UTIL_SRC_MATCH_CHECK_SRC_ADDR = 0x23,
    UTIL_SRC_MATCH_ACK_ALL_PENDING = 0x24,
    UTIL_SRC_MATCH_CHECK_ALL_PENDING = 0x25,
    UTIL_ADDRMGR_EXT_ADDR_LOOKUP = 0x40,
    UTIL_ADDRMGR_NWK_ADDR_LOOKUP = 0x41,
    UTIL_APSME_LINK_KEY_DATA_GET = 0x44,
    UTIL_APSME_LINK_KEY_NV_ID_GET = 0x45,
    UTIL_APSME_REQUEST_KEY_CMD = 0x4B,
    UTIL_ASSOC_COUNT = 0x48,
    UTIL_ASSOC_FIND_DEVICE = 0x49,
    UTIL_ASSOC_GET_WITH_ADDRESS = 0x4A,
    UTIL_BIND_ADD_ENTRY = 0x4D,
    UTIL_ZCL_KEY_EST_INIT_EST = 0x80,
    UTIL_ZCL_KEY_EST_SIGN = 0x81,
    UTIL_SRNG_GEN = 0x4C,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtUtilCallbackId {
    UTIL_SYNC_REQ = 0xE0,
    UTIL_ZCL_KEY_ESTABLISH_IND = 0xE1,
    // TODO - implement ParseByte
}