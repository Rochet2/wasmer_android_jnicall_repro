#[macro_use] extern crate lazy_static;
extern crate jni;
#[macro_use] extern crate log;
extern crate android_logger;

use log::Level;
use android_logger::Config;
use jni::{
    objects::{GlobalRef, JClass, JObject},
    sys::jbyteArray,
    JNIEnv, JavaVM,
};
use std::sync::Mutex;

use wasmtime::{Store, Module, Instance, Func};

lazy_static! {
    static ref ENV: Mutex<Option<JavaVM>> = { Mutex::new(None) };
    static ref CLASS: Mutex<Option<GlobalRef>> = { Mutex::new(None) };
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_wasmer_android_MainActivity_JNIExecuteWasm(
    env: JNIEnv,
    _: JClass,
    callback: JObject,
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

    // set global variables
    let class = env.new_global_ref(callback).unwrap();
    *CLASS.lock().unwrap() = Some(class);
    let vm = env.get_java_vm().unwrap();
    *ENV.lock().unwrap() = Some(vm);

    // Succeeds
    java_test();
    java_test();

    // fails to call java_test
    let function = main_instance
        .get_export("main")
        .unwrap()
        .func()
        .expect("`main` is not an exported function");

    function.call(&[]).expect("Couldn't call the function");

    java_test();
}

pub fn load_module(module_bytes: &[u8]) -> Instance {
    let store = Store::default();
    let module = Module::from_binary(&store, module_bytes).expect("Cannot load file from binary");
    let func = test_func(&store);
    let instance = Instance::new(&module, &[func.into()]);

    instance.expect("Couldn't create an instance")
}

fn test_func(store: &Store) -> Func {
    Func::wrap(store, || {
        java_test()
    })
}

fn java_test() {
    // Get env.
    let ovm = &*ENV.lock().unwrap();
    let vm = ovm.as_ref().unwrap();
    let env = vm.get_env().unwrap();
    // Get the class.
    let o_class = &*CLASS.lock().unwrap();
    let class_ref = o_class.as_ref().unwrap();
    let class = class_ref.as_obj();
    // Call java code.
    env.call_method(*class, "Test", "()V", &[])
        .unwrap();
}
