#![allow(non_camel_case_types)]

use libc::*;

//void stbi_write_func(void *context, void *data, int size)
pub type stbi_write_func = extern "C" fn (context: *mut c_void, data: *mut c_void, size: c_int);

#[link(name = "stb-image-write", kind = "static")]
extern "C" {
  pub fn stbi_write_png_to_func(
      //tbi_write_func *func, void *context, int w, int h, int comp, const void  *data, int stride_in_bytes
      func: stbi_write_func,
      context: *mut c_void,
      w: c_int,
      h: c_int,
      comp: c_int,
      data: *const c_void,
      stride_in_bytes: c_int,
  ) -> c_int;
}
