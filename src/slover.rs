use slint::include_modules;
use std::ops::{Add, AddAssign, Sub, Mul};

#[derive(Copy, Clone)]
pub struct Vector2f {
  pub x:f32, // TODO prv
  pub y:f32
}

// impl  Vector2f {
//     pub fn clone(&self) -> Vector2f {
//       return Vector2f{x:self.x.clone(),y:self.y.clone()};
//     }
// }

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

impl Mul<f32> for Vector2f {
  type Output = Self;

  fn mul(self, other: f32) -> Self {
      Self {x: self.x * other, y: self.y * other}
  }
}

//------------------------------------------------------------------------------

pub struct Cobject {
  pub position:Vector2f, // 新位置
  position_last:Vector2f, // 当前位置
  acceleration:Vector2f,  // 加速度
  pub radius:f32,  // 小球半径
  pub color:tiny_skia::Color // 小球颜色
}

impl Cobject {
  pub fn new(position:Vector2f, radius:f32, color:tiny_skia::Color) -> Cobject {
    let position_last = position.clone();
    Cobject {
      position,
      position_last,
      acceleration  : Vector2f{x:0.,y:0.},
      radius,
      color
    }
  }

  pub fn accelerate(&mut self, a:&Vector2f) {
    self.acceleration += *a;
  }

  pub fn update(&mut self, t:f32) {
    // 计算位移
    let displacement = self.position - self.position_last;
    // 更新位置
    self.position_last = self.position;
    self.position      = self.position + displacement + self.acceleration * (t * t); // 移位 + 外力作用？
    // 重置加速度
    self.acceleration  = Vector2f{x:0.,y:0.};
  }
}
//------------------------------------------------------------------------------

pub struct Slover {
  objects:Vec<Cobject>, // 对象列表
  pub sub_step:u32,     // 帧步进
  pub step_dt:f32,      // 步进时间
  pub gravity:Vector2f  // 重力加速度
}

impl Slover {
  pub fn new() -> Slover {
    Slover {
      objects : Vec::new(),
      sub_step: 4,
      step_dt: 1.,
      gravity: Vector2f { x: 0., y: 0. }
    }
  }

  // 添加一个对象
  pub fn addObject(&mut self, position:Vector2f, radius:f32, color:tiny_skia::Color) {
    self.objects.push(Cobject::new(position, radius, color));
  }

  // 获取全部对象
  pub fn getObjects(&self) -> &Vec<Cobject> {
    return &self.objects;
  }

  // 更新状态
  pub fn update(&mut self) {
    for _i in 0..self.sub_step { // 循环 sub_step 轮
      self.applyGravity();
      // checkCollisions(self.step_dt);
      // applyConstraint();
      self.updateObjects();
    }
  }

  // 添加重力
  fn applyGravity(&mut self) {
    for item in self.objects.iter_mut() {
      item.accelerate(&self.gravity);
    }
  }

  // 更新对象
  fn updateObjects(&mut self) {
    for item in self.objects.iter_mut() {
      item.update(self.step_dt);
    }
  }
}