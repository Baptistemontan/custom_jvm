use crate::parser::utils::{pop_u16, pop_u2_as_index, FileByte, ParseError};

use super::{
    attributes::{parse_n_attributes, AttributeInfo},
    constant_pool::ConstantPool,
};

#[derive(Debug, Clone)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: usize,
    descriptor_index: usize,
    attributes: Vec<AttributeInfo>,
}

fn parse_field_info<I>(bytes: &mut I, constant_pool: &ConstantPool) -> Result<FieldInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let access_flags = pop_u16(bytes)?;
    let name_index = pop_u2_as_index(bytes)?;
    let descriptor_index = pop_u2_as_index(bytes)?;
    let attributes_count = pop_u2_as_index(bytes)?;
    let attributes = parse_n_attributes(bytes, attributes_count, constant_pool)?;

    Ok(FieldInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    })
}

#[derive(Debug, Clone)]
pub struct Fields {
    fields: Vec<FieldInfo>,
}

pub fn parse_fields<I>(bytes: &mut I, constant_pool: &ConstantPool) -> Result<Fields, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let fields_count: usize = pop_u2_as_index(bytes)?;

    let mut fields = Vec::with_capacity(fields_count);

    for _ in 0..fields_count {
        let field_info = parse_field_info(bytes, constant_pool)?;
        fields.push(field_info);
    }

    Ok(Fields { fields })
}
