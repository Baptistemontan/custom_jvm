use crate::parser::utils::{pop_u2_as_index, pop_u4_as_index, FileByte, ParseError, skip_n};

use super::{
    constant_pool::{ConstantInfo, ConstantPool},
    opcode::{parse_n_opcodes, OpCode},
};

#[derive(Debug, Clone)]
pub enum Attribute {
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute),
    StackMapTable,       // TODO
    BootStrapMethod,     // TODO
    NestHost,            // TODO
    NestMembers,         // TODO
    PermittedSubclasses, // TODO
    Exceptions,          // TODO
    InnerClasses,        // TODO
    EnclosingMethod,     // TODO
    Synthetic,           // TODO
    Signature,           // TODO
    Record,              // TODO
    SourceFile(SourceFileAttribute),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable,     // TODO
    LocalVariableTypeTable, // TODO
    // other attributes but not critical (see specs 4.7.3)
    Unknown(String), // TODO: String for debug pupose, remove after
}

#[derive(Debug, Clone)]
pub struct AttributeInfo {
    attribute_name_index: usize,
    attribute: Attribute,
}

fn get_attribute_name(
    constant_pool: &ConstantPool,
    attribute_name_index: usize,
) -> Result<&str, ParseError> {
    if let Some(ConstantInfo::Utf8(str)) = constant_pool.get(attribute_name_index) {
        Ok(&str)
    } else {
        let pool_size = constant_pool.size();
        Err(ParseError::BadConstPoolIndex {
            target_index: attribute_name_index,
            pool_size,
        })
    }
}

pub fn parse_attribute_info<I>(
    bytes: &mut I,
    constant_pool: &ConstantPool,
) -> Result<AttributeInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let attribute_name_index = pop_u2_as_index(bytes)?;
    let attribute_len: usize = pop_u4_as_index(bytes)?;
    let name = get_attribute_name(constant_pool, attribute_name_index)?;

    let attribute = match name {
        "ConstantValue" => Attribute::ConstantValue(parse_constant_value(bytes, attribute_len)?),
        "Code" => Attribute::Code(parse_code_attribute(bytes, constant_pool)?),
        "LineNumberTable" => Attribute::LineNumberTable(parse_line_number_table_attribute(bytes)?),
        "SourceFile" => Attribute::SourceFile(parse_source_file_attribute(bytes)?),
        _ => {
            // silently ignore unknown attributes

            // still need to skip the bytes
            skip_n(bytes, attribute_len)?;
            Attribute::Unknown(name.to_string())
        }
    };

    Ok(AttributeInfo {
        attribute_name_index,
        attribute,
    })
}

#[derive(Debug, Clone)]
pub struct Attributes {
    attributes: Vec<AttributeInfo>,
}

pub fn parse_n_attributes<I>(
    bytes: &mut I,
    attributes_count: usize,
    constant_pool: &ConstantPool,
) -> Result<Vec<AttributeInfo>, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let mut attributes = Vec::with_capacity(attributes_count);

    for _ in 0..attributes_count {
        let attribute_info = parse_attribute_info(bytes, constant_pool)?;
        attributes.push(attribute_info);
    }

    Ok(attributes)
}

pub fn parse_attributes<I>(
    bytes: &mut I,
    constant_pool: &ConstantPool,
) -> Result<Attributes, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let attributes_count: usize = pop_u2_as_index(bytes)?;

    let attributes = parse_n_attributes(bytes, attributes_count, constant_pool)?;

    Ok(Attributes { attributes })
}

#[derive(Debug, Clone)]
pub struct ConstantValueAttribute {
    constant_value_index: usize,
}

fn parse_constant_value<I>(
    bytes: &mut I,
    attribute_len: usize,
) -> Result<ConstantValueAttribute, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let constant_value_index = pop_u2_as_index(bytes)?;

    Ok(ConstantValueAttribute {
        constant_value_index,
    })
}

#[derive(Debug, Clone)]
pub struct ExceptionTableInfo {
    start_pc: usize,
    end_pc: usize,
    handler_pc: usize,
    catch_type: usize,
}

#[derive(Debug, Clone)]
pub struct CodeAttribute {
    max_stack: usize,
    max_locals: usize,
    code: Vec<OpCode>,
    exception_table: Vec<ExceptionTableInfo>,
    attributes: Vec<AttributeInfo>,
}

fn parse_exception_table_info<I>(bytes: &mut I) -> Result<ExceptionTableInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let start_pc = pop_u2_as_index(bytes)?;
    let end_pc = pop_u2_as_index(bytes)?;
    let handler_pc = pop_u2_as_index(bytes)?;
    let catch_type = pop_u2_as_index(bytes)?;

    Ok(ExceptionTableInfo {
        start_pc,
        end_pc,
        handler_pc,
        catch_type,
    })
}

fn parse_code_attribute<I>(
    bytes: &mut I,
    constant_pool: &ConstantPool,
) -> Result<CodeAttribute, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let max_stack = pop_u2_as_index(bytes)?;
    let max_locals = pop_u2_as_index(bytes)?;
    let code_length = pop_u4_as_index(bytes)?;
    let code = parse_n_opcodes(bytes, code_length)?;
    let exception_table_len = pop_u2_as_index(bytes)?;

    let mut exception_table = Vec::with_capacity(exception_table_len);

    for _ in 0..exception_table_len {
        let exception_info = parse_exception_table_info(bytes)?;
        exception_table.push(exception_info);
    }

    let attributes_count = pop_u2_as_index(bytes)?;

    let attributes = parse_n_attributes(bytes, attributes_count, constant_pool)?;

    Ok(CodeAttribute {
        max_stack,
        max_locals,
        code,
        exception_table,
        attributes,
    })
}

#[derive(Debug, Clone)]
pub struct LineNumberTableInfo {
    start_pc: usize,
    line_number: usize,
}

fn parse_line_number_table_info<I>(bytes: &mut I) -> Result<LineNumberTableInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let start_pc = pop_u2_as_index(bytes)?;
    let line_number = pop_u2_as_index(bytes)?;

    Ok(LineNumberTableInfo {
        start_pc,
        line_number,
    })
}

#[derive(Debug, Clone)]
pub struct LineNumberTableAttribute {
    infos: Vec<LineNumberTableInfo>,
}

pub fn parse_line_number_table_attribute<I>(
    bytes: &mut I,
) -> Result<LineNumberTableAttribute, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let infos_count = pop_u2_as_index(bytes)?;

    let mut infos = Vec::with_capacity(infos_count);

    for _ in 0..infos_count {
        let info = parse_line_number_table_info(bytes)?;
        infos.push(info);
    }

    Ok(LineNumberTableAttribute { infos })
}

#[derive(Debug, Clone)]
pub struct SourceFileAttribute {
    source_file_index: usize,
}

fn parse_source_file_attribute<I>(bytes: &mut I) -> Result<SourceFileAttribute, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let source_file_index = pop_u2_as_index(bytes)?;

    Ok(SourceFileAttribute { source_file_index })
}
