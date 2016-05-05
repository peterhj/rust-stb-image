//#![allow(non_upper_case_globals)]
//#![allow(non_camel_case_types)]

use libc::*;

#[link(name = "stb-image-resize", kind = "static")]
extern "C" {
  // Easy API.
  pub fn stbir_resize_uint8(
      input_pixels: *const c_uchar, input_w: c_int, input_h: c_int, input_stride_in_bytes: c_int,
      output_pixels: *mut c_uchar, output_w: c_int, output_h: c_int, output_stride_in_bytes: c_int,
      num_channels: c_int,
  ) -> c_int;

  // Medium API (TODO).
  /*pub fn stbir_resize_uint8_generic(
      input_pixels: *const c_uchar, input_w: c_int, input_h: c_int, input_stride_in_bytes: c_int,
      output_pixels: *mut c_uchar, output_w: c_int, output_h: c_int, output_stride_in_bytes: c_int,
      num_channels: c_int, alpha_channel: c_int, flags: c_int,
      edge_wrap_mode: stbir_edge, filter: stbir_filter, space: stbir_colorspace,
      alloc_context: *mut c_void,
  ) -> c_int;*/

  // Full API (TODO).
}
