#![allow(non_snake_case)]
use std::{
    ffi::c_void,
    panic::{self, AssertUnwindSafe},
};

use crate::{
    vm::{VMExtern, VMExternFunction},
    wamr::{
        bindings::*,
        utils::convert::{IntoCApiType, IntoCApiValue, IntoWasmerType, IntoWasmerValue},
        vm::{VMFuncRef, VMFunction, VMFunctionCallback, VMFunctionEnvironment},
    },
    AsStoreMut, AsStoreRef, FromToNativeWasmType, FunctionEnv, FunctionEnvMut, IntoResult,
    NativeWasmTypeInto, RuntimeError, RuntimeFunction, RuntimeTrap, StoreMut, Value, WasmTypeList,
    WithEnv, WithoutEnv,
};

use super::{super::error::Trap, store::StoreHandle};
use wasmer_types::{FunctionType, RawValue};

pub(crate) mod env;
pub(crate) mod typed;

pub use typed::*;

type CCallback = unsafe extern "C" fn(
    *mut c_void,
    *const wasm_val_vec_t,
    *mut wasm_val_vec_t,
) -> *mut wasm_trap_t;

#[derive(Clone, PartialEq, Eq)]
pub struct Function {
    pub(crate) handle: VMFunction,
}

unsafe impl Send for Function {}
unsafe impl Sync for Function {}

impl From<VMFunction> for Function {
    fn from(handle: VMFunction) -> Self {
        Self { handle }
    }
}

pub(crate) struct FunctionCallbackEnv<'a, F> {
    pub(crate) store: StoreMut<'a>,
    pub(crate) func: F,
    pub(crate) env_handle: Option<StoreHandle<VMFunctionEnvironment>>,
}

impl<'a, F> std::fmt::Debug for FunctionCallbackEnv<'a, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionCallbackEnv")
            .field("env_is_some", &self.env_handle.is_some())
            .finish()
    }
}

impl Function {
    /// To `VMExtern`.
    pub fn to_vm_extern(&self) -> VMExtern {
        let extern_ = unsafe { wasm_func_as_extern(self.handle) };
        assert!(
            !extern_.is_null(),
            "Returned null Function extern from wasm-c-api"
        );
        VMExtern::Wamr(extern_)
    }

    #[allow(clippy::cast_ptr_alignment)]
    pub fn new_with_env<FT, F, T: Send + 'static>(
        store: &mut impl AsStoreMut,
        env: &FunctionEnv<T>,
        ty: FT,
        func: F,
    ) -> Self
    where
        FT: Into<FunctionType>,
        F: Fn(FunctionEnvMut<'_, T>, &[Value]) -> Result<Vec<Value>, RuntimeError>
            + 'static
            + Send
            + Sync,
    {
        let fn_ty: FunctionType = ty.into();
        let params = fn_ty.params();

        let mut param_types = params
            .into_iter()
            .map(|param| {
                let kind = param.into_ct();
                unsafe { wasm_valtype_new(kind) }
            })
            .collect::<Vec<_>>();

        let mut wasm_param_types = unsafe {
            let mut vec = Default::default();
            wasm_valtype_vec_new(&mut vec, param_types.len(), param_types.as_ptr());
            vec
        };

        let results = fn_ty.results();
        let mut result_types = results
            .into_iter()
            .map(|param| {
                let kind = param.into_ct();
                unsafe { wasm_valtype_new(kind) }
            })
            .collect::<Vec<_>>();

        let mut wasm_result_types = unsafe {
            let mut vec = Default::default();
            wasm_valtype_vec_new(&mut vec, result_types.len(), result_types.as_ptr());
            vec
        };

        let wasm_functype = unsafe {
            wasm_functype_new(
                &mut wasm_param_types as *mut _,
                &mut wasm_result_types as *mut _,
            )
        };

        let mut store = store.as_store_mut();
        let inner = store.inner.store.as_wamr().inner;

        let callback: CCallback = make_fn_callback(&func, param_types.len());

        let mut callback_env: *mut FunctionCallbackEnv<'_, F> =
            Box::leak(Box::new(FunctionCallbackEnv {
                store,
                func,
                env_handle: Some(env.as_wamr().handle.clone()),
            }));

        let wasm_function = unsafe {
            wasm_func_new_with_env(
                inner,
                wasm_functype,
                Some(callback),
                callback_env as *mut _ as _,
                None,
            )
        };

        if wasm_function.is_null() {
            panic!("failed when creating new typed function");
        }

        Function {
            handle: wasm_function,
        }
    }

    /// Creates a new host `Function` from a native function.
    pub fn new_typed<F, Args, Rets>(store: &mut impl AsStoreMut, func: F) -> Self
    where
        F: crate::HostFunction<(), Args, Rets, WithoutEnv> + 'static + Send + Sync,
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let mut param_types = Args::wasm_types()
            .into_iter()
            .map(|param| {
                let kind = param.into_ct();
                unsafe { wasm_valtype_new(kind) }
            })
            .collect::<Vec<_>>();

        let mut wasm_param_types = unsafe {
            let mut vec = Default::default();
            wasm_valtype_vec_new(&mut vec, param_types.len(), param_types.as_ptr());
            vec
        };

        let mut result_types = Rets::wasm_types()
            .into_iter()
            .map(|param| {
                let kind = param.into_ct();
                unsafe { wasm_valtype_new(kind) }
            })
            .collect::<Vec<_>>();

        let mut wasm_result_types = unsafe {
            let mut vec = Default::default();
            wasm_valtype_vec_new(&mut vec, result_types.len(), result_types.as_ptr());
            vec
        };

        let wasm_functype =
            unsafe { wasm_functype_new(&mut wasm_param_types, &mut wasm_result_types) };

        let mut store = store.as_store_mut();
        let inner = store.inner.store.as_wamr().inner;

        let callback: CCallback = unsafe { std::mem::transmute(func.wamr_function_callback()) };

        let mut callback_env: *mut FunctionCallbackEnv<'_, F> =
            Box::into_raw(Box::new(FunctionCallbackEnv {
                store,
                func,
                env_handle: None,
            }));

        let wasm_function = unsafe {
            wasm_func_new_with_env(
                inner,
                wasm_functype,
                Some(callback),
                callback_env as _,
                None,
            )
        };

        if wasm_function.is_null() {
            panic!("failed when creating new typed function");
        }

        Function {
            handle: wasm_function,
        }
    }

    pub fn new_typed_with_env<T, F, Args, Rets>(
        store: &mut impl AsStoreMut,
        env: &FunctionEnv<T>,
        func: F,
    ) -> Self
    where
        F: crate::HostFunction<T, Args, Rets, WithEnv>,
        Args: WasmTypeList,
        Rets: WasmTypeList,
        T: Send + 'static,
    {
        let mut param_types = Args::wasm_types()
            .into_iter()
            .map(|param| {
                let kind = param.into_ct();
                unsafe { wasm_valtype_new(kind) }
            })
            .collect::<Vec<_>>();

        let mut wasm_param_types = unsafe {
            let mut vec = wasm_valtype_vec_t::default();
            wasm_valtype_vec_new(&mut vec, param_types.len(), param_types.as_ptr());
            vec
        };

        let mut result_types = Rets::wasm_types()
            .into_iter()
            .map(|param| {
                let kind = param.into_ct();
                unsafe { wasm_valtype_new(kind) }
            })
            .collect::<Vec<_>>();

        let mut wasm_result_types = unsafe {
            let mut vec: wasm_valtype_vec_t = Default::default();
            wasm_valtype_vec_new(&mut vec, result_types.len(), result_types.as_ptr());
            vec
        };

        let wasm_functype = unsafe {
            wasm_functype_new(
                &mut wasm_param_types as *mut _,
                &mut wasm_result_types as *mut _,
            )
        };

        let mut store = store.as_store_mut();
        let inner = store.inner.store.as_wamr().inner;

        let callback: CCallback = unsafe { std::mem::transmute(func.wamr_function_callback()) };

        let mut callback_env: *mut FunctionCallbackEnv<'_, F> =
            Box::into_raw(Box::new(FunctionCallbackEnv {
                store,
                func,
                env_handle: Some(env.as_wamr().handle.clone()),
            }));

        let wasm_function = unsafe {
            wasm_func_new_with_env(
                inner,
                wasm_functype,
                Some(callback),
                callback_env as _,
                None,
            )
        };

        if wasm_function.is_null() {
            panic!("failed when creating new typed function");
        }

        Function {
            handle: wasm_function,
        }
    }

    pub fn ty(&self, _store: &impl AsStoreRef) -> FunctionType {
        let type_ = unsafe { wasm_func_type(self.handle) };
        let params: *const wasm_valtype_vec_t = unsafe { wasm_functype_params(type_) };
        let returns: *const wasm_valtype_vec_t = unsafe { wasm_functype_results(type_) };

        let params: Vec<wasmer_types::Type> = unsafe {
            let mut res = vec![];
            for i in 0..(*params).size {
                res.push((*(*params).data.wrapping_add(i)).into_wt());
            }
            res
        };

        let returns: Vec<wasmer_types::Type> = unsafe {
            let mut res = vec![];
            for i in 0..(*returns).size {
                res.push((*(*returns).data.wrapping_add(i)).into_wt());
            }
            res
        };

        FunctionType::new(params, returns)
    }

    pub fn call_raw(
        &self,
        _store: &mut impl AsStoreMut,
        _params: Vec<RawValue>,
    ) -> Result<Box<[Value]>, RuntimeError> {
        // There is no optimal call_raw in JSC, so we just
        // simply rely the call
        // self.call(store, params)
        unimplemented!();
    }

    pub fn call(
        &self,
        store: &mut impl AsStoreMut,
        params: &[Value],
    ) -> Result<Box<[Value]>, RuntimeError> {
        // unimplemented!();
        let store_mut = store.as_store_mut();
        // let wasm_func_param_arity(self.handle)

        let mut args = {
            unsafe {
                let mut wasm_params = params
                    .into_iter()
                    .map(|v| IntoCApiValue::into_cv(v.clone()))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                let mut vec = Default::default();
                wasm_val_vec_new(&mut vec, wasm_params.len(), wasm_params.as_ptr());
                vec
            }
        };

        let size = unsafe { wasm_func_result_arity(self.handle) };

        let mut results = {
            unsafe {
                let mut vec = Default::default();
                wasm_val_vec_new_uninitialized(&mut vec, size);
                vec
            }
        };

        let trap = unsafe { wasm_func_call(self.handle, &mut args as _, &mut results as *mut _) };

        if !trap.is_null() {
            return Err(Into::<Trap>::into(trap).into());
        }

        unsafe {
            let results = std::ptr::slice_from_raw_parts(results.data, results.size);
            return Ok((*results)
                .into_iter()
                .map(|v| IntoWasmerValue::into_wv(*v))
                .collect::<Vec<_>>()
                .into_boxed_slice());
        }
    }

    pub(crate) fn from_vm_extern(_store: &mut impl AsStoreMut, internal: VMExternFunction) -> Self {
        Self {
            handle: internal.into_wamr(),
        }
    }

    pub(crate) fn vm_funcref(&self, _store: &impl AsStoreRef) -> VMFuncRef {
        unimplemented!();
    }

    pub(crate) unsafe fn from_vm_funcref(
        _store: &mut impl AsStoreMut,
        _funcref: VMFuncRef,
    ) -> Self {
        unimplemented!();
    }

    /// Checks whether this `Function` can be used with the given context.
    pub fn is_from_store(&self, _store: &impl AsStoreRef) -> bool {
        true
    }
}

fn make_fn_callback<F, T: Send + 'static>(func: &F, args: usize) -> CCallback
where
    F: Fn(FunctionEnvMut<'_, T>, &[Value]) -> Result<Vec<Value>, RuntimeError>
        + 'static
        + Send
        + Sync,
{
    unsafe extern "C" fn fn_callback<F, T: Send + 'static>(
        env: *mut c_void,
        args: *const wasm_val_vec_t,
        rets: *mut wasm_val_vec_t,
    ) -> *mut wasm_trap_t
    where
        F: Fn(FunctionEnvMut<'_, T>, &[Value]) -> Result<Vec<Value>, RuntimeError>
            + 'static
            + Send
            + Sync,
    {
        let r: *mut (FunctionCallbackEnv<'_, F>) = env as _;

        let mut store = (*r).store.as_store_mut();
        let env_handle = (*r).env_handle.as_ref().unwrap().clone();
        let mut fn_env = env::FunctionEnv::from_handle(env_handle).into_mut(&mut store);
        let func: &F = &(*r).func;

        let mut wasmer_args = vec![];

        for i in 0..(*args).size {
            wasmer_args.push((*(*args).data.wrapping_add(i)).into_wv().clone());
        }

        let result = panic::catch_unwind(AssertUnwindSafe(|| unsafe {
            func(fn_env.into(), wasmer_args.as_slice())
        }));

        match result {
            Ok(Ok(native_results)) => {
                let mut c_results: Vec<wasm_val_t> = native_results
                    .into_iter()
                    .map(IntoCApiValue::into_cv)
                    .collect();

                if c_results.len() != (*rets).size {
                    panic!("when calling host function: number of observed results differ from wanted results")
                }

                unsafe {
                    for i in 0..(*rets).size {
                        *((*rets).data.wrapping_add(i)) = c_results[i]
                    }
                }

                unsafe { std::ptr::null_mut() }
            }

            Ok(Err(e)) => {
                let trap: Trap = Trap::user(Box::new(e));
                unsafe { trap.into_wasm_trap(&mut store) }
            }

            Err(e) => {
                unimplemented!("host function panicked");
            }
        }
    }

    return fn_callback::<F, T>;
}

impl std::fmt::Debug for Function {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("Function").finish()
    }
}

impl crate::Function {
    /// Consume [`self`] into [`crate::rt::wamr::function::Function`].
    pub fn into_wamr(self) -> crate::rt::wamr::function::Function {
        match self.0 {
            RuntimeFunction::Wamr(s) => s,
            _ => panic!("Not a `wamr` function!"),
        }
    }

    /// Convert a reference to [`self`] into a reference to [`crate::rt::wamr::function::Function`].
    pub fn as_wamr(&self) -> &crate::rt::wamr::function::Function {
        match self.0 {
            RuntimeFunction::Wamr(ref s) => s,
            _ => panic!("Not a `wamr` function!"),
        }
    }

    /// Convert a mutable reference to [`self`] into a mutable reference [`crate::rt::wamr::function::Function`].
    pub fn as_wamr_mut(&mut self) -> &mut crate::rt::wamr::function::Function {
        match self.0 {
            RuntimeFunction::Wamr(ref mut s) => s,
            _ => panic!("Not a `wamr` function!"),
        }
    }
}