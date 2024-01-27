use crate::slover;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};

//------------------------------------------------------------------------------
// renderer
//------------------------------------------------------------------------------
pub fn renderer(objects:&Vec<slover::Cobject>) -> Image {
  let mut frame_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(800, 800);
  let w = frame_buffer.width();
  let h = frame_buffer.height();
  let mut pixmap = tiny_skia::PixmapMut::from_bytes(frame_buffer.make_mut_bytes(), w, h).unwrap();
  pixmap.fill(tiny_skia::Color::TRANSPARENT);
  //pixmap.fill(tiny_skia::Color::BLACK);

  for item in objects.iter() { 
    let mut paint1 = tiny_skia::Paint::default();
    paint1.set_color(item.color.clone());
    paint1.anti_alias = true;

    let r = item.radius.clone();
    let x = item.position.x - r;
    let y = item.position.y;
    let circle = tiny_skia::PathBuilder::from_circle(x,y,r).unwrap();
  
    pixmap.fill_path(&circle, &paint1, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None);
  }

  Image::from_rgba8_premultiplied(frame_buffer)
}

//------------------------------------------------------------------------------
// rainbow_iter
//------------------------------------------------------------------------------
pub struct Rainbow {
  counter:f32,
  alpha:f32
}

impl Iterator for Rainbow {
  type Item = tiny_skia::Color;
  
  fn next(&mut self) -> Option<tiny_skia::Color> {
      let r = self.counter.sin().powi(2);
      let g = (self.counter + 0.33*2.*3.14).sin().powi(2);
      let b = (self.counter + 0.66*2.*3.14).sin().powi(2);
      self.counter += 0.08;
      Some(tiny_skia::Color::from_rgba(r, g, b, self.alpha)?)
  }
}

pub fn rainbow_iter(alpha:f32) -> Rainbow {
  Rainbow { counter: 0., alpha: alpha }
}
