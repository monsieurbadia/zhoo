// use zhoo_parser::tree::ty::{Ty, TyKind};
// use zhoo_parser::tree::PBox;

use codegen::ir::GlobalValue;
use cranelift::prelude::*;
use cranelift_module::{DataContext, FuncId, Linkage, Module};
use cranelift_object::ObjectModule;
use fnv::FnvHashMap;

/// an instance of a compile function
pub struct CompiledFunction {
  pub id: FuncId,
  pub defined: bool,
  pub input_len: usize,
}

impl CompiledFunction {
  /// create an instance of a compile function
  #[inline]
  pub const fn new(id: FuncId, defined: bool, input_len: usize) -> Self {
    Self {
      id,
      defined,
      input_len,
    }
  }
}

/// an instance of a variable builder
#[derive(Default)]
pub struct VariableBuilder {
  pub index: u32,
}

impl VariableBuilder {
  /// create a variable
  pub fn create_variable(
    &mut self,
    builder: &mut FunctionBuilder,
    value: Value,
    ty: types::Type,
  ) -> Variable {
    let variable = Variable::with_u32(self.index);

    builder.declare_var(variable, ty);
    builder.def_var(variable, value);

    self.index += 1;

    variable
  }
}

/// an instance of a data builder
#[derive(Default)]
pub struct DataBuilder {
  pub index: u32,
}

impl DataBuilder {
  /// create a data
  pub fn create_data(
    &mut self,
    builder: &mut FunctionBuilder,
    module: &mut ObjectModule,
    globals: &mut FnvHashMap<String, GlobalValue>,
    data: &String,
  ) -> Value {
    let data_id = match globals.get(data) {
      Some(data_id) => *data_id,
      None => {
        let data_name = format!("_data{}", data);

        let data_id = module
          .declare_data(&data_name, Linkage::Local, false, false)
          .unwrap();

        let mut data_context = DataContext::new();

        data_context.define(data.as_bytes().to_vec().into_boxed_slice());
        module.define_data(data_id, &data_context).unwrap();

        let data_id = module.declare_data_in_func(data_id, builder.func);

        globals.insert(data.to_string(), data_id);

        data_id
      }
    };

    builder.ins().global_value(types::I64, data_id)
  }
}

// impl From<PBox<Ty>> for types::Type {
//   fn from(ty: PBox<Ty>) -> Self {
//     match ty.kind {
//       TyKind::Bool => types::B1,
//       TyKind::Int => types::I64,
//       TyKind::Real => types::F64,
//       TyKind::Void => types::I64,
//       _ => panic!("from ty to types"),
//     }
//   }
// }
