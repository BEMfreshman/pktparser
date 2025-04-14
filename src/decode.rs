
#[inline]
pub fn le_u64_7<'a>(input: &'a [u8]) -> u64
{
    let mut full_bytes = [0u8;8];
    full_bytes[..7].copy_from_slice(input);
    u64::from_le_bytes(full_bytes)
}

#[inline]
pub fn be_u64_7<'a>(input: &'a [u8]) -> u64 
{
    let mut full_bytes = [0u8;8];
    full_bytes[1..].copy_from_slice(input);
    u64::from_be_bytes(full_bytes)
}

#[inline]
pub fn u64_7<'a>(endianness: nom::number::Endianness, input: &'a [u8]) -> u64
{
    if endianness == nom::number::Endianness::Big {
        be_u64_7(input)
    } else {
        le_u64_7(input)
    }
}

#[inline]
pub fn le_f64_5<'a>(input: &'a [u8]) -> f64
{
    let mut full_bytes = [0u8;8];
    full_bytes[..5].copy_from_slice(input);
    f64::from_le_bytes(full_bytes)
}

#[inline]
pub fn be_f64_5<'a>(input: &'a [u8]) -> f64 
{
    let mut full_bytes = [0u8;8];
    full_bytes[3..].copy_from_slice(input);
    f64::from_be_bytes(full_bytes)
}

#[inline]
pub fn f64_5<'a>(endianness: nom::number::Endianness, input: &'a [u8]) -> f64
{
    if endianness == nom::number::Endianness::Big {
        be_f64_5(input)
    } else {
        le_f64_5(input)
    }
}
