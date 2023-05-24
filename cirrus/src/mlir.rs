// Copyright (c) 2016-2021 Fabian Schuiki
// Copyright (c) 2022-2023 Kamyar Mohajerani

use cirrus_sys::*;
use crate::*;
use std::marker::PhantomData;

decl_wrapper!(Context, MlirContext);
impl IntoOwned for Context {
    fn destroy(&mut self) { 
        unsafe { mlirContextDestroy(self.raw()) }
    }
}
impl Context { 
    pub fn new() -> Owned<Self> { 
        Owned(Self(unsafe { mlirContextCreate() }))
    }
    pub fn load_dialect(&self, handle: &DialectHandle) {
        unsafe {
            mlirDialectHandleRegisterDialect(handle.raw(), self.raw());

            // FIXME: Do we need to return an [MlirDialect]? 
            mlirDialectHandleLoadDialect(handle.raw(), self.raw());
        }
    }
}

decl_wrapper!(Module, MlirModule);
impl IntoOwned for Module { 
    fn destroy(&mut self) {
        unsafe { mlirModuleDestroy(self.raw()) }
    }
}
impl Module {
    pub fn parse(ctx: &Context, s: &str) -> Owned<Self> {
        let sr = StringRef::from_str(s);
        unsafe {
            Owned(Self(mlirModuleCreateParse(ctx.raw(), sr.raw())))
        }
    }

    pub fn body(&self) -> Block {
        Block::try_from_raw(
            unsafe { mlirModuleGetBody(self.raw()) }
        ).unwrap()
    }
    pub fn op(&self) -> Option<Operation> {
        Operation::try_from_raw(
            unsafe { cirrus_sys::mlirModuleGetOperation(self.raw()) }
        )
    }
}

decl_wrapper!(DialectHandle, MlirDialectHandle);
impl DialectHandle {
    pub fn firrtl() -> Self { 
        Self(unsafe { mlirGetDialectHandle__firrtl__() })
    }
}

decl_wrapper!(Operation, MlirOperation);
impl Operation {
    pub fn dump(&self) {
        unsafe { mlirOperationDump(self.raw()) }
    }

    pub fn next(&self) -> Option<Self> {
        Self::try_from_raw(
            unsafe { mlirOperationGetNextInBlock(self.raw()) }
        )
    }
    pub fn first_region(&self) -> Option<Region> {
        Region::try_from_raw(
            unsafe { mlirOperationGetFirstRegion(self.raw()) }
        )
    }
    pub fn name(&self) -> Identifier {
        Identifier::try_from_raw(
            unsafe { mlirOperationGetName(self.raw()) }
        ).unwrap()
    }

    pub fn num_operands(&self) -> usize { 
        unsafe { 
            mlirOperationGetNumOperands(self.raw()).try_into().unwrap()
        }
    }
    pub fn operand_at(&self, n: usize) -> Option<Value> {
        Value::try_from_raw(
            unsafe { 
                mlirOperationGetOperand(self.raw(), n.try_into().unwrap())
            }
        )
    }

    pub fn result_at(&self, n: usize) -> Option<Value> {
        Value::try_from_raw(
            unsafe { 
                mlirOperationGetResult(self.raw(), n.try_into().unwrap())
            }
        )
    }

    pub fn results(&self) -> Option<Vec<Value>> {
        let num_results = self.num_results();
        if num_results == 0 {
            None
        } else { 
            let mut res = Vec::new();
            for idx in 0..num_results {
                res.push(self.result_at(idx).unwrap());
            }
            Some(res)
        }
    }

    pub fn operands(&self) -> Option<Vec<Value>> {
        let num_operands = self.num_operands();
        if num_operands == 0 {
            None
        } else { 
            let mut res = Vec::new();
            for idx in 0..num_operands {
                res.push(self.operand_at(idx).unwrap());
            }
            Some(res)
        }
    }

    pub fn num_results(&self) -> usize { 
        unsafe { 
            mlirOperationGetNumResults(self.raw()).try_into().unwrap()
        }
    }

    pub fn num_attributes(&self) -> usize { 
        unsafe { 
            mlirOperationGetNumAttributes(self.raw()).try_into().unwrap()
        }
    }

    pub fn attribute_at(&self, n: usize) -> Option<NamedAttribute> {
        NamedAttribute::try_from_raw(
            unsafe { 
                mlirOperationGetAttribute(self.raw(), n.try_into().unwrap())
            }
        )

    }
    pub fn attribute(&self, name: &str) -> Option<Attribute> {
        Attribute::try_from_raw(
            unsafe { 
                mlirOperationGetAttributeByName(self.raw(), 
                    StringRef::from_str(name).raw()
                )
            }
        )
    }
}

decl_wrapper!(Attribute, MlirAttribute);
impl Attribute {
    pub fn dump(&self) {
        unsafe { mlirAttributeDump(self.raw()) }
    }
}

pub struct NamedAttribute {
    _raw: MlirNamedAttribute,
    pub name: Identifier,
    pub attribute: Attribute, 
}
impl Wrapper for NamedAttribute {
    type Inner = MlirNamedAttribute;
    fn raw(&self) -> Self::Inner {
        self._raw
    }
    fn raw_ref(&self) -> &Self::Inner {
        &self._raw
    }
    fn raw_mut(&mut self) -> &mut Self::Inner {
        &mut self._raw
    }
    fn try_from_raw(raw: Self::Inner) -> Option<Self> {
        let name = Identifier::try_from_raw(raw.name);
        let attribute = Attribute::try_from_raw(raw.attribute);
        if name.is_none() || attribute.is_none() {
            return None;
        }
        Some(Self { 
            _raw: raw, 
            name: name.unwrap(), 
            attribute: attribute.unwrap() 
        })
    }

}

//decl_wrapper!(NamedAttribute, MlirNamedAttribute);
impl BindingType for MlirNamedAttribute {
    fn is_null(&self) -> bool { 
        self.name.is_null() || self.attribute.is_null()
    }
}
impl NamedAttribute {
    pub fn dump(&self) {
        unsafe { mlirAttributeDump(self.raw().attribute) }
    }
}


decl_wrapper!(Value, MlirValue);
impl Value {
    pub fn dump(&self) {
        unsafe { mlirValueDump(self.raw()) }
    }
    pub fn get_type(&self) -> Type { 
        Type::try_from_raw(
            unsafe { mlirValueGetType(self.raw()) }
        ).unwrap()
    }
}

decl_wrapper!(Region, MlirRegion);
impl Region {
    pub fn first_block(&self) -> Option<Block> {
        Block::try_from_raw(
            unsafe { mlirRegionGetFirstBlock(self.raw()) }
        )
    }
    pub fn next(&self) -> Option<Self> {
        Self::try_from_raw(
            unsafe { mlirRegionGetNextInOperation(self.raw()) }
        )
    }
}

decl_wrapper!(Block, MlirBlock);
impl Block {
    pub fn first_op(&self) -> Operation {
        Operation::try_from_raw(
            unsafe { 
                mlirBlockGetFirstOperation(self.raw())
            }
        ).unwrap()
    }
}

decl_wrapper!(Type, MlirType);
impl Type { 
    pub fn parse(ctx: &Context, type_name: &str) -> Option<Self> {
        Self::try_from_raw(
            unsafe {
                mlirTypeParseGet(ctx.raw(), 
                    StringRef::from_str(type_name).raw()
                )
            }
        )
    }
    pub fn dump(&self) {
        unsafe { mlirTypeDump(self.raw()) }
    }
}

decl_wrapper!(Identifier, MlirIdentifier);
impl Identifier {
    pub fn to_string_ref(&self) -> StringRef { 
        StringRef::try_from_raw(
            unsafe { 
                mlirIdentifierStr(self.raw())
            }
        ).unwrap()
    }
}

pub struct StringRef<'a>(MlirStringRef, PhantomData<&'a MlirStringRef>);
impl <'a> Wrapper for StringRef<'a> {
    type Inner = MlirStringRef;
    fn raw(&self) -> Self::Inner { self.0 }
    fn raw_ref(&self) -> &Self::Inner { &self.0 }
    fn raw_mut(&mut self) -> &mut Self::Inner { &mut self.0 }
    fn try_from_raw(raw: Self::Inner) -> Option<Self> {
        (!raw.is_null()).then_some(Self(raw, PhantomData))
    }
}
impl <'a> StringRef<'a> {
    pub fn as_bytes(&'a self) -> &'a [u8] {
        unsafe { 
            std::slice::from_raw_parts(self.0.data as _, self.0.length)
        }
    }
    pub fn from_str(s: &'a str) -> Self {
        StringRef(
            MlirStringRef { data: s.as_ptr() as _, length: s.len(), }, 
            PhantomData
        )
    }
    pub fn as_str(&'a self) -> &'a str {
        std::str::from_utf8(self.as_bytes()).unwrap()
    }
}



