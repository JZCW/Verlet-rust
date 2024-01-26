use slint::include_modules;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};

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

  // 添加一个加速度
  pub fn accelerate(&mut self, a:&Vector2f) {
    self.acceleration += *a;
  }

  // 设置初速度
  pub fn setVelocity(&mut self, v:&Vector2f, dt:f32) {
    self.position_last = self.position - (*v)*dt;
  }

  // 添加一个速度
  pub fn addVelocity(&mut self, v:&Vector2f, dt:f32) {
    self.position_last -= (*v)*dt;
  }

  // 获取当前速度
  pub fn getVelocity(&self, dt:f32) -> Vector2f {
    return (self.position - self.position_last)/dt;
  }

  // 更新状态
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

// fn main() {
//   let mut data = Data::new(vec![2, 3, 4]);

//   // this works
//   let slice = data.get_mut_slice(1);
//   slice[2] = 5.0;
//   println!("{:?}", data);

//   // and now this works too!
//   let mut slices = data.get_mut_slices(vec![0, 1]);
//   slices[0][0] = 2.0;
//   slices[1][0] = 3.0;
//   println!("{:?}", data);
// }

// #[derive(Debug)]
// struct Data {
//   data: Vec<Vec<f64>>,
// }

// impl Data {
//   fn new(lengths: Vec<usize>) -> Data {
//       Data {
//           data: lengths.iter().map(|n| vec![0_f64; *n]).collect(),
//       }
//   }

//   fn get_mut_slice(&mut self, index: usize) -> &mut [f64] {
//       &mut self.data[index][..]
//   }

//   // now works!
//   fn get_mut_slices(&mut self, mut indexes: Vec<usize>) -> Vec<&mut [f64]> {
//       // sort indexes for easier processing
//       indexes.sort();
//       let index_len = indexes.len();

//       // early return for edge case
//       if index_len == 0 {
//           return Vec::new();
//       }

//       // check that the largest index is in bounds
//       let max_index = indexes[index_len - 1];
//       if max_index > self.data.len() {
//           panic!("{} index is out of bounds of data", max_index);
//       }

//       // check that we have no overlapping indexes
//       indexes.dedup();
//       let uniq_index_len = indexes.len();
//       if index_len != uniq_index_len {
//           panic!("cannot return aliased mut refs to overlapping indexes");
//       }

//       // leverage the unsafe code that's written in the standard library
//       // to safely get multiple unique disjoint mutable references
//       // out of the Vec
//       let mut mut_slices_iter = self.data.iter_mut();
//       let mut mut_slices = Vec::with_capacity(index_len);
//       let mut last_index = 0;
//       for curr_index in indexes {
//           mut_slices.push(
//               mut_slices_iter
//                   .nth(curr_index - last_index)
//                   .unwrap()
//                   .as_mut_slice(),
//           );
//           last_index = curr_index;
//       }

//       // return results
//       mut_slices
//   }
// }
//------------------------------------------------------------------------------

pub struct Slover {
  objects:Vec<Cobject>, // 对象列表
  pub sub_step:u32,     // 帧步进
  pub step_dt:f32,      // 步进时间
  pub gravity:Vector2f, // 重力加速度
  pub constraint_center:Vector2f, // 约束中心（圆形）
  pub constraint_radius:f32       // 约束半径
}

impl Slover {
  pub fn new() -> Slover {
    Slover {
      objects : Vec::new(),
      sub_step: 4,
      step_dt: 1.,
      gravity: Vector2f { x: 0., y: 0. },
      constraint_center: Vector2f { x: 0., y: 0. },
      constraint_radius: 100.
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
      self.checkCollisions();
      self.applyConstraint();
      self.updateObjects();
    }
  }

  // 检测碰撞
  fn checkCollisions(&mut self) {
    const RESPONSE_COEF:f32 = 1.2; // 阻尼系数？

    let len = self.objects.len();
    if len>0 {
      for i in 0..(len-1) {
        let (_prev, current_and_end) = self.objects.split_at_mut(i);
        let (current, end) = current_and_end.split_at_mut(1);
        let object1 = &mut current[0];
        for object2 in end {
          let v = object1.position - object2.position; // 两个对象的中点偏差
          let dist2 = v.x.powi(2) + v.y.powi(2); // 两个对象距离的平方
          let min_dist = object1.radius + object2.radius;  // 两个对象的半径和
          if dist2 < min_dist.powi(2) { // 检查是否重叠
            let dist = dist2.sqrt(); // 当前距离
            let n = v/dist;     // 单位向量
            let mass_ratio_1 = object1.radius / (object1.radius + object2.radius);
            let mass_ratio_2 = object2.radius / (object1.radius + object2.radius);
            let delta = 0.5 * RESPONSE_COEF * (dist - min_dist); // 平均位移
            // 更新位置
            object1.position -= n * (mass_ratio_2 * delta);
            object2.position += n * (mass_ratio_1 * delta);
          }
        }
      }
    }
  }

  // 边框碰撞
  fn applyConstraint(&mut self) { //TODO 实现反弹？
    for item in self.objects.iter_mut() {
      let v = self.constraint_center - item.position;
      let dist = (v.x.powi(2)+v.y.powi(2)).sqrt();
      let constraint_dist = self.constraint_radius - item.radius;
      if dist > constraint_dist {
        let n = v / dist;
        item.position = self.constraint_center - n * constraint_dist;
      }
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