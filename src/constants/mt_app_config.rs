#[derive(Clone, Copy)]
pub enum MtAppConfigCommandId {
    APP_CNF_SET_NWK_FRAME_COUNTER = 0xFF,
    APP_CNF_SET_DEFAULT_REMOTE_ENDDEVICE_TIMEOUT = 0x01,
    APP_CNF_SET_ENDDEVICETIMEOUT = 0x02,
    APP_CNF_SET_ALLOWREJOIN_TC_POLICY = 0x03,
    APP_CNF_BDB_START_COMMISSIONING = 0x05,
    APP_CNF_BDB_SET_CHANNEL = 0x08,
    APP_CNF_BDB_ADD_INSTALLCODE = 0x04,
    APP_CNF_BDB_SET_TC_REQUIRE_KEY_EXCHANGE = 0x09,
    APP_CNF_BDB_SET_JOINUSESINSTALLCODEKEY = 0x06,
    APP_CNF_BDB_SET_ACTIVE_DEFAULT_CENTRALIZED_KEY = 0x07,
    APP_CNF_BDB_ZED_ATTEMPT_RECOVER_NWK = 0x0A,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtAppConfigCallbackId {
    APP_CNF_BDB_COMMISSIONING_NOTIFICATION = 0x80,
    // TODO - implement ParseByte
}