use crate::prelude::*;
pub struct Aggregate {
  objects: Vec<Arc<dyn Hittable>>,
  bvh_acc: Option<Arc<dyn Hittable>>,
}

impl Aggregate {
  pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Aggregate {
    Self { objects, bvh_acc: None }
  }
  pub fn add_object(&mut self, object: Arc<dyn Hittable>) {
    self.objects.push(object);
    self.bvh_acc = None;
  }
  pub fn clear(&mut self) {
    self.objects.clear();
    self.bvh_acc = None;
  }

  /// Pack existing objects with BVH strategy.
  pub fn bvh_accelerate(&mut self) {
    if self.bvh_acc.is_none() {
      self.bvh_acc = Some(BvhAggregate::build(self.objects.clone()));
    }
  }
}

impl Default for Aggregate {
  fn default() -> Self {
    Self::new(Vec::new())
  }
}

impl Hittable for Aggregate {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    if let Some(ref bvh) = self.bvh_acc {
      return bvh.hit(ray, t_min, t_max);
    }
    let mut closest_so_far = t_max;
    let mut closest_hit_record = None;
    for object in &self.objects {
      if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
        closest_so_far = hit_record.hit_t;
        closest_hit_record = Some(hit_record);
      }
    }
    closest_hit_record
  }
  fn bounding_box(&self) -> Aabb {
    if self.objects.is_empty() {
      return Aabb::default();
    }
    if let Some(ref bvh) = self.bvh_acc {
      return bvh.bounding_box();
    }
    let mut output = Aabb::default();
    for object in &self.objects {
      output = Aabb::union(output, object.bounding_box());
    }
    output
  }
}