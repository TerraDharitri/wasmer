use super::{shared::SharedMemory, view::*};
use wasmer_types::{MemoryType, Pages};
use wasmer_vm::{LinearMemory, MemoryError};

use crate::{
    vm::{VMExtern, VMExternMemory, VMMemory},
    AsStoreMut, AsStoreRef, ExportError, Exportable, Extern, StoreMut, StoreRef,
};

#[derive(Debug, Clone, PartialEq, Eq, derive_more::From)]
#[cfg_attr(feature = "artifact-size", derive(loupe::MemoryUsage))]
pub enum RuntimeMemory {
    #[cfg(feature = "sys")]
    /// The memory from the `sys` runtime.
    Sys(crate::rt::sys::entities::memory::Memory),

    #[cfg(feature = "wamr")]
    /// The memory from the `wamr` runtime.
    Wamr(crate::rt::wamr::entities::memory::Memory),

    #[cfg(feature = "v8")]
    /// The memory from the `v8` runtime.
    V8(crate::rt::v8::entities::memory::Memory),
}

impl RuntimeMemory {
    /// Creates a new host [`Memory`] from the provided [`MemoryType`].
    ///
    /// This function will construct the `Memory` using the store
    /// `BaseTunables`.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmer::{Memory, MemoryType, Pages, Store, Type, Value};
    /// # let mut store = Store::default();
    /// #
    /// let m = Memory::new(&mut store, MemoryType::new(1, None, false)).unwrap();
    /// ```
    pub fn new(store: &mut impl AsStoreMut, ty: MemoryType) -> Result<Self, MemoryError> {
        match &store.as_store_mut().inner.store {
            #[cfg(feature = "sys")]
            crate::RuntimeStore::Sys(s) => Ok(Self::Sys(
                crate::rt::sys::entities::memory::Memory::new(store, ty)?,
            )),
            #[cfg(feature = "wamr")]
            crate::RuntimeStore::Wamr(s) => Ok(Self::Wamr(
                crate::rt::wamr::entities::memory::Memory::new(store, ty)?,
            )),
            #[cfg(feature = "v8")]
            crate::RuntimeStore::V8(s) => Ok(Self::V8(
                crate::rt::v8::entities::memory::Memory::new(store, ty)?,
            )),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Create a memory object from an existing memory and attaches it to the store
    pub fn new_from_existing(new_store: &mut impl AsStoreMut, memory: VMMemory) -> Self {
        match new_store.as_store_mut().inner.store {
            #[cfg(feature = "sys")]
            crate::RuntimeStore::Sys(_) => {
                Self::Sys(crate::rt::sys::entities::memory::Memory::new_from_existing(
                    new_store,
                    memory.into_sys(),
                ))
            }
            #[cfg(feature = "wamr")]
            crate::RuntimeStore::Wamr(_) => Self::Wamr(
                crate::rt::wamr::entities::memory::Memory::new_from_existing(
                    new_store,
                    memory.into_wamr(),
                ),
            ),
            #[cfg(feature = "v8")]
            crate::RuntimeStore::V8(_) => {
                Self::V8(crate::rt::v8::entities::memory::Memory::new_from_existing(
                    new_store,
                    memory.into_v8(),
                ))
            }

            _ => panic!("No runtime enabled!"),
        }
    }

    /// Returns the [`MemoryType`] of the `Memory`.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmer::{Memory, MemoryType, Pages, Store, Type, Value};
    /// # let mut store = Store::default();
    /// #
    /// let mt = MemoryType::new(1, None, false);
    /// let m = Memory::new(&mut store, mt).unwrap();
    ///
    /// assert_eq!(m.ty(&mut store), mt);
    /// ```
    pub fn ty(&self, store: &impl AsStoreRef) -> MemoryType {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(s) => s.ty(store),
            #[cfg(feature = "wamr")]
            Self::Wamr(s) => s.ty(store),
            #[cfg(feature = "v8")]
            Self::V8(s) => s.ty(store),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Grow memory by the specified amount of WebAssembly [`Pages`] and return
    /// the previous memory size.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmer::{Memory, MemoryType, Pages, Store, Type, Value, WASM_MAX_PAGES};
    /// # let mut store = Store::default();
    /// #
    /// let m = Memory::new(&mut store, MemoryType::new(1, Some(3), false)).unwrap();
    /// let p = m.grow(&mut store, 2).unwrap();
    ///
    /// assert_eq!(p, Pages(1));
    /// assert_eq!(m.view(&mut store).size(), Pages(3));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if memory can't be grown by the specified amount
    /// of pages.
    ///
    /// ```should_panic
    /// # use wasmer::{Memory, MemoryType, Pages, Store, Type, Value, WASM_MAX_PAGES};
    /// # use wasmer::FunctionEnv;
    /// # let mut store = Store::default();
    /// # let env = FunctionEnv::new(&mut store, ());
    /// #
    /// let m = Memory::new(&mut store, MemoryType::new(1, Some(1), false)).unwrap();
    ///
    /// // This results in an error: `MemoryError::CouldNotGrow`.
    /// let s = m.grow(&mut store, 1).unwrap();
    /// ```
    pub fn grow<IntoPages>(
        &self,
        store: &mut impl AsStoreMut,
        delta: IntoPages,
    ) -> Result<Pages, MemoryError>
    where
        IntoPages: Into<Pages>,
    {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(s) => s.grow(store, delta),
            #[cfg(feature = "wamr")]
            Self::Wamr(s) => s.grow(store, delta),
            #[cfg(feature = "v8")]
            Self::V8(s) => s.grow(store, delta),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Grows the memory to at least a minimum size.
    ///
    /// # Note
    ///
    /// If the memory is already big enough for the min size this function does nothing.
    pub fn grow_at_least(
        &self,
        store: &mut impl AsStoreMut,
        min_size: u64,
    ) -> Result<(), MemoryError> {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(s) => s.grow_at_least(store, min_size),
            #[cfg(feature = "wamr")]
            Self::Wamr(s) => s.grow_at_least(store, min_size),
            #[cfg(feature = "v8")]
            Self::V8(s) => s.grow_at_least(store, min_size),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Resets the memory back to zero length
    pub fn reset(&self, store: &mut impl AsStoreMut) -> Result<(), MemoryError> {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(s) => s.reset(store),
            #[cfg(feature = "wamr")]
            Self::Wamr(s) => s.reset(store),
            #[cfg(feature = "v8")]
            Self::V8(s) => s.reset(store),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Attempts to duplicate this memory (if its clonable) in a new store
    /// (copied memory)
    pub fn copy_to_store(
        &self,
        store: &impl AsStoreRef,
        new_store: &mut impl AsStoreMut,
    ) -> Result<Self, MemoryError> {
        if !self.ty(store).shared {
            // We should only be able to duplicate in a new store if the memory is shared
            return Err(MemoryError::InvalidMemory {
                reason: "memory is not a shared memory type".to_string(),
            });
        }

        match self {
            #[cfg(feature = "sys")]
            Self::Sys(s) => s.try_copy(store).map(|new_memory| {
                Self::new_from_existing(
                    new_store,
                    VMMemory::Sys(crate::rt::sys::vm::VMMemory(new_memory)),
                )
            }),
            #[cfg(feature = "wamr")]
            Self::Wamr(s) => s
                .try_copy(store)
                .map(|new_memory| Self::new_from_existing(new_store, VMMemory::Wamr(new_memory))),
            #[cfg(feature = "v8")]
            Self::V8(s) => s
                .try_copy(store)
                .map(|new_memory| Self::new_from_existing(new_store, VMMemory::V8(new_memory))),
            _ => panic!("No runtime enabled!"),
        }
    }

    pub(crate) fn from_vm_extern(store: &mut impl AsStoreMut, vm_extern: VMExternMemory) -> Self {
        match &store.as_store_mut().inner.store {
            #[cfg(feature = "sys")]
            crate::RuntimeStore::Sys(s) => Self::Sys(
                crate::rt::sys::entities::memory::Memory::from_vm_extern(store, vm_extern),
            ),
            #[cfg(feature = "wamr")]
            crate::RuntimeStore::Wamr(s) => Self::Wamr(
                crate::rt::wamr::entities::memory::Memory::from_vm_extern(store, vm_extern),
            ),
            #[cfg(feature = "v8")]
            crate::RuntimeStore::V8(s) => Self::V8(
                crate::rt::v8::entities::memory::Memory::from_vm_extern(store, vm_extern),
            ),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Checks whether this `Memory` can be used with the given context.
    pub fn is_from_store(&self, store: &impl AsStoreRef) -> bool {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(s) => s.is_from_store(store),
            #[cfg(feature = "wamr")]
            Self::Wamr(s) => s.is_from_store(store),
            #[cfg(feature = "v8")]
            Self::V8(s) => s.is_from_store(store),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Attempt to create a new reference to the underlying memory; this new reference can then be
    /// used within a different store (from the same implementer).
    ///
    /// # Errors
    ///
    /// Fails if the underlying memory is not clonable.
    pub fn try_clone(&self, store: &impl AsStoreRef) -> Result<VMMemory, MemoryError> {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(s) => s.try_clone(store).map(VMMemory::Sys),
            #[cfg(feature = "wamr")]
            Self::Wamr(s) => s.try_clone(store).map(VMMemory::Wamr),
            #[cfg(feature = "v8")]
            Self::V8(s) => s.try_clone(store).map(VMMemory::V8),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Attempts to clone this memory (if its clonable) in a new store
    /// (cloned memory will be shared between those that clone it)
    pub fn share_in_store(
        &self,
        store: &impl AsStoreRef,
        new_store: &mut impl AsStoreMut,
    ) -> Result<Self, MemoryError> {
        if !self.ty(store).shared {
            // We should only be able to duplicate in a new store if the memory is shared
            return Err(MemoryError::InvalidMemory {
                reason: "memory is not a shared memory type".to_string(),
            });
        }

        match self {
            #[cfg(feature = "sys")]
            Self::Sys(s) => s
                .try_clone(store)
                .map(|new_memory| Self::new_from_existing(new_store, VMMemory::Sys(new_memory))),
            #[cfg(feature = "wamr")]
            Self::Wamr(s) => s
                .try_clone(store)
                .map(|new_memory| Self::new_from_existing(new_store, VMMemory::Wamr(new_memory))),
            #[cfg(feature = "v8")]
            Self::V8(s) => s
                .try_clone(store)
                .map(|new_memory| Self::new_from_existing(new_store, VMMemory::V8(new_memory))),

            _ => panic!("No runtime enabled!"),
        }
    }

    /// Get a [`SharedMemory`].
    ///
    /// Only returns `Some(_)` if the memory is shared, and if the target
    /// backend supports shared memory operations.
    ///
    /// See [`SharedMemory`] and its methods for more information.
    pub fn as_shared(&self, store: &impl AsStoreRef) -> Option<SharedMemory> {
        if !self.ty(store).shared {
            return None;
        }
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(s) => s.as_shared(store),
            #[cfg(feature = "wamr")]
            Self::Wamr(s) => s.as_shared(store),
            #[cfg(feature = "v8")]
            Self::V8(s) => s.as_shared(store),
            _ => panic!("No runtime enabled!"),
        }
    }

    /// Create a [`VMExtern`] from self.
    pub(crate) fn to_vm_extern(&self) -> VMExtern {
        match self {
            #[cfg(feature = "sys")]
            Self::Sys(s) => s.to_vm_extern(),
            #[cfg(feature = "wamr")]
            Self::Wamr(s) => s.to_vm_extern(),
            #[cfg(feature = "v8")]
            Self::V8(s) => s.to_vm_extern(),
            _ => panic!("No runtime enabled!"),
        }
    }
}