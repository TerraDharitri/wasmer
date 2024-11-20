use wasmer::*;

#[test]
#[cfg(any(feature = "wamr", feature = "v8"))]
fn can_create_multiple_engines() {
    let _: Engine = Cranelift::new().into();

    #[cfg(feature = "v8")]
    {
        let _: Engine = V8::new().into();
    }

    #[cfg(feature = "wamr")]
    {
        let _: Engine = Wamr::new().into();
    }
}

#[test]
#[cfg(feature = "v8")]
fn multiple_engines_can_run_together() {
    use std::u8;

    let clift: Engine = Cranelift::new().into();
    let mut clift_store = Store::new(clift);
    let c_hello = Runtime::new_typed(&mut clift_store, move || {
        println!("hello from cranelift!");
    });

    #[cfg(feature = "v8")]
    {
        let v8: Engine = V8::new().into();
        let mut v8_store = Store::new(v8);
        let v8_hello = Runtime::new_typed(&mut v8_store, move || {
            println!("hello from v8!");
        });
        c_hello.call(&mut clift_store, &[]).unwrap();
        v8_hello.call(&mut v8_store, &[]).unwrap();
    }
}