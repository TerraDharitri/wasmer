use std::sync::Arc;

use crate::{
    rt::v8::bindings::*, v8::error::Trap, vm::VMExtern, AsStoreMut, AsStoreRef, Exports, Extern,
    Imports, InstantiationError, Module,
};

#[derive(PartialEq, Eq)]
pub(crate) struct InstanceHandle(pub(crate) *mut wasm_instance_t);

unsafe impl Send for InstanceHandle {}
unsafe impl Sync for InstanceHandle {}

impl InstanceHandle {
    fn new(
        store: *mut wasm_store_t,
        module: *mut wasm_module_t,
        mut externs: Vec<VMExtern>,
    ) -> Result<Self, InstantiationError> {
        let mut trap: *mut wasm_trap_t = std::ptr::null_mut() as _;

        let externs: Vec<_> = externs.into_iter().map(|v| v.into_v8()).collect();

        let instance = unsafe {
            let ptr = externs.as_ptr();
            std::mem::forget(externs);

            wasm_instance_new(store, module, ptr as *const *const _, &mut trap)
        };

        if instance.is_null() {
            let trap = Trap::from(trap);
        }

        Ok(InstanceHandle(instance))
    }

    fn get_exports(&self, mut store: &mut impl AsStoreMut, module: &Module) -> Exports {
        let mut exports = unsafe {
            let mut vec = Default::default();
            wasm_instance_exports(self.0, &mut vec);
            vec
        };

        let wasm_exports: &[*mut wasm_extern_t] =
            unsafe { std::slice::from_raw_parts(exports.data, exports.size) };

        let exports_ty = module.exports().collect::<Vec<_>>();
        let exports = exports_ty
            .iter()
            .zip(wasm_exports.into_iter())
            .map(|(export_type, wasm_export)| {
                let name = export_type.name();
                let mut store = store.as_store_mut();
                let extern_type = export_type.ty();
                // Annotation is here to prevent spurious IDE warnings.

                let extern_ = Extern::from_vm_extern(&mut store, VMExtern::V8(*wasm_export));
                (name.to_string(), extern_)
            })
            .collect::<Exports>();
        exports
    }
}
impl Drop for InstanceHandle {
    fn drop(&mut self) {
        unsafe { wasm_instance_delete(self.0) }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Instance {
    pub(crate) handle: Arc<InstanceHandle>,
}

impl Instance {
    pub(crate) fn new(
        store: &mut impl AsStoreMut,
        module: &Module,
        imports: &Imports,
    ) -> Result<(Self, Exports), InstantiationError> {
        let externs = module
            .imports()
            .map(|import_ty| {
                imports
                    .get_export(import_ty.module(), import_ty.name())
                    .expect("Extern not found")
            })
            .collect::<Vec<_>>();

        return Self::new_by_index(store, module, &externs);
    }

    pub(crate) fn new_by_index(
        store: &mut impl AsStoreMut,
        module: &Module,
        externs: &[Extern],
    ) -> Result<(Self, Exports), InstantiationError> {
        let store_ref = store.as_store_ref();
        let externs: Vec<VMExtern> = externs
            .iter()
            .map(|extern_| extern_.to_vm_extern())
            .collect::<Vec<_>>();
        let instance = InstanceHandle::new(
            store_ref.inner.store.as_v8().inner,
            module.as_v8().handle.inner,
            externs,
        )?;
        let exports = instance.get_exports(store, module);

        Ok((
            Self {
                handle: Arc::new(instance),
            },
            exports,
        ))
    }
}

impl crate::RuntimeInstance {
    /// Consume [`self`] into a [`crate::rt::v8::instance::Instance`].
    pub(crate) fn into_v8(self) -> crate::rt::v8::instance::Instance {
        match self {
            Self::V8(s) => s,
            _ => panic!("Not a `v8` instance"),
        }
    }
}