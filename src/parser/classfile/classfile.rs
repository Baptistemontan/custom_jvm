use crate::parser::utils::{pop4, pop_u16, pop_u2_as_index, FileByte, ParseError};

use super::{
    attributes::{parse_attributes, Attributes},
    constant_pool::{parse_constant_pool, ConstantPool},
    fields::{parse_fields, Fields},
    interfaces::{parse_interfaces, Interfaces},
    methods::{parse_methods, Methods},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccesFlag(u16);

/*
    U4:                 magic
    U2:                 minor_version
    U2:                 major_version
    U2:                 constant_pool_count | size of next array
    cp_info[]:          constant_pool[size - 1] | contain information to creat the constant pool
    U2:                 access_flag
    U2:                 this_class
    U2:                 super_class
    U2:                 interface_count | size of next array
    U2[]:               interfaces[size]
    U2:                 fields_count | size of next array
    field_info[]:       fields[size]
    U2:                 methods_count | size of next array
    method_info[]:      methods[size]
    U2:                 attributes_count | size of next array
    attribute_info[]:   attributes[attributes_count];
*/

pub fn parse_class_file<I>(bytes: &mut I) -> Result<ClassFile, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let magic_bits = pop4(bytes)?;
    let magic = u32::from_be_bytes(magic_bits);

    if magic != 0xcafebabe {
        return Err(ParseError::BadFileFormat(magic));
    }

    let minor_version = pop_u16(bytes)?;
    let major_version = pop_u16(bytes)?;

    let constant_pool = parse_constant_pool(bytes)?;

    let acces_flag = AccesFlag(pop_u16(bytes)?);
    let this_class = pop_u2_as_index(bytes)?;
    let super_class = pop_u2_as_index(bytes)?;

    let interfaces = parse_interfaces(bytes)?;

    let fields = parse_fields(bytes, &constant_pool)?;

    let methods = parse_methods(bytes, &constant_pool)?;

    let attributes = parse_attributes(bytes, &constant_pool)?;

    Ok(ClassFile {
        magic,
        minor_version,
        major_version,
        this_class,
        super_class,
        constant_pool,
        acces_flag,
        interfaces,
        fields,
        methods,
        attributes,
    })
}

#[derive(Debug, Clone)]
pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    this_class: usize,
    super_class: usize,
    constant_pool: ConstantPool,
    acces_flag: AccesFlag,
    interfaces: Interfaces,
    fields: Fields,
    methods: Methods,
    attributes: Attributes,
}
