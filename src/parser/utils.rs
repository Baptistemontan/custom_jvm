use std::io::Error;

pub type FileByte = Result<u8, Error>;

#[derive(Debug)]
pub enum ParseError {
    EndOfStream,
    IoError(std::io::Error),
    InvalidTag(u8),
    BadFileFormat(u32),
    InvalidOpcodeJumpIndex,
    BadConstPoolIndex {
        target_index: usize,
        pool_size: usize,
    },
    InvalidTableSwitchBounds,
    InvalidOpCode,
    InvalidWideOpCode,
    InvalidMethodHandleKind(u8),
}

pub fn pop1<I>(bytes: &mut I) -> Result<u8, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    if let Some(next) = bytes.next() {
        next.map_err(ParseError::IoError)
    } else {
        Err(ParseError::EndOfStream)
    }
}

pub fn pop2<I>(bytes: &mut I) -> Result<[u8; 2], ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let high_half = pop1(bytes)?;
    let low_half = pop1(bytes)?;
    Ok([high_half, low_half])
}

pub fn pop_u16<I>(bytes: &mut I) -> Result<u16, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    pop2(bytes).map(u16::from_be_bytes)
}

pub fn pop_u32<I>(bytes: &mut I) -> Result<u32, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    pop4(bytes).map(u32::from_be_bytes)
}

pub fn pop4<I>(bytes: &mut I) -> Result<[u8; 4], ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let [high_half_h, high_half_l] = pop2(bytes)?;
    let [low_half_h, low_half_l] = pop2(bytes)?;
    Ok([high_half_h, high_half_l, low_half_h, low_half_l])
}

pub fn pop8<I>(bytes: &mut I) -> Result<[u8; 8], ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let mut bits = [0; 8];
    let high_half = pop4(bytes)?;
    let low_half = pop4(bytes)?;
    bits[..4].copy_from_slice(&high_half);
    bits[4..].copy_from_slice(&low_half);
    Ok(bits)
}

pub fn pop_u1_as_index<I>(bytes: &mut I) -> Result<usize, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    pop1(bytes).map(usize::from)
}

pub fn pop_u2_as_index<I>(bytes: &mut I) -> Result<usize, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    pop_u16(bytes).map(usize::from)
}

pub fn pop_u4_as_index<I>(bytes: &mut I) -> Result<usize, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let quad = pop_u32(bytes)?;

    Ok(quad as usize)
}

pub fn pop_n<I>(bytes: &mut I, n: usize) -> Result<Vec<u8>, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let mut bytes_buff = Vec::with_capacity(n);
    for _ in 0..n {
        let byte = pop1(bytes)?;
        bytes_buff.push(byte);
    }
    Ok(bytes_buff)
}

pub fn pop_u2_as_offset<I>(bytes: &mut I) -> Result<i16, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let bits = pop2(bytes)?;
    let offset = i16::from_be_bytes(bits);
    Ok(offset)
}

pub fn pop_u4_as_offset<I>(bytes: &mut I) -> Result<i32, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let bits = pop4(bytes)?;
    let offset = i32::from_be_bytes(bits);
    Ok(offset)
}

pub fn skip_n<I>(bytes: &mut I, n: usize) -> Result<(), ParseError>
where
    I: Iterator<Item = FileByte>,
{
    if bytes.take(n).count() != n {
        Err(ParseError::EndOfStream)
    } else {
        Ok(())
    }
}
