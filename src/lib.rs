// start
use imageproc::{drawing};
use rusttype::{Font, Scale};
use base64::{decode, encode};
use image::{GenericImage,GenericImageView};
use wasm_bindgen::prelude::*;
use wee_alloc; 

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  println!("The Rust function say() received {}", s);
  let r = String::from("hello ");
  return r + s;
}
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn myWatermark(watermark: &str,img_buf: &[u8]) -> String {

  let mut img = image::load_from_memory(img_buf).unwrap();
  let (w,h) = img.dimensions();
  println!("W {} H {}",w,h);
  let scale = Scale {
      x: w as f32 /10.0,
      y: h as f32 /10.0,
  };
  let font = Vec::from(include_bytes!("../fonts/adelia.ttf") as &[u8]);
  let font = Font::try_from_vec(font).unwrap();

  drawing::draw_text_mut(&mut img, image::Rgba([255u8, 255u8, 255u8, 255u8]), 0+(h/10),h/2, scale, &font, watermark);
  drawing::draw_text_mut(&mut img, image::Rgba([0u8, 0u8, 0u8, 0u8]), 5+(h/10),(h/2)+5, scale, &font, watermark);
  let mut buf = vec![];
  img.write_to(&mut buf, image::ImageOutputFormat::Png);
  let r64 = base64::encode(&buf);
  r64

}
