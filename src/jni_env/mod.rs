use jni::{
    errors::{Error, Result},
    objects::JString,
    JNIEnv,
};

use crate::Sealed;

impl Sealed for JNIEnv<'_> {}

trait JNIEnvInternalExt<'local> {
    unsafe fn try_jni_vtable(&self) -> Result<&'local jni::sys::JNINativeInterface_>;
}

impl<'local> JNIEnvInternalExt<'local> for JNIEnv<'local> {
    unsafe fn try_jni_vtable(&self) -> Result<&'local jni::sys::JNINativeInterface_> {
        (*self.get_native_interface())
            .as_ref()
            .ok_or(Error::NullPtr("*JNIEnv"))
    }
}

pub trait JNIEnvExt<'local>: crate::Sealed {
    fn get_string_region<'other_local: 'obj_ref, 'obj_ref>(
        &self,
        obj: &'obj_ref JString<'other_local>,
    ) -> Result<String>;
}

impl<'local> JNIEnvExt<'local> for JNIEnv<'local> {
    fn get_string_region<'other_local: 'obj_ref, 'obj_ref>(
        &self,
        obj: &'obj_ref JString<'other_local>,
    ) -> Result<String> {
        if obj.is_null() {
            Err(Error::NullPtr("get_string_region obj argument"))?;
        }

        let len = match unsafe { self.try_jni_vtable() }?.GetStringUTFLength {
            Some(r#fn) => unsafe { r#fn(self.get_raw(), obj.as_raw()) },
            None => Err(Error::JNIEnvMethodNotFound("GetStringUTFLength"))?,
        };

        let mut buf = vec![0; len as usize];

        match unsafe { self.try_jni_vtable() }?.GetStringUTFRegion {
            Some(r#fn) => unsafe {
                r#fn(
                    self.get_raw(),
                    obj.as_raw(),
                    0,
                    len,
                    buf.as_mut_ptr().cast(),
                )
            },
            None => Err(Error::JNIEnvMethodNotFound("GetStringUTFRegion"))?,
        };

        Ok(unsafe { String::from_utf8_unchecked(buf) })
    }
}
