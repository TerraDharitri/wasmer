use std::{path::Path, sync::Arc};

use shared_buffer::OwnedBuffer;
pub use wasmer_compiler::{
    types::target::Target, Artifact, BaseTunables, CompilerConfig, Engine, EngineBuilder, Tunables,
};
#[cfg(feature = "compiler")]
use wasmer_types::Features;
use wasmer_types::{DeserializeError, HashAlgorithm};

use crate::RuntimeEngine;

/// Get the default config for the sys Engine
#[allow(unreachable_code)]
pub fn get_default_compiler_config() -> Option<Box<dyn wasmer_compiler::CompilerConfig>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "cranelift")] {
            Some(Box::<wasmer_compiler_cranelift::Cranelift>::default())
        } else if #[cfg(feature = "llvm")] {
            Some(Box::<wasmer_compiler_llvm::LLVM>::default())
        } else if #[cfg(feature = "singlepass")] {
            Some(Box::<wasmer_compiler_singlepass::Singlepass>::default())
        }
        else {
            None
        }
    }
}

/// Returns the default engine for the Sys engine
pub(crate) fn default_engine() -> Engine {
    #[allow(unreachable_code, unused_mut)]
    fn get_engine() -> Engine {
        cfg_if::cfg_if! {
            if #[cfg(feature = "compiler")] {
                if let Some(config) = get_default_compiler_config() {
                    EngineBuilder::new(config)
                        .engine()
                } else {
                    EngineBuilder::headless()
                        .engine()
                }
            } else {
                EngineBuilder::headless().engine()
            }
        }
    }

    let mut engine = get_engine();
    let tunables = BaseTunables::for_target(engine.target());
    engine.set_tunables(tunables);
    engine
}

/// The custom trait to access to all the `sys` function in the common
/// engine.
pub trait NativeEngineExt {
    /// Create a new `Engine` with the given config
    #[cfg(feature = "compiler")]
    fn new(compiler_config: Box<dyn CompilerConfig>, target: Target, features: Features) -> Self;

    /// Sets the hash algorithm
    fn set_hash_algorithm(&mut self, hash_algorithm: Option<HashAlgorithm>);

    /// Create a headless `Engine`
    ///
    /// A headless engine is an engine without any compiler attached.
    /// This is useful for assuring a minimal runtime for running
    /// WebAssembly modules.
    ///
    /// For example, for running in IoT devices where compilers are very
    /// expensive, or also to optimize startup speed.
    ///
    /// # Important
    ///
    /// Headless engines can't compile or validate any modules,
    /// they just take already processed Modules (via `Module::serialize`).
    fn headless() -> Self;

    /// Gets the target
    fn target(&self) -> &Target;

    /// Attach a Tunable to this engine
    fn set_tunables(&mut self, tunables: impl Tunables + Send + Sync + 'static);

    /// Get a reference to attached Tunable of this engine
    fn tunables(&self) -> &dyn Tunables;

    /// Load a serialized WebAssembly module from a memory mapped file and deserialize it.
    ///
    /// NOTE: you should almost always prefer [`Self::deserialize_from_mmapped_file`].
    ///
    /// # Safety
    /// See [`Artifact::deserialize_unchecked`].
    unsafe fn deserialize_from_mmapped_file_unchecked(
        &self,
        file_ref: &Path,
    ) -> Result<crate::Module, DeserializeError>;

    /// Load a serialized WebAssembly module from a memory mapped file and deserialize it.
    ///
    /// # Safety
    /// See [`Artifact::deserialize`].
    unsafe fn deserialize_from_mmapped_file(
        &self,
        file_ref: &Path,
    ) -> Result<crate::Module, DeserializeError>;
}

impl NativeEngineExt for crate::engine::Engine {
    #[cfg(feature = "compiler")]
    fn new(compiler_config: Box<dyn CompilerConfig>, target: Target, features: Features) -> Self {
        Self(RuntimeEngine::Sys(Engine::new(
            compiler_config,
            target,
            features,
        )))
    }

    fn headless() -> Self {
        Self(RuntimeEngine::Sys(Engine::headless()))
    }

    fn target(&self) -> &Target {
        match self.0 {
            RuntimeEngine::Sys(ref s) => s.target(),
            _ => panic!("Not a `sys` engine!"),
        }
    }

    fn set_tunables(&mut self, tunables: impl Tunables + Send + Sync + 'static) {
        match self.0 {
            RuntimeEngine::Sys(ref mut s) => s.set_tunables(tunables),
            _ => panic!("Not a `sys` engine!"),
        }
    }

    fn tunables(&self) -> &dyn Tunables {
        match self.0 {
            RuntimeEngine::Sys(ref s) => s.tunables(),
            _ => panic!("Not a `sys` engine!"),
        }
    }

    unsafe fn deserialize_from_mmapped_file_unchecked(
        &self,
        file_ref: &Path,
    ) -> Result<crate::Module, DeserializeError> {
        let file = std::fs::File::open(file_ref)?;
        let artifact = Arc::new(Artifact::deserialize_unchecked(
            self.as_sys(),
            OwnedBuffer::from_file(&file)
                .map_err(|e| DeserializeError::Generic(format!("{e:?}")))?,
        )?);
        Ok(crate::Module::Sys(super::module::Module::from_artifact(
            artifact,
        )))
    }

    unsafe fn deserialize_from_mmapped_file(
        &self,
        file_ref: &Path,
    ) -> Result<crate::Module, DeserializeError> {
        let file = std::fs::File::open(file_ref)?;
        let artifact = Arc::new(Artifact::deserialize(
            self.as_sys(),
            OwnedBuffer::from_file(&file)
                .map_err(|e| DeserializeError::Generic(format!("{e:?}")))?,
        )?);
        Ok(crate::Module::Sys(super::module::Module::from_artifact(
            artifact,
        )))
    }

    fn set_hash_algorithm(&mut self, hash_algorithm: Option<HashAlgorithm>) {
        match self.0 {
            RuntimeEngine::Sys(ref mut s) => s.set_hash_algorithm(hash_algorithm),
            _ => panic!("Not a `sys` engine!"),
        }
    }
}

impl crate::Engine {
    /// Consume [`self`] into a [`crate::rt::sys::engine::Engine`].
    pub fn into_sys(self) -> crate::rt::sys::engine::Engine {
        match self.0 {
            RuntimeEngine::Sys(s) => s,
            _ => panic!("Not a `sys` engine!"),
        }
    }

    /// Convert a reference to [`self`] into a reference [`crate::rt::sys::engine::Engine`].
    pub fn as_sys(&self) -> &crate::rt::sys::engine::Engine {
        match self.0 {
            RuntimeEngine::Sys(ref s) => s,
            _ => panic!("Not a `sys` engine!"),
        }
    }

    /// Convert a mutable reference to [`self`] into a mutable reference [`crate::rt::sys::engine::Engine`].
    pub fn as_sys_mut(&mut self) -> &mut crate::rt::sys::engine::Engine {
        match self.0 {
            RuntimeEngine::Sys(ref mut s) => s,
            _ => panic!("Not a `sys` engine!"),
        }
    }

    /// Return true if [`self`] is an engine from the `sys` runtime.
    pub fn is_sys(&self) -> bool {
        matches!(self.0, RuntimeEngine::Sys(_))
    }
}

impl From<Engine> for crate::Engine {
    fn from(value: Engine) -> Self {
        Self(RuntimeEngine::Sys(value))
    }
}

impl From<&Engine> for crate::Engine {
    fn from(value: &Engine) -> Self {
        Self(RuntimeEngine::Sys(value.cloned()))
    }
}

#[cfg(feature = "cranelift")]
impl From<wasmer_compiler_cranelift::Cranelift> for crate::Engine {
    fn from(value: wasmer_compiler_cranelift::Cranelift) -> Self {
        Self(RuntimeEngine::Sys(value.into()))
    }
}

#[cfg(feature = "singlepass")]
impl From<wasmer_compiler_singlepass::Singlepass> for crate::Engine {
    fn from(value: wasmer_compiler_singlepass::Singlepass) -> Self {
        Self(RuntimeEngine::Sys(value.into()))
    }
}

#[cfg(feature = "llvm")]
impl From<wasmer_compiler_llvm::LLVM> for crate::Engine {
    fn from(value: wasmer_compiler_llvm::LLVM) -> Self {
        Self(RuntimeEngine::Sys(value.into()))
    }
}