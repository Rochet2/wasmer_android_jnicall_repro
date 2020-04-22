extern crate jni;
#[macro_use] extern crate log;
extern crate android_logger;

use log::Level;
use android_logger::Config;
use jni::{
    objects::JClass,
    sys::jbyteArray,
    JNIEnv
};

use wasmer_runtime::{compile, imports, Instance};

#[no_mangle]
pub unsafe extern "C" fn Java_com_wasmer_android_MainActivity_JNIExecuteWasm(
    env: JNIEnv,
    _: JClass,
    module_bytes: jbyteArray,
) {
    android_logger::init_once(
        Config::default().with_min_level(Level::Trace)
    );

    std::panic::set_hook(Box::new(|panic_info| {
        error!("ERR: {}", panic_info.to_string());
    }));

    // build module
    let module_bytes = env.convert_byte_array(module_bytes).unwrap();
    let main_instance = load_module(&module_bytes);

    // main inside wasm fails
    info!("MODULE CALL STARTING!");
    let x = main_instance.call("run", &[]);
    info!("FOOBAR: {:?}", x);
    info!("MODULE CALL FINISHED!");
}

pub fn load_module(module_bytes: &[u8]) -> Instance {
    // Compile the module.
    let module = compile(&module_bytes).unwrap();
    // Create the ImportObject with our interface imported.
    let import_object = imports! {};
    // Instantiate the module.
    let module_instance = module.instantiate(&import_object).unwrap();
    module_instance
}

