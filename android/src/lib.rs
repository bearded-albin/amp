use jni::{
    objects::{JClass, JString},
    JNIEnv,
    sys::{jint}
};
use lazy_static::lazy_static;
use std::sync::Mutex;
use amp_core::AppState;

lazy_static! {
    static ref APP_STATE: Mutex<AppState> = Mutex::new(AppState::new());
}

#[no_mangle]
pub extern "C" fn Java_com_amp_MainActivity_initApp(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    APP_STATE.lock().unwrap().count() as i32
}

#[no_mangle]
pub extern "C" fn Java_com_amp_MainActivity_addAddress(
    mut env: JNIEnv,
    _class: JClass,
    address_ptr: JString,
) -> jint {
    match env.get_string(&address_ptr) {
        Ok(address_java) => {
            let address = address_java.to_string_lossy().to_string();
            tracing::info!("Added address: {}", address);
            0
        }
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn Java_com_amp_MainActivity_getAddressCount(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    APP_STATE.lock().unwrap().count() as i32
}

#[no_mangle]
pub extern "C" fn Java_com_amp_MainActivity_clearAll(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    APP_STATE.lock().unwrap().clear_all();
    0
}
