


macro_rules! impl_binding_type {
    ($name:ident, $field:ident) => {
        impl BindingType for $name { 
            fn is_null(&self) -> bool {
                (self.$field as *const std::os::raw::c_void).is_null()
            }
        }
    }
}

macro_rules! decl_wrapper {
    ($name:ident, $inner:ident) => {
        pub struct $name(cirrus_sys::$inner);
        impl Wrapper for $name {
            type Inner = $inner;
            fn raw(&self) -> Self::Inner { self.0 }
            fn raw_ref(&self) -> &Self::Inner { &self.0 }
            fn raw_mut(&mut self) -> &mut Self::Inner { &mut self.0 }
            fn try_from_raw(raw: Self::Inner) -> Option<Self> {
                (!raw.is_null()).then_some(Self(raw))
            }
        }
    }
}


