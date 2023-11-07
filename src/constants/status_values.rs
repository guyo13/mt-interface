#[derive(Clone, Copy)]
pub enum GeneralStatus {
    ZSuccess = 0x00,
    ZFailure = 0x01,
    ZInvalidParameter = 0x02,
    ZDecodeError = 0x03,
    NV_ITEM_UNINIT = 0x09,
    NV_OPER_FAILED = 0x0a,
    NV_BAD_ITEM_LEN = 0x0c,
    ZMemError = 0x10,
    ZBufferFull = 0x11,
    ZUnsupportedMode = 0x12,
    ZMacMemError = 0x13,
    ZSapiInProgress = 0x20,
    ZSapiTimeout = 0x21,
    ZSapiInit = 0x22,
    ZNotAuthorized = 0x7E,
    ZMalformedCmd = 0x80,
    ZUnsupClusterCmd = 0x81,
    ZOtaAbort = 0x95,
    ZOtaImageInvalid = 0x96,
    ZOtaWaitForData = 0x97,
    ZOtaNoImageAvailable = 0x98,
    ZOtaRequireMoreImage = 0x99,
    ZApsFail = 0xb1,
    ZApsTableFull = 0xb2,
    ZApsIllegalRequest = 0xb3,
    ZApsInvalidBinding = 0xb4,
    ZApsUnsupportedAttrib = 0xb5,
    ZApsNotSupported = 0xb6,
    ZApsNoAck = 0xb7,
    ZApsDuplicateEntry = 0xb8,
    ZApsNoBoundDevice = 0xb9,
    ZApsNotAllowed = 0xba,
    ZApsNotAuthenticated = 0xbb,
    ZSecNoKey = 0xa1,
    ZSecOldFrmCount = 0xa2,
    ZSecMaxFrmCount = 0xa3,
    ZSecCcmFail = 0xa4,
    ZNwkInvalidParam = 0xc1,
    ZNwkInvalidRequest = 0xc2,
    ZNwkNotPermitted = 0xc3,
    ZNwkStartupFailure = 0xc4,
    ZNwkTableFull = 0xc7,
    ZNwkUnknownDevice = 0xc8,
    ZNwkUnsupportedAttribute = 0xc9,
    ZNwkNoNetworks = 0xca,
    ZNwkLeaveUnconfirmed = 0xcb,
    ZNwkNoAck = 0xcc,
    ZNwkNoRoute = 0xcd,
    ZMacNoACK = 0xe9,
    ZAfDuplicateEndpoint = 0xd0,
    ZAfEndpointMax = 0xd1,
    ZIcallNoMsg = 0x30,
    ZIcallTimeout = 0x31,
}

pub enum ZDOStatus {
    SUCCESS = 0x00,
    INVALID_REQTYPE = 0x80,
    DEVICE_NOT_FOUND = 0x81,
    INVALID_EP = 0x82,
    NOT_ACTIVE = 0x83,
    NOT_SUPPORTED = 0x84,
    TIMEOUT = 0x85,
    NO_MATCH = 0x86,
    NO_ENTRY = 0x88,
    NO_DESCRIPTOR = 0x89,
    INSUFFICIENT_SPACE = 0x8a,
    NOT_PERMITTED = 0x8b,
    TABLE_FULL = 0x8c,
    NOT_AUTHORIZED = 0x8d,
    BINDING_TABLE_FULL = 0x8e,
}
