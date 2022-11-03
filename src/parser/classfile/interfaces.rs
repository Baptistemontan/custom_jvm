use crate::parser::utils::{pop_u2_as_index, FileByte, ParseError};

#[derive(Debug, Clone)]
pub struct Interfaces {
    interfaces: Vec<usize>,
}

pub fn parse_interfaces<I>(bytes: &mut I) -> Result<Interfaces, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let interfaces_count: usize = pop_u2_as_index(bytes)?;

    let mut interfaces = Vec::with_capacity(interfaces_count);

    for _ in 0..interfaces_count {
        let interface_index = pop_u2_as_index(bytes)?;
        interfaces.push(interface_index);
    }

    Ok(Interfaces { interfaces })
}
