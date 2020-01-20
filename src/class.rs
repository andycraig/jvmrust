pub struct BytecodeClass {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<ConstantPoolEnum>,
    pub access_flags: u16,
    pub this_class: u16,
    pub this_class_name: String,
    pub super_class: u16,
    pub super_class_name: String,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeEnum>,
}

impl BytecodeClass {
    pub fn get_main_method(&self) -> &MethodInfo {
        let mut ii = 0;
        for i in 0..self.methods.len() {
            if self.methods[i].name == "main" {
                ii = i;
                break;
            }
        }
        &self.methods[ii]
    }
}

#[derive(Clone)]
pub enum ConstantPoolEnum {
    ConstantClassItem(ConstantClassClass),
    ConstantStringItem(ConstantStringClass),
    ConstantUtf8Item(ConstantUtf8Class),
    ConstantFieldrefItem(ConstantFieldrefClass),
    ConstantMethodrefItem(ConstantMethodrefClass),
    ConstantNameAndTypeItem(ConstantNameAndTypeClass),
}

#[derive(Clone)]
pub struct ConstantUtf8Class {
    pub length: u16,
    pub bytes: String, 
}

#[derive(Copy, Clone)]
pub struct ConstantClassClass {
    pub name_index: u16,
}

#[derive(Copy, Clone)]
pub struct ConstantStringClass {
    pub string_index: u16,
}

#[derive(Copy, Clone)]
pub struct ConstantFieldrefClass { 
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Copy, Clone)]
pub struct ConstantMethodrefClass {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Copy, Clone)]
pub struct ConstantNameAndTypeClass {
    pub name_index: u16,
    pub descriptor_index: u16,
}

pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub name: String,
    pub descriptor_index: u16,
    pub descriptor: String,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeEnum>,
}

impl MethodInfo {
    pub fn get_code(&self) -> &Vec<u8> {
        let mut ii = 0;
        for i in 0..self.attributes.len() {
            match &self.attributes[i] {
                AttributeEnum::CodeItem(_code_class) => {
                    ii = i;
                    break;
                }
                _ => {}, 
            };
        }
        match &self.attributes[ii] {
            AttributeEnum::CodeItem(code_class) => {
                &code_class.code
            },
            _ => panic!("Something unexpected happened"), 
        }
    }
}

pub enum AttributeEnum {
    CodeItem(CodeClass),
    LineNumberTableItem(LineNumberTableClass),
    StackMapTableItem(StackMapTableClass),
    SourceFileItem(SourceFileClass),
}

pub struct ExceptionTable {}

pub struct CodeClass {
    pub attribute_name_index: u16,
    pub attribute_name: String,
    pub attribute_length: u32,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub exception_table_length: u16,
    pub exception_table: ExceptionTable,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeEnum>,
}

pub struct LineNumberTableElem {
    pub start_pc: u16,
    pub line_number: u16,
}

pub struct LineNumberTableClass {
    pub attribute_name_index: u16,
    pub attribute_name: String,
    pub attribute_length: u32,
    pub line_number_table_length: u16,
    pub line_number_table: Vec<LineNumberTableElem>,
}

pub struct StackMapTableClass {
    pub attribute_name_index: u16,
    pub attribute_name: String,
    pub attribute_length: u32,
}

pub struct SourceFileClass {
    pub attribute_name_index: u16,
    pub attribute_name: String,
    pub sourcefile_index: u16,
}
