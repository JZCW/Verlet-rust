use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};
use tiny_skia::Color;

//------------------------------------------------------------------------------
// Vector2f
//------------------------------------------------------------------------------
#[derive(Copy, Clone)]
pub struct Vector2f {
  pub x:f32,
  pub y:f32
}

impl Vector2f {
  pub fn set_zero(&mut self) {
    self.x = 0.;
    self.y = 0.;
  }

  pub fn get_mod(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }

  pub fn get_unit(&self) -> Self {
    let dist = self.get_mod();
    Vector2f { x: self.x/dist, y: self.y/dist }
  }
}

impl Add for Vector2f {
  type Output = Self;

  fn add(self, other: Self) -> Self {
      Self {x: self.x + other.x, y: self.y + other.y}
  }
}

impl AddAssign for Vector2f {
  fn add_assign(&mut self, other: Self) {
      *self = *self + other;
  }
}

impl Sub for Vector2f {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
      Self {x: self.x - other.x, y: self.y - other.y}
  }
}

impl SubAssign for Vector2f {
  fn sub_assign(&mut self, other: Self) {
    *self = *self - other;
  }
}

impl Mul<f32> for Vector2f {
  type Output = Self;

  fn mul(self, other: f32) -> Self {
      Self {x: self.x * other, y: self.y * other}
  }
}

impl Div<f32> for Vector2f {
  type Output = Self;

  fn div(self, other: f32) -> Self {
      Self {x: self.x / other, y: self.y / other}
  }
}
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
// Cobject
//------------------------------------------------------------------------------
pub struct Cobject {
  pub position  :Vector2f, // 新位置
  position_last :Vector2f, // 当前位置
  acceleration  :Vector2f, // 加速度
  pub radius    :f32,      // 小球半径
  pub color     :Color     // 小球颜色
}

impl Cobject {
  pub fn new(position:Vector2f, radius:f32, color:Color) -> Self {
    let position_last = position.clone();
    let acceleration = Vector2f{x:0.,y:0.};
    Cobject {position, position_last, acceleration, radius, color}
  }

  // 添加一个加速度
  pub fn accelerate(&mut self, a:&Vector2f) {
    self.acceleration += *a;
  }

  // 设置初速度
  pub fn set_velocity(&mut self, v:&Vector2f, dt:f32) {
    self.position_last = self.position - (*v)*dt;
  }

  // 添加一个速度
  pub fn add_velocity(&mut self, v:&Vector2f, dt:f32) {
    self.position_last -= (*v)*dt;
  }

  // 获取当前速度
  pub fn get_velocity(&self, dt:f32) -> Vector2f {
    return (self.position - self.position_last)/dt;
  }

  // 更新状态
  pub fn update(&mut self, dt:f32) {
    // 计算位移
    let displacement = self.position - self.position_last;
    // 更新位置
    self.position_last = self.position;
    self.position      = self.position + displacement + self.acceleration * dt.powi(2); // 移位 + 外力作用？
    // 重置加速度
    self.acceleration.set_zero();
  }
}
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
// Slover
//------------------------------------------------------------------------------
pub struct Slover {
  objects               :Vec<Cobject>, // 对象列表
  pub sub_step          :u32,          // 帧步进
  pub step_dt           :f32,          // 步进时间
  pub gravity           :Vector2f,     // 重力加速度
  pub constraint_center :Vector2f,     // 约束中心（圆形）
  pub constraint_radius :f32           // 约束半径
}

impl Slover {
  pub fn new() -> Self {
    Slover {
      objects:           Vec::new(),
      sub_step:          4,
      step_dt:           1.,
      gravity:           Vector2f { x: 0., y: 0. },
      constraint_center: Vector2f { x: 0., y: 0. },
      constraint_radius: 100.
    }
  }

  // 添加一个对象
  pub fn add_object(&mut self, position:Vector2f, radius:f32, velocity:Vector2f, color:Color) -> &Cobject {
    let mut obj = Cobject::new(position, radius, color);
    obj.set_velocity(&velocity, self.step_dt);
    self.objects.push(obj);
    return self.objects.last_mut().unwrap();
  }

  // 获取全部对象
  pub fn get_objects(&self) -> &Vec<Cobject> {
    return &self.objects;
  }

  // 更新状态
  pub fn update(&mut self) {
    for _i in 0..self.sub_step {
      self.apply_gravity();
      self.check_collisions();
      self.apply_constraint();
      self.update_objects();
    }
  }

  // 检测碰撞
  fn check_collisions(&mut self) {
    const RESPONSE_COEF:f32 = 1.2; //？

    let len = self.objects.len();
    if len>0 {
      // 嵌套循环 遍历每个对象及其之后全部对象
      for i in 0..(len-1) {
        let (_prev, current_and_end) = self.objects.split_at_mut(i);
        let (current, end) = current_and_end.split_at_mut(1);
        let object1 = &mut current[0];
        for object2 in end {
          let v        = object1.position - object2.position;                      // 两个对象的中点偏差
          let dist     = v.get_mod();                                              // 两个对象的距离  //FIXME 0？
          let min_dist = object1.radius + object2.radius;                          // 两个对象的半径和
          if dist < min_dist {  // 检查是否重叠
            let n            = v.get_unit();                                       // 单位向量
            let mass_ratio_1 = object1.radius / (object1.radius + object2.radius);
            let mass_ratio_2 = object2.radius / (object1.radius + object2.radius);
            let delta        = 0.5 * RESPONSE_COEF * (dist - min_dist);            // 平均位移
            // 更新位置
            object1.position -= n * (mass_ratio_2 * delta);
            object2.position += n * (mass_ratio_1 * delta);
          }
        }
      }
    }
  }

  // 边框碰撞
  fn apply_constraint(&mut self) {
    for item in self.objects.iter_mut() {
      let v               = self.constraint_center - item.position;
      let dist            = v.get_mod();
      let constraint_dist = self.constraint_radius - item.radius;
      if dist > constraint_dist { //TODO 实现反弹？
        item.position = self.constraint_center - v.get_unit() * constraint_dist;
      }
    }
  }

  // 添加重力
  fn apply_gravity(&mut self) {
    for item in self.objects.iter_mut() {
      item.accelerate(&self.gravity);
    }
  }

  // 更新对象
  fn update_objects(&mut self) {
    for item in self.objects.iter_mut() {
      item.update(self.step_dt);
    }
  }
}
