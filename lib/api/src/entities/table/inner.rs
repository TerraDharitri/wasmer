use wasmer_types::TableType;

use crate::{
    error::RuntimeError,
    store::RuntimeStore,
    vm::{VMExtern, VMExternTable},
    AsStoreMut, AsStoreRef, ExportError, Exportable, Extern, StoreMut, StoreRef, Value,
};
/// A WebAssembly `table` instance.
///
/// The `Table` struct is an array-like structure representing a WebAssembly Table,
/// which stores function references.
///
/// A table created by the host or in WebAssembly code will be accessible and
/// mutable from both host and WebAssembly.
///
/// Spec: <https://webassembly.github.io/spec/core/exec/runtime.html#table-instances>
#[derive(Debug, Clone, PartialEq, Eq, derive_more::From)]
#[cfg_attr(feature = "artifact-size", derive(loupe::MemoryUsage))]
pub enum RuntimeTable {
    #[cfg(feature = "sys")]
    /// The extern ref from the `sys` runtime.
    Sys(crate::rt::sys::entities::table::Table),
    #[cfg(feature = "wamr")]
    /// The extern ref from the `wamr` runtime.
    Wamr(crate::rt::wamr::entities::table::Table),
    #[cfg(feature = "v8")]
    /// The extern ref from the `v8` runtime.
    V8(crate::rt::v8::entities::table::Table),
}

impl RuntimeTable {
    /// Creates a new table with the provided [`TableType`] definition.
    ///
    /// All the elements in the table will be set to the `init` value.
    ///
    /// This function will construct the table using the store `BaseTunables`.
    pub fn new(
        store: &mut impl AsStoreMut,
        ty: TableType,
        init: Value,
    ) -> Result<Self, RuntimeError> {
        match &store.as_store_mut().inner.store {
            #[cfg(feature = "sys")]
            RuntimeStore::Sys(_) => Ok(Self::Sys(crate::rt::sys::entities::table::Table::new(
                store, ty, init,
            )?)),
            #[cfg(feature = "wamr")]
            RuntimeStore::Wamr(_) => Ok(Self::Wamr(crate::rt::wamr::entities::table::Table::new(
                store, ty, init,
            )?)),
            #[cfg(feature = "v8")]
            RuntimeStore::V8(_) => Ok(Self::V8(crate::rt::v8::entities::table::Table::new(
                store, ty, init,
            )?)),

            _ => panic!("No runtime enabled!"),
        }
    }

    /// Returns the [`TableType`] of the table.
    pub fn ty(&self, store: &impl AsStoreRef) -> TableType {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(t) => t.ty(store),

            #[cfg(feature = "wamr")]
            Self::Wamr(t) => t.ty(store),

            #[cfg(feature = "v8")]
            Self::V8(t) => t.ty(store),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Retrieves an element of the table at the provided `index`.
    pub fn get(&self, store: &mut impl AsStoreMut, index: u32) -> Option<Value> {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(t) => t.get(store, index),

            #[cfg(feature = "wamr")]
            Self::Wamr(t) => t.get(store, index),
            #[cfg(feature = "v8")]
            Self::V8(t) => t.get(store, index),

            _ => panic!("No runtime enabled!"),
        }
    }

    /// Sets an element `val` in the Table at the provided `index`.
    pub fn set(
        &self,
        store: &mut impl AsStoreMut,
        index: u32,
        val: Value,
    ) -> Result<(), RuntimeError> {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(t) => t.set(store, index, val),

            #[cfg(feature = "wamr")]
            Self::Wamr(t) => t.set(store, index, val),

            #[cfg(feature = "v8")]
            Self::V8(t) => t.set(store, index, val),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Retrieves the size of the `Table` (in elements)
    pub fn size(&self, store: &impl AsStoreRef) -> u32 {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(t) => t.size(store),

            #[cfg(feature = "wamr")]
            Self::Wamr(t) => t.size(store),

            #[cfg(feature = "v8")]
            Self::V8(t) => t.size(store),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Grows the size of the `Table` by `delta`, initializating
    /// the elements with the provided `init` value.
    ///
    /// It returns the previous size of the `Table` in case is able
    /// to grow the Table successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if the `delta` is out of bounds for the table.
    pub fn grow(
        &self,
        store: &mut impl AsStoreMut,
        delta: u32,
        init: Value,
    ) -> Result<u32, RuntimeError> {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(t) => t.grow(store, delta, init),
            #[cfg(feature = "wamr")]
            Self::Wamr(t) => t.grow(store, delta, init),
            #[cfg(feature = "v8")]
            Self::V8(t) => t.grow(store, delta, init),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Copies the `len` elements of `src_table` starting at `src_index`
    /// to the destination table `dst_table` at index `dst_index`.
    ///
    /// # Errors
    ///
    /// Returns an error if the range is out of bounds of either the source or
    /// destination tables.
    pub fn copy(
        store: &mut impl AsStoreMut,
        dst_table: &Self,
        dst_index: u32,
        src_table: &Self,
        src_index: u32,
        len: u32,
    ) -> Result<(), RuntimeError> {
        match &store.as_store_mut().inner.store {
            #[cfg(feature = "sys")]
            RuntimeStore::Sys(_) => crate::rt::sys::entities::table::Table::copy(
                store,
                dst_table.as_sys(),
                dst_index,
                src_table.as_sys(),
                src_index,
                len,
            ),
            #[cfg(feature = "wamr")]
            RuntimeStore::Wamr(_) => crate::rt::wamr::entities::table::Table::copy(
                store,
                dst_table.as_wamr(),
                dst_index,
                src_table.as_wamr(),
                src_index,
                len,
            ),
            #[cfg(feature = "v8")]
            RuntimeStore::V8(_) => crate::rt::v8::entities::table::Table::copy(
                store,
                dst_table.as_v8(),
                dst_index,
                src_table.as_v8(),
                src_index,
                len,
            ),

            _ => panic!("No runtime enabled!"),
        }
    }

    pub(crate) fn from_vm_extern(store: &mut impl AsStoreMut, ext: VMExternTable) -> Self {
        match &store.as_store_mut().inner.store {
            #[cfg(feature = "sys")]
            RuntimeStore::Sys(_) => Self::Sys(
                crate::rt::sys::entities::table::Table::from_vm_extern(store, ext),
            ),

            #[cfg(feature = "wamr")]
            RuntimeStore::Wamr(_) => Self::Wamr(
                crate::rt::wamr::entities::table::Table::from_vm_extern(store, ext),
            ),
            #[cfg(feature = "v8")]
            RuntimeStore::V8(_) => Self::V8(crate::rt::v8::entities::table::Table::from_vm_extern(
                store, ext,
            )),

            _ => panic!("No runtime enabled!"),
        }
    }

    /// Checks whether this `Table` can be used with the given context.
    pub fn is_from_store(&self, store: &impl AsStoreRef) -> bool {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(t) => t.is_from_store(store),

            #[cfg(feature = "wamr")]
            Self::Wamr(t) => t.is_from_store(store),

            #[cfg(feature = "v8")]
            Self::V8(t) => t.is_from_store(store),
            _ => panic!("No runtime enabled!"),
        }
    }

    pub(crate) fn to_vm_extern(&self) -> VMExtern {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(t) => t.to_vm_extern(),

            #[cfg(feature = "wamr")]
            Self::Wamr(t) => t.to_vm_extern(),

            #[cfg(feature = "v8")]
            Self::V8(t) => t.to_vm_extern(),
            _ => panic!("No runtime enabled!"),
        }
    }
}

#[cfg(test)]
mod test {
    /// Check the example from <https://github.com/wasmerio/wasmer/issues/3197>.
    #[test]
    #[cfg_attr(
        feature = "wamr",
        ignore = "wamr does not support direct calls to grow table"
    )]
    #[cfg_attr(feature = "wasmi", ignore = "wasmi does not support funcrefs")]
    #[cfg_attr(
        feature = "v8",
        ignore = "growing tables in v8 is not currently supported"
    )]
    fn table_grow_issue_3197() {
        use crate::{imports, Instance, Module, Store, Table, TableType, Type, Value};

        const WAT: &str = r#"(module (table (import "env" "table") 100 funcref))"#;

        // Tests that the table type of `table` is compatible with the export in the WAT
        // This tests that `wasmer_types::types::is_table_compatible` works as expected.
        let mut store = Store::default();
        let module = Module::new(&store, WAT).unwrap();
        let ty = TableType::new(Type::FuncRef, 0, None);
        let table = Table::new(&mut store, ty, Value::FuncRef(None)).unwrap();
        table.grow(&mut store, 100, Value::FuncRef(None)).unwrap();
        assert_eq!(table.ty(&store).minimum, 0);
        let imports = imports! {"env" => {"table" => table}};
        let _instance = Instance::new(&mut store, &module, &imports).unwrap();
    }
}