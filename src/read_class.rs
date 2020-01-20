use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io;
use std::string::String;
use log::info;

use crate::class::BytecodeClass;
use crate::class::ConstantPoolEnum;
use crate::class::ConstantUtf8Class;
use crate::class::ConstantClassClass;
use crate::class::ConstantStringClass;
use crate::class::ConstantFieldrefClass;
use crate::class::ConstantMethodrefClass;
use crate::class::ConstantNameAndTypeClass;
use crate::class::MethodInfo;
use crate::class::AttributeEnum;
use crate::class::CodeClass;
use crate::class::LineNumberTableClass;
use crate::class::StackMapTableClass;
use crate::class::SourceFileClass;
use crate::class::LineNumberTableElem;
use crate::class::ExceptionTable;

pub fn parse(path: String) -> BytecodeClass {
    let file = File::open(path).unwrap();
    let mut con = io::BufReader::new(file);
    let magic = con.read_u32::<BigEndian>().unwrap();
    let minor_version = con.read_u16::<BigEndian>().unwrap();
    let major_version = con.read_u16::<BigEndian>().unwrap();
    let constant_pool_count = con.read_u16::<BigEndian>().unwrap();
    info!("Magic: {}", magic);
    info!("Minor version: {}", minor_version);
    info!("Major version: {}", major_version);
    info!("Constant pool count: {}", constant_pool_count);
    let constant_pool: Vec<ConstantPoolEnum> = (0..(constant_pool_count - 1))
        .map(|_x| read_cp_info(&mut con))
        .collect();
    let access_flags = con.read_u16::<BigEndian>().unwrap();
    let this_class = con.read_u16::<BigEndian>().unwrap();
    info!("Access flags: {}", access_flags);
    // TODO Deal better with constant pool being 1-indexed
    let this_class_class: &ConstantPoolEnum = &constant_pool[(this_class - 1) as usize];
    info!("This class index (should be 5): {}", this_class);
    let this_class_utf8 = match this_class_class {
        ConstantPoolEnum::ConstantClassItem(x) => 
            &constant_pool[(x.name_index - 1) as usize],
        _ => panic!("Something went wrong!"),
    };
    let this_class_name = match this_class_utf8 {
        ConstantPoolEnum::ConstantUtf8Item(x) => x.bytes.clone(),
        _ => panic!("Something went wrong!"),
    };
    info!("This class name: {}", this_class_name);
    let super_class = con.read_u16::<BigEndian>().unwrap();
    info!("Super class index (should be 6): {}", super_class);
    let super_class_class: &ConstantPoolEnum = &constant_pool[(super_class - 1) as usize];
    let super_class_utf8 = match super_class_class {
        ConstantPoolEnum::ConstantClassItem(x) => 
            &constant_pool[(x.name_index - 1) as usize],
        _ => panic!("Something went wrong!"),
    };
    let super_class_name = match super_class_utf8 {
        ConstantPoolEnum::ConstantUtf8Item(x) => x.bytes.clone(),
        _ => panic!("Something went wrong!"),
    };
    info!("Super class name: {}", super_class_name);
    
    let interfaces_count = con.read_u16::<BigEndian>().unwrap();
    // TODO
    if interfaces_count > 0 {
        panic!("Interfaces not implemented")
    }
    info!("Read interfaces");
    let fields_count = con.read_u16::<BigEndian>().unwrap();
    // TODO
    if fields_count > 0 {
        panic!("Fields not implemented")
    }
    info!("Read fields");
    let methods_count = con.read_u16::<BigEndian>().unwrap();
    info!("Methods: {}", methods_count);
    let methods: Vec<MethodInfo> = (0..methods_count)
        .map(|_x| read_method_info(&mut con, &constant_pool))
        .collect();
    info!("Read methods");
    let attributes_count = con.read_u16::<BigEndian>().unwrap();
    let attributes: Vec<AttributeEnum> = (0..attributes_count)
        .map(|_x| read_attribute(&mut con, &constant_pool))
        .collect();

    return BytecodeClass {
        magic: magic,
        minor_version: minor_version,
        major_version: major_version,
        constant_pool: constant_pool,
        access_flags: access_flags,
        this_class: this_class,
        this_class_name: this_class_name,
        super_class: super_class,
        super_class_name: super_class_name,
        methods: methods,
        attributes: attributes,
    };
}

fn read_cp_info(con: &mut io::BufReader<File>) -> ConstantPoolEnum {
    // Was con.read_u8::<BigEndian>().unwrap() 
    let tag: u8 = con.read_u8().unwrap();
    info!("Tag: {}", tag);
    let info = match tag {
        1 => {
            let n_bytes = con.read_u16::<BigEndian>().unwrap();
            let byte_slice_vec: Vec<u8> = (0..n_bytes)
                .map(|_x| con.read_u8().unwrap())
                .collect();
            let bytes = String::from_utf8(byte_slice_vec).unwrap();
            return ConstantPoolEnum::ConstantUtf8Item(
                ConstantUtf8Class { length: n_bytes, bytes: bytes });
        },
        7 => ConstantPoolEnum::ConstantClassItem(
            ConstantClassClass { name_index: con.read_u16::<BigEndian>().unwrap() }),
        8 => ConstantPoolEnum::ConstantStringItem(
            ConstantStringClass { string_index: con.read_u16::<BigEndian>().unwrap() }),
        9 => ConstantPoolEnum::ConstantFieldrefItem(
            ConstantFieldrefClass { 
                class_index: con.read_u16::<BigEndian>().unwrap(),
                name_and_type_index: con.read_u16::<BigEndian>().unwrap() }),
        10 => ConstantPoolEnum::ConstantMethodrefItem(
            ConstantMethodrefClass {
                class_index: con.read_u16::<BigEndian>().unwrap(),
                name_and_type_index: con.read_u16::<BigEndian>().unwrap() }),
        12 => ConstantPoolEnum::ConstantNameAndTypeItem(
            ConstantNameAndTypeClass {
                name_index: con.read_u16::<BigEndian>().unwrap(),
                descriptor_index: con.read_u16::<BigEndian>().unwrap() }),
        _ => panic!("Something went wrong!"),
    };
    return info;
}

fn read_method_info(con: &mut io::BufReader<File>, constant_pool: &Vec<ConstantPoolEnum>) -> MethodInfo {
    let access_flags = con.read_u16::<BigEndian>().unwrap();
    info!("Read access_flag");
    let name_index = con.read_u16::<BigEndian>().unwrap();
    info!("Read name_index");
    let name_index_utf8: &ConstantPoolEnum = &constant_pool[(name_index - 1) as usize];
    let name = match name_index_utf8 {
        ConstantPoolEnum::ConstantUtf8Item(x) => x.bytes.clone(),
        _ => panic!("Something went wrong!"),
    };
    info!("Read name {}", name);
    let descriptor_index = con.read_u16::<BigEndian>().unwrap();
        let descriptor_index_utf8: &ConstantPoolEnum = &constant_pool[(descriptor_index - 1) as usize];
    let descriptor = match descriptor_index_utf8 {
        ConstantPoolEnum::ConstantUtf8Item(x) => x.bytes.clone(),
        _ => panic!("Something went wrong!"),
    };
    info!("Read descriptor {}", descriptor);
    let attributes_count = con.read_u16::<BigEndian>().unwrap();
    info!("Read attributes_count {}", attributes_count);
    let attributes: Vec<AttributeEnum> = (0..attributes_count)
        .map(|_x| read_attribute(con, &constant_pool))
        .collect();
    info!("Read attributes");
        MethodInfo {
        access_flags: access_flags,
        name_index: name_index,
        name: name,
        descriptor_index: descriptor_index,
        descriptor: descriptor,
        attributes_count: attributes_count,
        attributes: attributes
    }
}

fn read_line_number_table(con: &mut io::BufReader<File>) -> LineNumberTableElem {
    LineNumberTableElem {
        start_pc: con.read_u16::<BigEndian>().unwrap(),
        line_number: con.read_u16::<BigEndian>().unwrap(),
    }
}

fn read_attribute(con: &mut io::BufReader<File>, constant_pool: &Vec<ConstantPoolEnum>) -> AttributeEnum {
    let attribute_name_index = con.read_u16::<BigEndian>().unwrap();
    info!("Read attribute_name_index {}", attribute_name_index);
    let attribute_length = con.read_u32::<BigEndian>().unwrap();
    info!("Read attribute_length {}", attribute_length);
    // TODO This pattern appears a lot - refactor?
    let attribute_name_index_utf8: &ConstantPoolEnum = &constant_pool[(attribute_name_index - 1) as usize];
    let attribute_name: String = match attribute_name_index_utf8 {
        ConstantPoolEnum::ConstantUtf8Item(x) => x.bytes.clone(),
        _ => panic!("Something went wrong!"),
    };
    info!("Read attribute_name {}", attribute_name);
    let attribute = match attribute_name.as_ref() {
        "Code" => {
            let max_stack = con.read_u16::<BigEndian>().unwrap();
            info!("Read max_stack: {}", max_stack);
            let max_locals = con.read_u16::<BigEndian>().unwrap();
            info!("Read max_locals: {}", max_locals);
            let code_length = con.read_u32::<BigEndian>().unwrap();
            info!("Read code_length: {}", code_length);
            let code: Vec<u8> = (0..code_length)
                .map(|_x| con.read_u8().unwrap())
                .collect();
            info!("Read byte_slice_vec");
            let exception_table_length = con.read_u16::<BigEndian>().unwrap();
            // TODO Handle exception table of non-0 length
            if exception_table_length > 0 {
                panic!("Exception table of length 1+ not implemented");
            }
            let exception_table = ExceptionTable {};
            let attributes_count = con.read_u16::<BigEndian>().unwrap();
            let attributes: Vec<AttributeEnum> = (0..attributes_count)
                .map(|_x| read_attribute(con, &constant_pool))
                .collect();
            info!("About to create CodeItem");
            AttributeEnum::CodeItem(
                CodeClass {
                    attribute_name_index: attribute_name_index,
                    attribute_name: attribute_name,
                    attribute_length: attribute_length,
                    max_stack: max_stack,
                    max_locals: max_locals,
                    code_length: code_length,
                    code: code,
                    exception_table_length: exception_table_length,
                    exception_table: exception_table,
                    attributes_count: attributes_count,
                    attributes: attributes
                })
        },
        "LineNumberTable" => {
            let line_number_table_length = con.read_u16::<BigEndian>().unwrap();
            let line_number_table: Vec<LineNumberTableElem> = (0..line_number_table_length)
                .map(|_x| read_line_number_table(con))
                .collect();
            AttributeEnum::LineNumberTableItem(
                LineNumberTableClass {
                    attribute_name_index: attribute_name_index,
                    attribute_name: attribute_name,
                    attribute_length: attribute_length,
                    line_number_table_length: line_number_table_length,
                    line_number_table: line_number_table
                })
        },
        "StackMapTable" => {
            // TODO Parse stack_map_frame
            let _: Vec<u16> = (0..attribute_length)
                .map(|_x| con.read_u16::<BigEndian>().unwrap())
                .collect();
            AttributeEnum::StackMapTableItem(
                StackMapTableClass {
                    attribute_name_index: attribute_name_index,
                    attribute_name: attribute_name,
                    attribute_length: attribute_length
                })
        },
        "SourceFile" => 
            AttributeEnum::SourceFileItem(
                SourceFileClass {
                    attribute_name_index: attribute_name_index,
                    attribute_name: attribute_name,
                    sourcefile_index: con.read_u16::<BigEndian>().unwrap()
            }),
        _ => panic!("Unknown attribute_name"),
    };
    attribute
}
