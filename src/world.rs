use crate::prelude::*;

pub type BackgroundShader = Box<dyn Fn(&Ray) -> ColorRgb + Send + Sync>;

pub struct World {
  objects: Aggregate,
  background_shader: BackgroundShader,
}

impl World {
  /// BackgroundShader: Box<dyn Fn(&Ray) -> ColorRgb + Send + Sync>
  pub fn new_with(background_shader: BackgroundShader) -> Self {
    World {
      objects: Aggregate::default(),
      background_shader,
    }
  }

  pub fn new<F>(shader: F) -> Self
  where
    F: Fn(&Ray) -> ColorRgb + Send + Sync + 'static,
  {
    World::new_with(Box::new(shader))
  }

  pub fn objects(&self) -> &Aggregate {
    &self.objects
  }

  pub fn background_shader(&self) -> &BackgroundShader {
    &self.background_shader
  }
  // blue to white
  pub fn default_background_shader() -> BackgroundShader {
    Box::new(|_ray: &Ray| {
      // let t = 0.5 * (ray.direction().normalize().y + 1.0);
      let t = 1.0;
      ColorRgb::lerp(ColorRgb::WHITE, ColorRgb::SKYBLUE, t)
    })
  }
  pub fn add_object(&mut self, object: Arc<dyn Hittable>) {
    self.objects.add_object(object);
  }

  pub fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
    self.objects.hit(ray, t_min, t_max)
  }
  /// Pack existing objects with BVH strategy.
  pub fn bvh_finalize(&mut self) {
    self.objects.bvh_accelerate();
  }
}

impl Default for World {
  fn default() -> Self {
    Self::new_with(Self::default_background_shader())
  }
}
