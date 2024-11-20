use wasmer_types::TableType;

use crate::{
    v8::{
        bindings::{self, *},
        utils::convert::{IntoCApiType, IntoCApiValue, IntoWasmerType, IntoWasmerValue},
        vm::VMTable,
    },
    vm::{VMExtern, VMExternTable},
    AsStoreMut, AsStoreRef, RuntimeError, RuntimeTable, Value,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub(crate) handle: VMTable,
}

unsafe impl Send for Table {}
unsafe impl Sync for Table {}

impl Table {
    pub(crate) fn type_to_v8(ty: TableType) -> *mut wasm_tabletype_t {
        let valtype = unsafe { wasm_valtype_new(ty.ty.into_ct()) };

        let limits = Box::into_raw(Box::new(wasm_limits_t {
            min: ty.minimum,
            max: match ty.maximum {
                Some(v) => v,
                None => 0,
            },
        }));

        unsafe { wasm_tabletype_new(valtype, limits) }
    }

    pub fn new(
        store: &mut impl AsStoreMut,
        ty: TableType,
        init: Value,
    ) -> Result<Self, RuntimeError> {
        let store_mut = store.as_store_mut();
        let engine = store_mut.engine();

        let wasm_tablety = Self::type_to_v8(ty);
        let init: wasm_val_t = init.into_cv();

        Ok(Self {
            handle: unsafe {
                wasm_table_new(
                    store_mut.inner.store.as_v8().inner,
                    wasm_tablety,
                    init.of.ref_,
                )
            },
        })
    }

    pub fn to_vm_extern(&self) -> VMExtern {
        VMExtern::V8(unsafe { wasm_table_as_extern(self.handle) })
    }

    pub fn ty(&self, _store: &impl AsStoreRef) -> TableType {
        let table_type: *mut wasm_tabletype_t = unsafe { wasm_table_type(self.handle) };
        let table_limits = unsafe { wasm_tabletype_limits(table_type) };
        let table_type = unsafe { wasm_tabletype_element(table_type) };

        TableType {
            ty: table_type.into_wt(),
            minimum: unsafe { (*table_limits).min },
            maximum: unsafe {
                if (*table_limits).max == 0 {
                    None
                } else {
                    Some((*table_limits).max)
                }
            },
        }
    }

    pub fn get(&self, store: &mut impl AsStoreMut, index: u32) -> Option<Value> {
        unsafe {
            let ref_ = wasm_table_get(self.handle, index);

            if ref_.is_null() {
                return None;
            }

            let kind = match self.ty(store).ty {
                wasmer_types::Type::ExternRef => wasm_valkind_enum_WASM_ANYREF,
                wasmer_types::Type::FuncRef => wasm_valkind_enum_WASM_FUNCREF,
                ty => panic!("unsupported table type: {ty:?}"),
            } as u8;

            let value = wasm_val_t {
                kind,
                of: bindings::wasm_val_t__bindgen_ty_1 { ref_ },
            };

            Some(value.into_wv())
        }
    }

    pub fn set(
        &self,
        store: &mut impl AsStoreMut,
        index: u32,
        val: Value,
    ) -> Result<(), RuntimeError> {
        unsafe {
            let init = match val {
                Value::ExternRef(None) | Value::FuncRef(None) => std::ptr::null_mut(),
                Value::FuncRef(Some(ref r)) => wasm_func_as_ref(r.as_v8().handle),
                _ => {
                    return Err(RuntimeError::new(format!(
                        "Could not grow table due to unsupported init value type: {val:?} "
                    )))
                }
            };

            if !wasm_table_set(self.handle, index, init) {
                return Err(RuntimeError::new(format!(
                    "Could not set value {val:?} table at index {index}"
                )));
            }

            Ok(())
        }
    }

    pub fn size(&self, store: &impl AsStoreRef) -> u32 {
        unsafe { wasm_table_size(self.handle) }
    }

    pub fn grow(
        &self,
        store: &mut impl AsStoreMut,
        delta: u32,
        init: Value,
    ) -> Result<u32, RuntimeError> {
        unsafe {
            let size = wasm_table_size(self.handle);
            let init = match init {
                Value::ExternRef(None) | Value::FuncRef(None) => std::ptr::null_mut(),
                Value::FuncRef(Some(r)) => wasm_func_as_ref(r.as_v8().handle),
                _ => {
                    return Err(RuntimeError::new(format!(
                        "Could not grow table due to unsupported init value type: {init:?} "
                    )))
                }
            };
            if !wasm_table_grow(self.handle, delta, init) {
                return Err(RuntimeError::new("Could not grow table"));
            }

            Ok(size)
        }
    }

    pub fn copy(
        _store: &mut impl AsStoreMut,
        _dst_table: &Self,
        _dst_index: u32,
        _src_table: &Self,
        _src_index: u32,
        _len: u32,
    ) -> Result<(), RuntimeError> {
        unimplemented!("Copying tables is currently not implemented!")
    }

    pub(crate) fn from_vm_extern(_store: &mut impl AsStoreMut, vm_extern: VMExternTable) -> Self {
        Self {
            handle: vm_extern.into_v8(),
        }
    }

    pub fn is_from_store(&self, _store: &impl AsStoreRef) -> bool {
        true
    }
}

impl crate::Table {
    /// Consume [`self`] into [`crate::rt::v8::table::Table`].
    pub fn into_v8(self) -> crate::rt::v8::table::Table {
        match self.0 {
            RuntimeTable::V8(s) => s,
            _ => panic!("Not a `v8` table!"),
        }
    }

    /// Convert a reference to [`self`] into a reference [`crate::rt::v8::table::Table`].
    pub fn as_v8(&self) -> &crate::rt::v8::table::Table {
        match self.0 {
            RuntimeTable::V8(ref s) => s,
            _ => panic!("Not a `v8` table!"),
        }
    }

    /// Convert a mutable reference to [`self`] into a mutable reference [`crate::rt::v8::table::Table`].
    pub fn as_v8_mut(&mut self) -> &mut crate::rt::v8::table::Table {
        match self.0 {
            RuntimeTable::V8(ref mut s) => s,
            _ => panic!("Not a `v8` table!"),
        }
    }
}

impl crate::RuntimeTable {
    /// Consume [`self`] into [`crate::rt::v8::table::Table`].
    pub fn into_v8(self) -> crate::rt::v8::table::Table {
        match self {
            Self::V8(s) => s,
            _ => panic!("Not a `v8` table!"),
        }
    }

    /// Convert a reference to [`self`] into a reference [`crate::rt::v8::table::Table`].
    pub fn as_v8(&self) -> &crate::rt::v8::table::Table {
        match self {
            Self::V8(ref s) => s,
            _ => panic!("Not a `v8` table!"),
        }
    }

    /// Convert a mutable reference to [`self`] into a mutable reference [`crate::rt::v8::table::Table`].
    pub fn as_v8_mut(&mut self) -> &mut crate::rt::v8::table::Table {
        match self {
            Self::V8(ref mut s) => s,
            _ => panic!("Not a `v8` table!"),
        }
    }
}