slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
  // 参数定义
  const WINDOW_W:i32 = 1920; // 窗口宽
  const WINDOW_H:i32 = 1080; // 窗口高
  const FRAME_RATE:u32 = 60; // 最大帧率

  const FRAME_TIME:u32 = 1000/FRAME_RATE;

  use slint::{Color, Model, Timer, TimerMode, SharedPixelBuffer, Rgba8Pixel, Image};

  let ui = AppWindow::new()?; // 构建窗体
  ui.set_w_width(WINDOW_W);
  ui.set_w_height(WINDOW_H);

  let ui_handle = ui.as_weak();
  let mut clock = std::time::Instant::now();
  let mut ball_cnt = 0;
  let frametimer = slint::Timer::default();
  frametimer.start(TimerMode::Repeated, std::time::Duration::from_millis(FRAME_TIME.into()), move || {
    // 计算当前帧率
    let rate = (1000/clock.elapsed().as_millis()) as f32; 
    clock = std::time::Instant::now();

    let ui = ui_handle.unwrap();

    // 刷新文字
    ui.set_fps(rate);
    ui.set_ballnum(ball_cnt);

    let mut frame_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(800, 800);
    let mut pixmap = tiny_skia::PixmapMut::from_bytes(frame_buffer.make_mut_bytes(), frame_buffer.width(), frame_buffer.height()).unwrap();
    pixmap.fill(tiny_skia::Color::TRANSPARENT);
    
    let circle = tiny_skia::PathBuilder::from_circle(320., 240., 150.).unwrap();
    
    let mut paint = tiny_skia::Paint::default();
    paint.shader = tiny_skia::LinearGradient::new(
        tiny_skia::Point::from_xy(100.0, 100.0),
        tiny_skia::Point::from_xy(400.0, 400.0),
        vec![
            tiny_skia::GradientStop::new(0.0, tiny_skia::Color::from_rgba8(50, 127, 150, 200)),
            tiny_skia::GradientStop::new(1.0, tiny_skia::Color::from_rgba8(220, 140, 75, 180)),
        ],
        tiny_skia::SpreadMode::Pad,
        tiny_skia::Transform::identity(),
    ).unwrap();
    
    pixmap.fill_path(&circle, &paint, tiny_skia::FillRule::Winding, Default::default(), None);
    
    let image = Image::from_rgba8_premultiplied(frame_buffer);
    ui.set_frame(image)

  });

  ui.run()
}