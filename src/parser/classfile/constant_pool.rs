use crate::parser::utils::{pop1, pop2, pop4, pop8, pop_n, pop_u2_as_index, FileByte, ParseError};

/*
    cp_info {
        u1 tag;
        u1 info[];
    }

    Constant Type Value
    CONSTANT_Utf8 1
    NONE
    CONSTANT_Integer 3
    CONSTANT_Float 4
    CONSTANT_Long 5
    CONSTANT_Double 6
    CONSTANT_Class 7
    CONSTANT_String 8
    CONSTANT_Fieldref 9
    CONSTANT_Methodref 10
    CONSTANT_InterfaceMethodref 11
    CONSTANT_NameAndType 12
    NONE
    NONE
    CONSTANT_MethodHandle 15
    CONSTANT_MethodType 16
    NONE
    CONSTANT_InvokeDynamic 18

    CONSTANT_Class_info {
        u1 tag; = 7
        u2 name_index; -> CONSTANT_utf8_info
    }
    CONSTANT_Fieldref_info {
        u1 tag; = 9
        u2 class_index; -> CONSTANT_Class_info
        u2 name_and_type_index; -> CONSTANT_NameAndType_info
    }
    CONSTANT_Methodref_info {
        u1 tag; = 10
        u2 class_index; -> CONSTANT_Class_info
        u2 name_and_type_index; -> CONSTANT_NameAndType_info
    }
    CONSTANT_InterfaceMethodref_info {
        u1 tag; = 11
        u2 class_index; -> CONSTANT_Class_info
        u2 name_and_type_index; -> CONSTANT_NameAndType_info
    }
    CONSTANT_String_info {
        u1 tag; = 8
        u2 string_index; -> CONSTANT_utf8_info
    }
    CONSTANT_Integer_info {
        u1 tag; = 3
        u4 bytes; -> big endian integer
    }
    CONSTANT_Float_info {
        u1 tag; = 4
        u4 bytes; -> big endian float
    }
    CONSTANT_Long_info {
        u1 tag; = 5
        u4 high_bytes; -> high half
        u4 low_bytes; -> low half
    }
    CONSTANT_Double_info {
        u1 tag; = 6
        u4 high_bytes; -> high half
        u4 low_bytes; -> low half
    }
    CONSTANT_NameAndType_info {
        u1 tag; = 12
        u2 name_index; -> CONSTANT_Utf8_info
        u2 descriptor_index; -> CONSTANT_Utf8_info
    }
    CONSTANT_Utf8_info {
        u1 tag; = 1
        u2 length;
        u1 bytes[length];
    }
    CONSTANT_MethodType_info {
        u1 tag; = 16
        u2 descriptor_index; -> CONSTANT_Utf8_info
    }
    CONSTANT_InvokeDynamic_info {
        u1 tag; = 18
        u2 bootstrap_method_attr_index; -> bootstrap table
        u2 name_and_type_index; -> CONSTANT_NameAndType_info
    }



    CONSTANT_MethodHandle_info {
        u1 tag; = 15
        u1 reference_kind;
        u2 reference_index;
    }
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodHandleKind {
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}

impl TryFrom<u8> for MethodHandleKind {
    type Error = ParseError;

    fn try_from(kind: u8) -> Result<Self, Self::Error> {
        match kind {
            1 => Ok(MethodHandleKind::GetField),
            2 => Ok(MethodHandleKind::GetStatic),
            3 => Ok(MethodHandleKind::PutField),
            4 => Ok(MethodHandleKind::PutStatic),
            5 => Ok(MethodHandleKind::InvokeVirtual),
            6 => Ok(MethodHandleKind::InvokeStatic),
            7 => Ok(MethodHandleKind::InvokeSpecial),
            8 => Ok(MethodHandleKind::NewInvokeSpecial),
            9 => Ok(MethodHandleKind::InvokeInterface),
            _ => Err(ParseError::InvalidMethodHandleKind(kind)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantInfo {
    Class {
        name_index: usize,
    },
    FieldRef {
        class_index: usize,
        name_and_type_index: usize,
    },
    MethodRef {
        class_index: usize,
        name_and_type_index: usize,
    },
    InterfaceMethodRef {
        class_index: usize,
        name_and_type_index: usize,
    },
    String {
        string_index: usize,
    },
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType {
        name_index: usize,
        descriptor_index: usize,
    },
    Utf8(String),
    MethodHandle {
        reference_kind: MethodHandleKind,
        reference_index: usize,
    },
    MethodType {
        descriptor_index: usize,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: usize,
        name_and_type_index: usize,
    },
}

fn parse_utf8<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let len_bits = pop2(bytes)?;
    let len = u16::from_be_bytes(len_bits).into();
    let string_bytes = pop_n(bytes, len)?;

    let s = String::from_utf8(string_bytes).unwrap();
    Ok(ConstantInfo::Utf8(s))
}

fn parse_integer<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    pop4(bytes)
        .map(i32::from_be_bytes)
        .map(ConstantInfo::Integer)
}

fn parse_float<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    pop4(bytes).map(f32::from_be_bytes).map(ConstantInfo::Float)
}

fn parse_long<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    pop8(bytes).map(i64::from_be_bytes).map(ConstantInfo::Long)
}

fn parse_double<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    pop8(bytes)
        .map(f64::from_be_bytes)
        .map(ConstantInfo::Double)
}

fn parse_class<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let name_index = pop_u2_as_index(bytes)?;
    Ok(ConstantInfo::Class { name_index })
}

fn parse_string<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let string_index = pop_u2_as_index(bytes)?;
    Ok(ConstantInfo::String { string_index })
}

fn parse_field_ref<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let class_index = pop_u2_as_index(bytes)?;
    let name_and_type_index = pop_u2_as_index(bytes)?;
    Ok(ConstantInfo::FieldRef {
        class_index,
        name_and_type_index,
    })
}

fn parse_method_ref<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let class_index = pop_u2_as_index(bytes)?;
    let name_and_type_index = pop_u2_as_index(bytes)?;
    Ok(ConstantInfo::MethodRef {
        class_index,
        name_and_type_index,
    })
}

fn parse_interface_method_ref<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let class_index = pop_u2_as_index(bytes)?;
    let name_and_type_index = pop_u2_as_index(bytes)?;
    Ok(ConstantInfo::InterfaceMethodRef {
        class_index,
        name_and_type_index,
    })
}

fn parse_name_and_type<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let name_index = pop_u2_as_index(bytes)?;
    let descriptor_index = pop_u2_as_index(bytes)?;
    Ok(ConstantInfo::NameAndType {
        name_index,
        descriptor_index,
    })
}

fn parse_method_handle<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let reference_kind = pop1(bytes).and_then(u8::try_into)?;
    let reference_index = pop_u2_as_index(bytes)?;
    Ok(ConstantInfo::MethodHandle {
        reference_kind,
        reference_index,
    })
}

fn parse_method_type<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let descriptor_index = pop_u2_as_index(bytes)?;
    Ok(ConstantInfo::MethodType { descriptor_index })
}

fn parse_invoke_dynamic<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let bootstrap_method_attr_index = pop_u2_as_index(bytes)?;
    let name_and_type_index = pop_u2_as_index(bytes)?;
    Ok(ConstantInfo::InvokeDynamic {
        bootstrap_method_attr_index,
        name_and_type_index,
    })
}

fn parse_constant_info<I>(bytes: &mut I) -> Result<ConstantInfo, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let tag = pop1(bytes)?;
    match tag {
        1 => parse_utf8(bytes),
        // 2 =>
        3 => parse_integer(bytes),
        4 => parse_float(bytes),
        5 => parse_long(bytes),
        6 => parse_double(bytes),
        7 => parse_class(bytes),
        8 => parse_string(bytes),
        9 => parse_field_ref(bytes),
        10 => parse_method_ref(bytes),
        11 => parse_interface_method_ref(bytes),
        12 => parse_name_and_type(bytes),
        // 13 =>
        // 14 =>
        15 => parse_method_handle(bytes),
        16 => parse_method_type(bytes),
        // 17 =>
        18 => parse_invoke_dynamic(bytes),
        _ => Err(ParseError::InvalidTag(tag)),
    }
}

pub fn parse_constant_pool<I>(bytes: &mut I) -> Result<ConstantPool, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let info_count: usize = pop_u2_as_index(bytes)?;

    let mut infos = Vec::with_capacity(info_count);

    for _ in 1..info_count {
        let constant_info = parse_constant_info(bytes)?;
        infos.push(constant_info)
    }

    Ok(ConstantPool { infos })
}

#[derive(Debug, Clone)]
pub struct ConstantPool {
    infos: Vec<ConstantInfo>,
}

impl ConstantPool {
    pub fn get(&self, index: usize) -> Option<&ConstantInfo> {
        self.infos.get(index - 1)
    }

    pub fn size(&self) -> usize {
        self.infos.len() + 1
    }
}
