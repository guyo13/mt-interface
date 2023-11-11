#[derive(Clone, Copy)]
pub enum ScanChannels {
    None = 0x00000000,
    AllChannels = 0x07FFF800,
    Channel11 = 0x00000800,
    Channel12 = 0x00001000,
    Channel13 = 0x00002000,
    Channel14 = 0x00004000,
    Channel15 = 0x00008000,
    Channel16 = 0x00010000,
    Channel17 = 0x00020000,
    Channel18 = 0x00040000,
    Channel19 = 0x00080000,
    Channel20 = 0x00100000,
    Channel21 = 0x00200000,
    Channel22 = 0x00400000,
    Channel23 = 0x00800000,
    Channel24 = 0x01000000,
    Channel25 = 0x02000000,
    Channel26 = 0x04000000,
}