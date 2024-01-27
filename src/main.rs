slint::include_modules!();

mod slover;
mod renderer;

use slover::Vector2f;

fn main() {
  // 参数定义
  const WINDOW_W              :i32      = 1920;                  // 窗口宽
  const WINDOW_H              :i32      = 1080;                  // 窗口高
  const CONSTRAINT_RADIUS     :i32      = 400;                   // 约束区域半径(圆形)
  const FRAME_RATE            :u32      = 60;                    // 最大帧率
  const SUB_STEP              :u32      = 8;                     // 子步进
  const GRAVITY               :Vector2f = Vector2f{x:0.,y:0.01}; // 重力
  const OBJECT_SPAWN_DELAY    :u32      = 1;                     // 对象生成间隔(frame)
  const OBJECT_SPAWN_POSITION :Vector2f = Vector2f{x:CONSTRAINT_RADIUS as f32,y:(CONSTRAINT_RADIUS as f32)*0.4};  // 新对象生成点
  const OBJECT_SPAWN_SPEED    :Vector2f = Vector2f{x:1.,y:-1.};  // 对象初始速度
  // const OBJECT_MIN_RADIUS     :f32      = 1.0;                   // 对象尺寸最小值
  // const OBJECT_MAX_RADIUS     :f32      = 20.0;                  // 对象尺寸最大值
  const OBJECT_RADIUS         :f32      = 8.0;                   // 对象尺寸
  const MAX_OBJECTS_COUNT     :usize    = 4000;                  // 最大对象数
  const FRAME_TIME            :u32      = 1000/FRAME_RATE;       // 帧时间(ms)
  const FRAME_MIN             :f32      = FRAME_RATE as f32/2.;  // 最小帧率

  // 构建窗体
  let ui = AppWindow::new().unwrap();
  ui.set_w_width(WINDOW_W);
  ui.set_w_height(WINDOW_H);

  // 初始化主结构
  let mut s = slover::Slover::new();
  s.sub_step = SUB_STEP;
  s.step_dt  = (FRAME_TIME/SUB_STEP) as f32;
  s.gravity  = GRAVITY;
  s.constraint_radius = CONSTRAINT_RADIUS as f32;
  s.constraint_center = Vector2f{x:CONSTRAINT_RADIUS as f32 + 10. ,y:CONSTRAINT_RADIUS as f32};
  // s.add_object(Vector2f{x:400.,y:10.},10.0, Vector2f {x:0., y:0.}, tiny_skia::Color::from_rgba8(0,255,255,127));

  // 开始运行
  let ui_handle = ui.as_weak();
  let mut clock = std::time::Instant::now(); // 帧率计时器
  let mut timer_gen_obj:u32 = 0;             // 生成计时器
  let mut color = renderer::rainbow_iter(0.9); // 颜色生成器
  // 帧循环
  let t = slint::Timer::default();
  t.start(
    slint::TimerMode::Repeated, 
    std::time::Duration::from_millis(FRAME_TIME.into()),  // 限制刷新率为最大帧率
    move || {
      // 计算当前帧率
      let rate = (1000/clock.elapsed().as_millis()) as f32; 
      clock = std::time::Instant::now();

      let num = s.get_objects().len(); // 当前对象数

      // 添加新对象 (当前帧率大于最小值, 对象总数小于最大值, 满足生成间隔)
      if (rate>FRAME_MIN) && (num<MAX_OBJECTS_COUNT) && (timer_gen_obj>OBJECT_SPAWN_DELAY) {
        timer_gen_obj = 0;
        s.add_object(OBJECT_SPAWN_POSITION,OBJECT_RADIUS, OBJECT_SPAWN_SPEED, color.next().unwrap());
      }
      timer_gen_obj += 1;

      // 计算帧
      s.update();

      // 更新GUI信息
      let ui = ui_handle.unwrap();
      //  刷新文字
      ui.set_fps(rate);
      ui.set_ballnum(num.try_into().unwrap());
      //  绘制结果
      let image = renderer::renderer(s.get_objects());
      ui.set_frame(image)
    }
  );

  ui.run().unwrap();
}
