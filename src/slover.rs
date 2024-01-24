pub struct Vector2f {
  pub x:f32, // TODO prv
  pub y:f32
}

impl  Vector2f {
    pub fn clone(&self) -> Vector2f {
      return Vector2f{x:self.x.clone(),y:self.y.clone()};
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
}
//------------------------------------------------------------------------------

pub struct Slover {
  objects:Vec<Cobject> // 对象列表
}

impl Slover {
  pub fn new() -> Slover {
    Slover {
      objects : Vec::new()
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
}