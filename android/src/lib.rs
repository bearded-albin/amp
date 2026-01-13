/// Android FFI bindings for Flutter
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::runtime::Runtime;

use amp_core::geolocation::GeolocationService;
use amp_core::models::GpsCoordinate;

/// Global runtime for async operations
lazy_static::lazy_static! {
    static ref RT: Runtime = Runtime::new().unwrap();
}

/// Global geolocation service
lazy_static::lazy_static! {
    static ref GEO_SERVICE: Arc<RwLock<Option<GeolocationService>>> =
        Arc::new(RwLock::new(None));
}

/// Initialize geolocation service
#[no_mangle]
pub extern "C" fn Java_com_amp_MainActivity_initGeoService(
    env: JNIEnv,
    _class: JClass,
) -> jint {
    let mut service = GEO_SERVICE.write();
    *service = Some(GeolocationService::new());
    0
}

/// Get address from GPS coordinates
#[no_mangle]
pub extern "C" fn Java_com_amp_MainActivity_getAddressFromGPS(
    env: JNIEnv,
    _class: JClass,
    lat: f64,
    lon: f64,
) -> jstring {
    let service = GEO_SERVICE.read();

    if let Some(geo) = service.as_ref() {
        let coord = match GpsCoordinate::new(
            &lat.to_string(),
            &lon.to_string(),
        ) {
            Ok(c) => c,
            Err(e) => {
                let error_msg = env
                    .new_string(format!("Invalid coordinate: {}", e))
                    .unwrap();
                return error_msg.into_inner();
            }
        };

        let address = RT.block_on(async {
            geo.get_address_from_gps(&coord, "sv").await
        });

        match address {
            Ok(addr) => {
                let output = env.new_string(addr).unwrap();
                output.into_inner()
            }
            Err(e) => {
                let error_msg = env
                    .new_string(format!("Geocoding failed: {}", e))
                    .unwrap();
                error_msg.into_inner()
            }
        }
    } else {
        let error_msg = env.new_string("Service not initialized").unwrap();
        error_msg.into_inner()
    }
}

/// Send notification (FFI for Flutter)
#[no_mangle]
pub extern "C" fn send_notification(
    title: *const u8,
    message: *const u8,
    hours_until: i32,
) -> i32 {
    if title.is_null() || message.is_null() {
        return -1;
    }

    unsafe {
        let title_str = std::ffi::CStr::from_ptr(title as *const i8)
            .to_string_lossy()
            .to_string();
        let message_str = std::ffi::CStr::from_ptr(message as *const i8)
            .to_string_lossy()
            .to_string();

        eprintln!("📢 Notification: {} - {} ({}h)", title_str, message_str, hours_until);
        0
    }
}
