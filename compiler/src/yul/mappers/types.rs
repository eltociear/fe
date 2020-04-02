use crate::errors::CompileError;
use crate::yul::namespace::scopes::Scope;
use crate::yul::namespace::types;
use crate::yul::namespace::types::{FixedSize, Type};
use vyper_parser::ast as vyp;

pub fn type_desc(scope: Scope, typ: &vyp::TypeDesc) -> Result<Type, CompileError> {
    types::type_desc(&scope.module_scope().borrow().defs, typ)
}

pub fn type_desc_fixed_size(scope: Scope, typ: &vyp::TypeDesc) -> Result<FixedSize, CompileError> {
    types::type_desc_fixed_size(&scope.module_scope().borrow().defs, typ)
}
