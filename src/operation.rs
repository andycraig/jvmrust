use crate::class::BytecodeClass;
use crate::class::ConstantPoolEnum;
use std::process::exit;
use log::info;

struct Envt {
  pc: usize,
  stack: Vec<String>,
  // frame
}

struct Op {
  pub opcode: u8,
  pub operands: Vec<u8>,
}

pub fn execute(class: BytecodeClass) {
  execute_code(class.get_main_method().get_code(), &class.constant_pool);
}

fn execute_code(code: &Vec<u8>, constant_pool: &Vec<ConstantPoolEnum>) {
  let mut envt = Envt {
    pc: 0,
    stack: vec![],
  };
  info!("At start, pc: {}", envt.pc);
  while envt.pc < code.len() {
    let op: Op = read_operation(code, &mut envt);
    info!("pc: {}", envt.pc);
    info!("opcode: {}", op.opcode);
    info!("Number of operands: {}", op.operands.len());
    execute_operation(op, constant_pool, &mut envt);
  }
}

fn execute_operation(op: Op, constant_pool: &Vec<ConstantPoolEnum>, envt: &mut Envt) {
  match op.opcode {
    // ldc, 1 operand
    18 => {
      info!("ldc");
      let index: u8 = op.operands[0];
      let constant_pool_index: &ConstantPoolEnum = &constant_pool[(index - 1) as usize];
      let constant_pool_index_string_index = match constant_pool_index {
        ConstantPoolEnum::ConstantStringItem(x) => x.string_index,
        _ => panic!("Something went wrong!"),
      };
      let constant_pool_index_utf8: &ConstantPoolEnum = 
        &constant_pool[(constant_pool_index_string_index - 1) as usize];
      // TODO envt.stack is not a Vec of Strings
      let name: String = match constant_pool_index_utf8 {
          ConstantPoolEnum::ConstantUtf8Item(x) => x.bytes.clone(),
          _ => panic!("Something went wrong!"),
      };
      info!("name: {}", name);
      envt.stack.push(name);
    },
    // getstatic, 2 operands
    178 => {
      info!("getstatic");
      let cp_index = as_u2(op.operands[0], op.operands[1]);
      info!("cp_index: {}", cp_index);
      let symbol_name_index: &ConstantPoolEnum = &constant_pool[(cp_index - 1) as usize];
      let symbol_name_index_class_index = match symbol_name_index {
        ConstantPoolEnum::ConstantFieldrefItem(x) => x.class_index,
        ConstantPoolEnum::ConstantMethodrefItem(x) => x.class_index,
        _ => panic!("Something went wrong!"),
      };
      let constant_pool_symbol_name_index_class_index: &ConstantPoolEnum = 
        &constant_pool[(symbol_name_index_class_index - 1) as usize];
      let cls_name_index = match constant_pool_symbol_name_index_class_index {
        ConstantPoolEnum::ConstantClassItem(x) => x.name_index,
        ConstantPoolEnum::ConstantNameAndTypeItem(x) => x.name_index,
        _ => panic!("Something went wrong!"),
      };
      let cls_name_index_utf8: &ConstantPoolEnum = &constant_pool[(cls_name_index - 1) as usize];
      let cls: String = match cls_name_index_utf8 {
          ConstantPoolEnum::ConstantUtf8Item(x) => x.bytes.clone(),
          _ => panic!("Something went wrong!"),
      };
      info!("cls: {}", cls);
      // TODO Second time this check has been performed
      let symbol_name_index_name_and_type_index = match symbol_name_index {
        ConstantPoolEnum::ConstantFieldrefItem(x) => x.name_and_type_index,
        _ => panic!("Something went wrong!"),
      };
      let constant_pool_symbol_name_index_name_and_type_index: &ConstantPoolEnum =
        &constant_pool[(symbol_name_index_name_and_type_index - 1) as usize];
      let field_name_index = match constant_pool_symbol_name_index_name_and_type_index {
        ConstantPoolEnum::ConstantClassItem(x) => x.name_index,
        ConstantPoolEnum::ConstantNameAndTypeItem(x) => x.name_index,
        _ => panic!("Something went wrong!"),
      };
      let field_name_index_utf8: &ConstantPoolEnum = &constant_pool[(field_name_index - 1) as usize];
      let field: String = match field_name_index_utf8 {
          ConstantPoolEnum::ConstantUtf8Item(x) => x.bytes.clone(),
          _ => panic!("Something went wrong!"),
      };
      info!("field: {}", field);
      let name = format!("{}.{}", cls, field);
      info!("name: {}", name);
      envt.stack.push(name);
      },
    // invokevirtual, 2 operands
    182 => {
      info!("invokevirtual");
      let index = as_u2(op.operands[0], op.operands[1]);
      let constant_pool_index: &ConstantPoolEnum = &constant_pool[(index - 1) as usize];
      let constant_pool_index_name_and_type_index: u16 = match constant_pool_index {
        ConstantPoolEnum::ConstantFieldrefItem(x) => x.name_and_type_index,
        ConstantPoolEnum::ConstantMethodrefItem(x) => x.name_and_type_index,
        _ => panic!("Something went wrong!"),
      }; 
      let callee: &ConstantPoolEnum =
        &constant_pool[(constant_pool_index_name_and_type_index - 1) as usize];
      let callee_name_index = match callee {
        ConstantPoolEnum::ConstantClassItem(x) => x.name_index,
        ConstantPoolEnum::ConstantNameAndTypeItem(x) => x.name_index,
        _ => panic!("Something went wrong!"),
      }; 
      let method_name_utf8: &ConstantPoolEnum =
        &constant_pool[(callee_name_index - 1) as usize];
      let method_name: String = match method_name_utf8 {
          ConstantPoolEnum::ConstantUtf8Item(x) => x.bytes.clone(),
          _ => panic!("Something went wrong!"),
      };
      if method_name != "println" {
        panic!("Not implemented: {}", method_name);
      }
      let args = envt.stack.pop();
      let object_name = envt.stack.pop();
      println!("{:?}", args.unwrap());
      info!("object_name: {:?}", object_name);
      },
    // return, 0 operands
    // TODO Replace with return implementation
    177 => exit(1),
    // iconst_3, 0 operands
    6 => panic!("iconst_3 not implemented"),
    // istore, 1 operand
    54 => panic!("istore not implemented"),
    // iload, 1 operand
    21 =>  panic!("iload not implemented"),
    // ireturn, 0 operands
    172 =>  panic!("ireturn not implemented"),
    _ => panic!("Operation not implemented"),
  }
}

fn read_operation(code: &Vec<u8>, envt: &mut Envt) -> Op {
  let pc = envt.pc;
  let opcode = code[pc];
  let arity = match opcode {
    18 => 1,
    178 => 2,
    182 => 2,
    177 => 0,
    _ => panic!("Operation not implemented"),
  };
  // TODO Probably a better way of slicing
  let operands = (0..arity)
    .map(|x| code[pc + 1 + x])
    .collect();
  envt.pc = pc + 1 + arity;
  Op {
    opcode: opcode,
    operands: operands,
  }
}

fn as_u2(byte1: u8, byte2: u8) -> u16 {
  let byte1_u16: u16 = byte1.into();
  let byte2_u16: u16 = byte2.into();
  let byte1_shifted: u16 = byte1_u16 * (2^8);
  byte1_shifted + byte2_u16
}
