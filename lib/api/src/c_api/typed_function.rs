//! Native Functions.
//!
//! This module creates the helper `TypedFunction` that let us call WebAssembly
//! functions with the native ABI, that is:
//!
//! ```ignore
//! let add_one = instance.exports.get_function("function_name")?;
//! let add_one_native: TypedFunction<i32, i32> = add_one.typed().unwrap();
//! ```
use crate::as_c::result_to_value;
use crate::bindings::{
    wasm_extern_as_func, wasm_func_call, wasm_func_t, wasm_global_set, wasm_global_t,
    wasm_val_vec_new, wasm_val_vec_new_uninitialized,
};

use crate::as_c::param_from_c;
use crate::c_api::externals::function::Function;
use crate::c_api::trap::Trap;
use crate::native_type::NativeWasmTypeInto;
use crate::Value;
use crate::{AsStoreMut, TypedFunction};
use crate::{FromToNativeWasmType, RuntimeError, WasmTypeList};
// use std::panic::{catch_unwind, AssertUnwindSafe};
use std::iter::FromIterator;
use wasmer_types::RawValue;

macro_rules! impl_native_traits {
    (  $( $x:ident ),* ) => {
        #[allow(unused_parens, non_snake_case)]
         impl<$( $x , )* Rets> TypedFunction<( $( $x ),* ), Rets>
         where
             $( $x: FromToNativeWasmType, )*
             Rets: WasmTypeList,
         {
             /// Call the typed func and return results.
             #[allow(clippy::too_many_arguments)]
             pub fn call(&self, mut store: &mut impl AsStoreMut, $( $x: $x, )* ) -> Result<Rets, RuntimeError> where
             $( $x: FromToNativeWasmType + NativeWasmTypeInto, )*
             {
                 // // let store_ptr = Value::I64(store.as_store_mut().as_raw() as _).as_jsvalue(store);
                 #[allow(unused_unsafe)]
                 let params_list: Vec<_> = unsafe {
                     vec![ $( {
                         let raw = $x.into_raw(store);
                         let value = Value::from_raw(&mut store, $x::WASM_TYPE, raw);
                         result_to_value(&value)
                     } ),* ]
                 };

                 let mut params = unsafe {std::mem::zeroed()};

                 unsafe {
                     wasm_val_vec_new(&mut params, params_list.len(), params_list.as_ptr());
                 }

                 let mut results = unsafe {std::mem::zeroed()};

                 unsafe {
                     wasm_val_vec_new_uninitialized(&mut results, Rets::wasm_types().len())
                 }

                 let func = unsafe { wasm_extern_as_func(self.func.to_vm_extern()) };

                 unsafe { wasm_func_call(func, &params, &mut results); }

                let mut rets_list_array = Rets::empty_array();
                let mut_rets = rets_list_array.as_mut() as *mut [RawValue] as *mut RawValue;

                match Rets::size() {
                    0 => {},
                    1 => unsafe {
                        let val = (*results.data.wrapping_add(0)).clone();
                        let val = param_from_c(&val);
                        *mut_rets = val.as_raw(&mut store);
                    }
                    _n => {
                        for (i, ret_type) in Rets::wasm_types().iter().enumerate() {
                            unsafe {
                                let val = (*results.data.wrapping_add(i)).clone();
                                let val = param_from_c(&val);
                                let slot = mut_rets.add(i);
                                *slot = val.as_raw(&mut store);
                            }
                        }
                    }
                }

                Ok(unsafe { Rets::from_array(store, rets_list_array) })

            }
        }
    };
}

impl_native_traits!();
impl_native_traits!(A1);
impl_native_traits!(A1, A2);
impl_native_traits!(A1, A2, A3);
impl_native_traits!(A1, A2, A3, A4);
impl_native_traits!(A1, A2, A3, A4, A5);
impl_native_traits!(A1, A2, A3, A4, A5, A6);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16);
impl_native_traits!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17);
impl_native_traits!(
    A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18
);
impl_native_traits!(
    A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19
);
impl_native_traits!(
    A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20
);
