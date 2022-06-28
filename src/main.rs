use wasmtime::{Config, Engine, Instance, Module, Store};

fn main() {
    let engine = Engine::new(&Config::new().epoch_interruption(true)).unwrap();
    let module = Module::new(
        &engine,
        include_bytes!("../guest/target/wasm32-unknown-unknown/debug/guest.wasm"),
    )
    .unwrap();
    let mut store = Store::new(&engine, ());

    // store.set_epoch_deadline(100000);

    let instance = Instance::new(&mut store, &module, &[]).unwrap();
    let instance_foo = instance
        .get_typed_func::<(), u32, _>(&mut store, "foo")
        .unwrap();

    let foo_num = instance_foo.call(&mut store, ()).unwrap();
    println!("{foo_num}");
}
