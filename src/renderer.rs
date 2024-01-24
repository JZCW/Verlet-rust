//mod slover;

use crate::slover;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, StandardListViewItem};

pub fn renderer(objects:&Vec<slover::Cobject>) -> slint::Image {

  let mut frame_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(800, 800);
  let w = frame_buffer.width();
  let h = frame_buffer.height();
  let mut pixmap = tiny_skia::PixmapMut::from_bytes(frame_buffer.make_mut_bytes(), w, h).unwrap();
  pixmap.fill(tiny_skia::Color::TRANSPARENT);


  for item in objects.iter() { 
    let mut paint1 = tiny_skia::Paint::default();
    paint1.set_color(item.color.clone());
    paint1.anti_alias = true;

    let r = item.radius.clone();
    let x = item.position.x - r;
    let y = item.position.y - r;
    let circle = tiny_skia::PathBuilder::from_circle(x,y,r).unwrap();
  
    pixmap.fill_path(&circle, &paint1, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None);
  }

  let image = slint::Image::from_rgba8_premultiplied(frame_buffer);

  return image;
}