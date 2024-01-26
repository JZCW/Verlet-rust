use crate::slover::Vector2f;

slint::include_modules!();

mod slover;
mod renderer;

fn main() -> Result<(), slint::PlatformError> {
  // 参数定义
  const WINDOW_W:i32 = 1920; // 窗口宽
  const WINDOW_H:i32 = 1080; // 窗口高
  const FRAME_RATE:u32 = 60; // 最大帧率
  const SUB_STEP:u32 = 8;    // 子步进
  const GRAVITY:slover::Vector2f = slover::Vector2f{x:0.,y:0.01}; // 重力


  const FRAME_TIME:u32 = 1000/FRAME_RATE;
  //const CONSTRAINT_RADIUS:i32 = WINDOW_H/2;
  const CONSTRAINT_RADIUS:i32 = 400;

  use slint::{Color, Model, Timer, TimerMode, SharedPixelBuffer, Rgba8Pixel, Image};

  let ui = AppWindow::new()?; // 构建窗体
  ui.set_w_width(WINDOW_W);
  ui.set_w_height(WINDOW_H);


  let mut s = slover::Slover::new();
  s.sub_step = SUB_STEP;
  s.step_dt  = (FRAME_TIME/SUB_STEP) as f32;
  s.gravity  = GRAVITY;
  s.constraint_radius = CONSTRAINT_RADIUS as f32;
  s.constraint_center = slover::Vector2f{x:CONSTRAINT_RADIUS as f32 ,y:CONSTRAINT_RADIUS as f32};
  s.addObject(slover::Vector2f{x:400.,y:10.},10.0,tiny_skia::Color::from_rgba8(0,255,255,127));
  s.addObject(slover::Vector2f{x:450.,y:80.},8.0,tiny_skia::Color::from_rgba8(255,0,255,127));

  let ui_handle = ui.as_weak();
  let mut clock = std::time::Instant::now();
  //let mut ball_cnt = 0;
  let frametimer = slint::Timer::default();
  frametimer.start(TimerMode::Repeated, std::time::Duration::from_millis(FRAME_TIME.into()), move || {
    // 计算当前帧率
    let rate = (1000/clock.elapsed().as_millis()) as f32; 
    clock = std::time::Instant::now();

    let ui = ui_handle.unwrap();

    // 刷新文字
    ui.set_fps(rate);
    let num = s.getObjects().len();
    ui.set_ballnum(num.try_into().unwrap());

    // 计算帧
    s.update();
    // 绘制结果
    let image = renderer::renderer(s.getObjects());
    ui.set_frame(image)

  });

  ui.run()
}
