use jni::objects::*;
use jni::JNIEnv;

#[no_mangle]
pub unsafe extern "C" fn Java_JNI_ExcelJNI_init(env: JNIEnv, _class: JClass) {
    println!("rust-java-demo inited");
}
