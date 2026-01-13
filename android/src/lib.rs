mod notifications;
mod gps;
mod ffi;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jint;

#[no_mangle]
pub extern "C" fn Java_com_amp_MainActivity_initGeoService(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    0
}

#[no_mangle]
pub extern "C" fn Java_com_amp_MainActivity_getAddressFromGPS(
    env: JNIEnv,
    _class: JClass,
    _lat: f64,
    _lon: f64,
) -> jni::sys::jstring {
    let output = env.new_string("Detected Address").unwrap();
    output.into_inner()
}
