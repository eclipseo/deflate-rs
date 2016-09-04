use std::io::{Write, Result};

// CM = 8 means to use the DEFLATE compression method
const DEFAULT_CM: u8 = 8;
// CINFO = 7 Indicates a 32k window size
const DEFAULT_CINFO: u8 = 7 << 4;
const DEFAULT_CMF: u8 = DEFAULT_CM | DEFAULT_CINFO;
// const DEFAULT_FCHECK:
// No dict by default
const DEFAULT_FDICT: u8 = 0;
// FLEVEL = 0 means fastest compression algorithm
const DEFAULT_FLEVEL: u8 = 0 << 7;

// The 16-bit value consisting of CMF and FLG must be divisible by this to be valid
const FCHECK_DIVISOR: u8 = 31;

#[repr(u8)]
pub enum CompressionLevel {
    Fastest = 0,
    Fast = 1,
    Default = 2,
    Maximum = 3,
}

/// Generate FCHECK from CMF and FLG (without FCKECH )so that they are correct according to the specification,
/// i.e (CMF*256 + FCHK) % 31 = 0
/// Returns flg with the FCHKECK bits added (any existing FCHECK bits are ignored)
fn add_fcheck(cmf: u8, flg: u8) -> u8 {
    let rem = ((usize::from(cmf) * 256) + usize::from(flg)) % usize::from(FCHECK_DIVISOR);

    // Clear existing FCHECK if any
    let flg = flg & 0b00000111;

    // Casting is safe as rem can't overflow since it is a value mod 31
    // We can simply add the value to flg as (31 - rem) will never be above 2^5
    flg + (FCHECK_DIVISOR - rem as u8)
}

pub fn write_zlib_header<W: Write>(level: CompressionLevel, writer: &mut W) -> Result<()> {
    let cmf = DEFAULT_CMF;
    let flg = add_fcheck(cmf, (level as u8) << 6);
    let bytes = [cmf, flg];
    writer.write_all(&bytes)
}

#[cfg(test)]
mod test {
    use super::DEFAULT_CMF;

    #[test]
    fn test_gen_fcheck() {
        let cmf = DEFAULT_CMF;
        let flg = super::add_fcheck(DEFAULT_CMF, super::CompressionLevel::Default as u8 | super::DEFAULT_FDICT);
        assert_eq!(((usize::from(cmf) * 256) + usize::from(flg)) % 31, 0);
    }
}
