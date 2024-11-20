use crate::{
    utils::{FromToNativeWasmType, IntoResult, NativeWasmTypeInto, WasmTypeList},
    AsStoreMut, AsStoreRef, FunctionEnv, FunctionEnvMut, FunctionType, HostFunction, RuntimeError,
    StoreInner, StoreMut, StoreRef, Value, WithEnv, WithoutEnv,
};

use std::panic::{self, AssertUnwindSafe};
use std::{cell::UnsafeCell, cmp::max, ffi::c_void};
use wasmer_types::{NativeWasmType, RawValue};

macro_rules! impl_host_function {
    ([$c_struct_representation:ident] $c_struct_name:ident, $( $x:ident ),* ) => {
#[allow(unused_parens)]
impl< $( $x, )* Rets, RetsAsResult, T, Func> crate::HostFunction<T, ( $( $x ),* ), Rets, WithEnv> for Func where
    $( $x: FromToNativeWasmType, )*
    Rets: WasmTypeList,
    RetsAsResult: IntoResult<Rets>,
    T: Send + 'static,
    Func: Fn(FunctionEnvMut<'_, T>, $( $x , )*) -> RetsAsResult + 'static,
{

  #[cfg(feature = "wamr")]
  #[allow(non_snake_case)]
  fn wamr_function_callback(&self) -> crate::rt::wamr::vm::VMFunctionCallback {
	use crate::rt::wamr::bindings::*;
    use crate::rt::wamr::utils::convert::*;

	unsafe extern "C" fn func_wrapper<$( $x, )* Rets, RetsAsResult, Func, T>(env: *mut c_void, args: *const wasm_val_vec_t, results: *mut wasm_val_vec_t) -> *mut wasm_trap_t
	where
	  $( $x: FromToNativeWasmType, )*
	  Rets: WasmTypeList,
	  RetsAsResult: IntoResult<Rets>,
	  T: Send + 'static,
	  Func: Fn(FunctionEnvMut<'_, T>, $( $x , )*) -> RetsAsResult + 'static,
    {

	  let r: *mut (crate::rt::wamr::function::FunctionCallbackEnv<'_, Func>) = env as _;
	  let store = &mut (*r).store.as_store_mut();

	  let mut i = 0;

	  $(
	  let c_arg = (*(*args).data.wrapping_add(i)).clone();
	  let wasmer_arg = c_arg.into_wv();
	  let raw_arg : RawValue = wasmer_arg.as_raw(store);
	  let $x : $x = FromToNativeWasmType::from_native($x::Native::from_raw(store, raw_arg));

	  i += 1;
      )*

	  let env_handle = (*r).env_handle.as_ref().unwrap().clone();
	  let mut fn_env = crate::rt::wamr::function::env::FunctionEnv::from_handle(env_handle).into_mut(store);
	  let func: &Func = &(*r).func;

	  let result = panic::catch_unwind(AssertUnwindSafe(|| unsafe {
	      ((*r).func)(FunctionEnvMut::Wamr(fn_env), $( $x, )* ).into_result()
	  }));


	  match result {
	      Ok(Ok(result)) => {
	  	  let types = Rets::wasm_types();
	  	  let mut native_results = result.into_array(store);
	  	  let native_results = native_results.as_mut();

	  	  let native_results: Vec<Value> = native_results.into_iter().enumerate().map(|(i, r)| Value::from_raw(store, types[i], r.clone())).collect();

	  	  let mut c_results: Vec<wasm_val_t> = native_results.into_iter().map(IntoCApiValue::into_cv).collect();

	  	  if c_results.len() != (*results).size {
	  	      panic!("when calling host function: number of observed results differ from wanted results")
	  	  }

	  	  unsafe {
	  	      for i in 0..(*results).size {
	  	          *((*results).data.wrapping_add(i)) = c_results[i]
	  	      }

	  	  }

	  	  unsafe { std::ptr::null_mut() }
	    },

	    Ok(Err(e)) => { let trap = crate::rt::wamr::error::Trap::user(Box::new(e)); unsafe { trap.into_wasm_trap(store) } },

	    Err(e) => { unimplemented!("host function panicked"); }
	  }
	}

	func_wrapper::< $( $x, )* Rets, RetsAsResult, Self, T> as _

  }

  #[cfg(feature = "v8")]
  #[allow(non_snake_case)]
  fn v8_function_callback(&self) -> crate::rt::v8::vm::VMFunctionCallback {
	use crate::rt::v8::bindings::*;
    use crate::rt::v8::utils::convert::*;
    unsafe extern "C" fn func_wrapper<$( $x, )* Rets, RetsAsResult, Func, T>(env: *mut c_void, args: *const wasm_val_t, results: *mut wasm_val_t) -> *mut wasm_trap_t
    where
	  $( $x: FromToNativeWasmType, )*
	  Rets: WasmTypeList,
	  RetsAsResult: IntoResult<Rets>,
	  T: Send + 'static,
	  Func: Fn(FunctionEnvMut<'_, T>, $( $x , )*) -> RetsAsResult + 'static,
    {

	  let r: *mut (crate::rt::v8::function::FunctionCallbackEnv<'_, Func>) = env as _;
	  let store = &mut (*r).store.as_store_mut();
	  let mut i = 0;

      $(
	  let c_arg = (*(args).wrapping_add(i)).clone();
	  let wasmer_arg = c_arg.into_wv();
	  let raw_arg : RawValue = wasmer_arg.as_raw(store);
	  let $x : $x = FromToNativeWasmType::from_native($x::Native::from_raw(store, raw_arg));
	  i += 1;
	  )*

	  let env_handle = (*r).env_handle.as_ref().unwrap().clone();
	  let mut fn_env = crate::rt::v8::function::env::FunctionEnv::from_handle(env_handle).into_mut(store);
	  let func: &Func = &(*r).func;
	  let result = panic::catch_unwind(AssertUnwindSafe(|| unsafe {
	      ((*r).func)(FunctionEnvMut::V8(fn_env), $( $x, )* ).into_result()
	  }));

	  match result {
  	    Ok(Ok(result)) => {
		  let types = Rets::wasm_types();
  	  	  let size = types.len();
  	  	  let mut native_results = result.into_array(store);
  	  	  let native_results = native_results.as_mut();

  	  	  let native_results: Vec<Value> = native_results.into_iter().enumerate()
  	  	      .map(|(i, r)| Value::from_raw(store, types[i], r.clone()))
  	  	      .collect();

  	  	  let mut c_results: Vec<wasm_val_t> = native_results.into_iter().map(|r| r.into_cv()).collect();

  	  	  if c_results.len() != size {
  	  	      panic!("when calling host function: number of observed results differ from wanted results")
  	  	  }

  	  	  unsafe {
  	  	      for i in 0..size {
  	  	          *((results).wrapping_add(i)) = c_results[i]
  	  	      }
  	  	  }

  	  	  unsafe { std::ptr::null_mut() }
  	    },
  	    Ok(Err(e)) => {
		  let trap: crate::rt::v8::error::Trap =  crate::rt::v8::error::Trap::user(Box::new(e));
  	      unsafe { trap.into_wasm_trap(store) }
  	    },
  	    Err(e) => { unimplemented!("host function panicked"); }
  	  }
    }

    func_wrapper::< $( $x, )* Rets, RetsAsResult, Self, T> as _
  }

  #[cfg(feature = "sys")]
  #[allow(non_snake_case)]
  fn sys_function_callback(&self) -> crate::rt::sys::vm::VMFunctionCallback {
    unsafe extern "C" fn func_wrapper<T: Send + 'static, $( $x, )* Rets, RetsAsResult, Func>( env: &crate::rt::sys::function::StaticFunction<Func, T>, $( $x: <$x::Native as NativeWasmType>::Abi, )* ) -> Rets::CStruct
    where
  	$( $x: FromToNativeWasmType, )*
  	Rets: WasmTypeList,
  	RetsAsResult: IntoResult<Rets>,
  	Func: Fn(FunctionEnvMut<T>, $( $x , )*) -> RetsAsResult + 'static,
    {
  	let mut store = StoreMut::from_raw(env.raw_store as *mut _);
  	let result = wasmer_vm::on_host_stack(|| {
  	    panic::catch_unwind(AssertUnwindSafe(|| {
  	        $(
  	            let $x = FromToNativeWasmType::from_native(NativeWasmTypeInto::from_abi(&mut store, $x));
  	        )*
  	        let store_mut = StoreMut::from_raw(env.raw_store as *mut _);
  	        let f_env = crate::rt::sys::function::env::FunctionEnvMut {
  	            store_mut,
  	            func_env: env.env.as_sys().clone(),
  	        }.into();
  	        (env.func)(f_env, $($x),* ).into_result()
  	    }))
  	});

  	match result {
  	    Ok(Ok(result)) => return result.into_c_struct(&mut store),
  	    Ok(Err(trap)) => wasmer_vm::raise_user_trap(Box::new(trap)),
  	    Err(panic) => wasmer_vm::resume_panic(panic) ,
  	}
    }
    func_wrapper::< T, $( $x, )* Rets, RetsAsResult, Self > as _
  }

  #[cfg(feature = "sys")]
  #[allow(non_snake_case)]
  fn sys_call_trampoline_address() -> crate::rt::sys::vm::VMTrampoline {
    unsafe extern "C" fn call_trampoline<$( $x: FromToNativeWasmType, )* Rets: WasmTypeList>
  	(
          vmctx: *mut crate::rt::sys::vm::VMContext,
          body: crate::rt::sys::vm::VMFunctionCallback,
          args: *mut RawValue,
      ) {
	 let body: unsafe extern "C" fn(vmctx: *mut crate::rt::sys::vm::VMContext, $( $x: <$x::Native as NativeWasmType>::Abi, )*) -> Rets::CStruct = std::mem::transmute(body);
  	 let mut _n = 0;
  	 $(
  	     let $x = *args.add(_n).cast();
  	     _n += 1;
  	 )*

  	 let results = body(vmctx, $( $x ),*);
  	 Rets::write_c_struct_to_ptr(results, args);
    }
	 call_trampoline::<$( $x, )* Rets> as _
  }
}

// Implement `HostFunction` for a function that has the same arity than the tuple.
#[allow(unused_parens)]
impl< $( $x, )* Rets, RetsAsResult, Func >
    crate::HostFunction<(), ( $( $x ),* ), Rets, WithoutEnv>
for
    Func
where
    $( $x: FromToNativeWasmType, )*
    Rets: WasmTypeList,
    RetsAsResult: IntoResult<Rets>,
    Func: Fn($( $x , )*) -> RetsAsResult + 'static
{

  #[cfg(feature = "wamr")]
  #[allow(non_snake_case)]
  fn wamr_function_callback(&self) -> crate::rt::wamr::vm::VMFunctionCallback {
      use crate::rt::wamr::bindings::*;
      use crate::rt::wamr::utils::convert::*;
      /// This is a function that wraps the real host
      /// function. Its address will be used inside the
      /// runtime.
      unsafe extern "C" fn func_wrapper<$( $x, )* Rets, RetsAsResult, Func>(env: *mut c_void, args: *const wasm_val_vec_t, results: *mut wasm_val_vec_t) -> *mut wasm_trap_t
      where
          $( $x: FromToNativeWasmType, )*
          Rets: WasmTypeList,
          RetsAsResult: IntoResult<Rets>,
          Func: Fn($( $x , )*) -> RetsAsResult + 'static,
      {
          let mut r: *mut crate::rt::wamr::function::FunctionCallbackEnv<Func> = unsafe {std::mem::transmute(env)};
          let store = &mut (*r).store.as_store_mut();
          let mut i = 0;

          $(
              let c_arg = (*(*args).data.wrapping_add(i)).clone();
              let wasmer_arg = c_arg.into_wv();
              let raw_arg : RawValue = wasmer_arg.as_raw(store);
              let $x : $x = FromToNativeWasmType::from_native($x::Native::from_raw(store, raw_arg));

              i += 1;
          )*

          let result = panic::catch_unwind(AssertUnwindSafe(|| unsafe {
              ((*r).func)( $( $x, )* ).into_result()
          }));

          match result {
              Ok(Ok(result)) => {

                  let types = Rets::wasm_types();
                  let mut native_results = result.into_array(store);
                  let native_results = native_results.as_mut();

                  let native_results: Vec<Value> = native_results.into_iter().enumerate()
                      .map(|(i, r)| Value::from_raw(store, types[i], r.clone()))
                      .collect();

                  let mut c_results: Vec<wasm_val_t> = native_results.into_iter().map(IntoCApiValue::into_cv).collect();

                  if c_results.len() != (*results).size {
                      panic!("when calling host function: number of observed results differ from wanted results")
                  }

                  unsafe {
                      for i in 0..(*results).size {
                          *((*results).data.wrapping_add(i)) = c_results[i]
                      }
                  }

                   unsafe { std::ptr::null_mut() }
              },

              Ok(Err(e)) => {
                  let trap =  crate::rt::wamr::error::Trap::user(Box::new(e));
                  unsafe { trap.into_wasm_trap(store) }
                  // unimplemented!("host function panicked");
              },

              Err(e) => {
                  unimplemented!("host function panicked");
              }
          }
      }
      func_wrapper::< $( $x, )* Rets, RetsAsResult, Self > as _
  }

  #[cfg(feature = "v8")]
  #[allow(non_snake_case)]
  fn v8_function_callback(&self) -> crate::rt::v8::vm::VMFunctionCallback {
      use crate::rt::v8::bindings::*;
      use crate::rt::v8::utils::convert::*;

  	unsafe extern "C" fn func_wrapper<$( $x, )* Rets, RetsAsResult, Func>(env: *mut c_void, args: *const wasm_val_t, results: *mut wasm_val_t) -> *mut wasm_trap_t
  	where
  	  $( $x: FromToNativeWasmType, )*
  	  Rets: WasmTypeList,
  	  RetsAsResult: IntoResult<Rets>,
  	  Func: Fn($( $x , )*) -> RetsAsResult + 'static,
      {
		let mut r: *mut crate::rt::v8::function::FunctionCallbackEnv<Func> = unsafe {std::mem::transmute(env)};
  	  	let store = &mut (*r).store.as_store_mut();
  	  	let mut i = 0;

        $(
		let c_arg = (*(args).wrapping_add(i)).clone();
  	  	let wasmer_arg = c_arg.into_wv();
  	  	let raw_arg : RawValue = wasmer_arg.as_raw(store);
  	  	let $x : $x = FromToNativeWasmType::from_native($x::Native::from_raw(store, raw_arg));
        i += 1;
        )*

        let result = panic::catch_unwind(AssertUnwindSafe(|| unsafe { ((*r).func)( $( $x, )* ).into_result() }));

        match result {
		  Ok(Ok(result)) => {
  		    let types = Rets::wasm_types();
  		    let size = types.len();
  		    let mut native_results = result.into_array(store);
  		    let native_results = native_results.as_mut();
  		    let native_results: Vec<Value> = native_results.into_iter().enumerate()
  		  	.map(|(i, r)| Value::from_raw(store, types[i], r.clone()))
  		  	.collect();
  		    let mut c_results: Vec<wasm_val_t> = native_results.into_iter().map(|r| r.into_cv()).collect();

  		    if c_results.len() != size {
  		  	panic!("when calling host function: number of observed results differ from wanted results")
  		    }

  		    unsafe {
  		  	for i in 0..size {
  		  	  *((results).wrapping_add(i)) = c_results[i]
  		  	}
  		    }
  		    unsafe { std::ptr::null_mut() }
  		  },
		  Ok(Err(e)) => {
  		      let trap: crate::rt::v8::error::Trap =  crate::rt::v8::error::Trap::user(Box::new(e));
  		      unsafe { trap.into_wasm_trap(store) }
  		      // unimplemented!("host function panicked");
  		  },
		  Err(e) => {
  		      unimplemented!("host function panicked");
  		  }
		}
	 }
  	func_wrapper::< $( $x, )* Rets, RetsAsResult, Self > as _
  }

  #[cfg(feature = "sys")]
  #[allow(non_snake_case)]
  fn sys_function_callback(&self) -> crate::rt::sys::vm::VMFunctionCallback {
	unsafe extern "C" fn func_wrapper<$( $x, )* Rets, RetsAsResult, Func>( env: &crate::rt::sys::function::StaticFunction<Func, ()>, $( $x: <$x::Native as NativeWasmType>::Abi, )* ) -> Rets::CStruct
    where
	  $( $x: FromToNativeWasmType, )*
	  Rets: WasmTypeList,
	  RetsAsResult: IntoResult<Rets>,
	  Func: Fn($( $x , )*) -> RetsAsResult + 'static,
    {
	  let mut store = StoreMut::from_raw(env.raw_store as *mut _);
	  let result = wasmer_vm::on_host_stack(|| {
	      panic::catch_unwind(AssertUnwindSafe(|| {
	          $( let $x = FromToNativeWasmType::from_native(NativeWasmTypeInto::from_abi(&mut store, $x));)*
	          (env.func)($($x),*).into_result()
	      }))
	  });

	  match result {
	      Ok(Ok(result)) => return result.into_c_struct(&mut store),
	      Ok(Err(trap)) => wasmer_vm::raise_user_trap(Box::new(trap)),
	      Err(panic) => wasmer_vm::resume_panic(panic) ,
	  }
    }
    func_wrapper::< $( $x, )* Rets, RetsAsResult, Self > as _
  }

  #[cfg(feature = "sys")]
  #[allow(non_snake_case)]
  fn sys_call_trampoline_address() -> crate::rt::sys::vm::VMTrampoline {
	unsafe extern "C" fn call_trampoline<$( $x: FromToNativeWasmType, )* Rets: WasmTypeList>(
          vmctx: *mut crate::rt::sys::vm::VMContext,
          body: crate::rt::sys::vm::VMFunctionCallback,
          args: *mut RawValue,
    ) {
	  let body: unsafe extern "C" fn(vmctx: *mut crate::rt::sys::vm::VMContext, $( $x: <$x::Native as NativeWasmType>::Abi, )*) -> Rets::CStruct = std::mem::transmute(body);
	  let mut _n = 0;
	  $(
	  let $x = *args.add(_n).cast();
	  _n += 1;
	  )*

	  let results = body(vmctx, $( $x ),*);
	  Rets::write_c_struct_to_ptr(results, args);
    }
	  call_trampoline::<$( $x, )* Rets> as _
  }
}
    };
}

// Black-magic to count the number of identifiers at compile-time.
macro_rules! count_idents {
    ( $($idents:ident),* ) => {
        {
            #[allow(dead_code, non_camel_case_types)]
            enum Idents { $( $idents, )* __CountIdentsLast }
            const COUNT: usize = Idents::__CountIdentsLast as usize;
            COUNT
        }
    };
}

// Here we go! Let's generate all the C struct, `WasmTypeList`
// implementations and `HostFunction` implementations.
impl_host_function!([C] S0,);
impl_host_function!([transparent] S1, A1);
impl_host_function!([C] S2, A1, A2);
impl_host_function!([C] S3, A1, A2, A3);
impl_host_function!([C] S4, A1, A2, A3, A4);
impl_host_function!([C] S5, A1, A2, A3, A4, A5);
impl_host_function!([C] S6, A1, A2, A3, A4, A5, A6);
impl_host_function!([C] S7, A1, A2, A3, A4, A5, A6, A7);
impl_host_function!([C] S8, A1, A2, A3, A4, A5, A6, A7, A8);
impl_host_function!([C] S9, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_host_function!([C] S10, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
impl_host_function!([C] S11, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
impl_host_function!([C] S12, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);
impl_host_function!([C] S13, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13);
impl_host_function!([C] S14, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14);
impl_host_function!([C] S15, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15);
impl_host_function!([C] S16, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16);
impl_host_function!([C] S17, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17);
impl_host_function!([C] S18, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18);
impl_host_function!([C] S19, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19);
impl_host_function!([C] S20, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20);
impl_host_function!([C] S21, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21);
impl_host_function!([C] S22, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21, A22);
impl_host_function!([C] S23, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21, A22, A23);
impl_host_function!([C] S24, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21, A22, A23, A24);
impl_host_function!([C] S25, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21, A22, A23, A24, A25);
impl_host_function!([C] S26, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21, A22, A23, A24, A25, A26);