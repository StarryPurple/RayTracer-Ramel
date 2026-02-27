use crate::prelude::*;

pub struct Instance {
  object: Arc<dyn Hittable>,
  trans_mat: Mat4d,
  inv_trans: Mat4d,
  bbox: Aabb,
}

impl Instance {
  pub fn new(object: Arc<dyn Hittable>, trans_mat: Mat4d) -> Self {
    // calculate bbox here.
    let obj_bbox = object.bounding_box();
    let mut res_bbox = Aabb::default();
    for i in 0..2 {
      for j in 0..2 {
        for k in 0..2 {
          let x = if i == 1 { obj_bbox.max.x } else { obj_bbox.min.x };
          let y = if j == 1 { obj_bbox.max.y } else { obj_bbox.min.y };
          let z = if k == 1 { obj_bbox.max.z } else { obj_bbox.min.z };

          let new_p = trans_mat.transform_point(Point { x, y, z });

          for c in 0..3 {
            res_bbox.min[c] = res_bbox.min[c].min(new_p[c]);
            res_bbox.max[c] = res_bbox.max[c].max(new_p[c]);
          }
        }
      }
    }
    Self {
      bbox: res_bbox,
      object,
      trans_mat,
      inv_trans: trans_mat.inverse().unwrap(),
    }
  }
  pub fn new_arc(object: Arc<dyn Hittable>, trans_mat: Mat4d) -> Arc<Self> {
    Arc::new(Self::new(object, trans_mat))
  }
}

impl Hittable for Instance {
  fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
    let origin = self.inv_trans.transform_point(ray.origin);
    let direction = self.inv_trans.transform_vector(ray.direction);
    let local_ray = Ray::new(origin, direction);
    let mut rec = self.object.hit(&local_ray, t_min, t_max)?;
    rec.point = self.trans_mat.transform_point(rec.point);
    rec.unit_normal = self
      .inv_trans
      .transpose_transform_vector(rec.unit_normal)
      .normalize(); // comment this?
    Some(rec)
  }
  fn bounding_box(&self) -> Aabb {
    self.bbox
  }
}
