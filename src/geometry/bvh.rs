use crate::prelude::*;

pub struct BvhAggregate {
  left: Arc<dyn Hittable>,
  right: Arc<dyn Hittable>,
  bbox: Aabb,
}

impl BvhAggregate {
  pub fn new(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>) -> Self {
    Self {
      bbox: Aabb::union(left.bounding_box(), right.bounding_box()),
      left,
      right,
    }
  }
  pub fn build(mut objects: Vec<Arc<dyn Hittable>>) -> Arc<dyn Hittable> {
    let axis = {
      let mut surround = Aabb::default();
      for obj in &objects {
        surround = Aabb::union(surround, obj.bounding_box());
      }
      surround.longest_axis()
    };
    let len = objects.len();
    match len {
      0 => panic!("BvhAggregate requires at least one object per frame"),
      1 => objects.pop().unwrap(),
      _ => {
        let mid = len / 2;
        objects.select_nth_unstable_by(mid, |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
          let a = a.bounding_box();
          let b = b.bounding_box();
          a.min[axis].partial_cmp(&b.min[axis]).unwrap()
        });
        let right_objects = objects.split_off(mid);
        let left_objects = objects;
        let left = Self::build(left_objects);
        let right = Self::build(right_objects);
        Arc::new(Self::new(left, right))
      }
    }
  }
}

impl Hittable for BvhAggregate {
  fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
    if !self.bbox.might_hit(ray, t_min, t_max) {
      return None;
    }
    let rec_left = self.left.hit(ray, t_min, t_max);
    if let Some(ref rec) = rec_left {
      t_max = rec.hit_t;
    }
    let rec_right = self.right.hit(ray, t_min, t_max);
    rec_right.or(rec_left)
  }
  fn bounding_box(&self) -> Aabb {
    self.bbox
  }
}
