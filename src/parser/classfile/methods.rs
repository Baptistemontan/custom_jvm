use crate::parser::utils::{pop_u16, pop_u2_as_index, FileByte, ParseError};

use super::{
    attributes::{parse_attribute_info, AttributeInfo},
    constant_pool::ConstantPool,
};

pub struct MethodAttributeInfo {}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: usize,
    descriptor_index: usize,
    attributes: Vec<AttributeInfo>,
}

fn parse_method_info<I>(
    bytes: &mut I,
    constant_pool: &ConstantPool,
) -> Result<MethodInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let access_flags = pop_u16(bytes)?;
    let name_index = pop_u2_as_index(bytes)?;
    // let method_name_index = constant_pool.get(name_index);
    let descriptor_index = pop_u2_as_index(bytes)?;
    let attributes_count: usize = pop_u2_as_index(bytes)?;
    let mut attributes = Vec::with_capacity(attributes_count);

    for _ in 0..attributes_count {
        let attribute = parse_attribute_info(bytes, constant_pool)?;
        attributes.push(attribute);
    }

    Ok(MethodInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    })
}

#[derive(Debug, Clone)]
pub struct Methods {
    methods: Vec<MethodInfo>,
}

pub fn parse_methods<I>(bytes: &mut I, constant_pool: &ConstantPool) -> Result<Methods, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let methods_count: usize = pop_u2_as_index(bytes)?;

    let mut methods = Vec::with_capacity(methods_count);

    for _ in 0..methods_count {
        let method_info = parse_method_info(bytes, constant_pool)?;
        methods.push(method_info);
    }

    Ok(Methods { methods })
}
