use crate::prelude::*;
use std::sync::Once;

pub struct TriangleMesh {
  vertices: Vec<Point>,        // vertex pool. Length V.
  normals: Vec<Direction>,     // normal vector pool. Length 0 or V.
  tex_coords: Vec<UV>,         // tex uv pool. Length 0 or V.
  indices: Vec<u32>,           // continuous 3 indices stands for a triangle. Length F.
  material: Arc<dyn Material>, // global material of this mesh
}

pub struct Triangle {
  mesh: Arc<TriangleMesh>,
  idx: usize, // idx = 3, 6, 9... Visit mesh.indices[idx ~ idx+2] for the 3 indices.
  // caches:
  bbox: Aabb,
  face_unit_normal: Direction,
}

impl Triangle {
  pub fn new(mesh: Arc<TriangleMesh>, idx: usize) -> Self {
    let i0 = mesh.indices[idx];
    let i1 = mesh.indices[idx + 1];
    let i2 = mesh.indices[idx + 2];

    let v0 = mesh.vertices[i0 as usize];
    let v1 = mesh.vertices[i1 as usize];
    let v2 = mesh.vertices[i2 as usize];

    let min_x = v0.x.min(v1.x.min(v2.x));
    let min_y = v0.y.min(v1.y.min(v2.y));
    let min_z = v0.z.min(v1.z.min(v2.z));
    let max_x = v0.x.max(v1.x.max(v2.x));
    let max_y = v0.y.max(v1.y.max(v2.y));
    let max_z = v0.z.max(v1.z.max(v2.z));

    let bbox = Aabb {
      min: Point { x: min_x, y: min_y, z: min_z },
      max: Point { x: max_x, y: max_y, z: max_z },
    };

    let face_normal = (v1 - v0).cross(v2 - v0);
    let face_unit_normal = if face_normal.near_zero() {
      static WARNING: Once = Once::new();
      WARNING.call_once(|| {
        println!("Triangle: Face normal calculated is too small, set to y-hat.");
      });
      Direction::new(0.0, 1.0, 0.0) // 默认向上，或者使用 vup 方向
    } else {
      face_normal.normalize()
    };

    Self { mesh, idx, bbox, face_unit_normal }
  }

  pub fn v0(&self) -> Point {
    self.mesh.vertices[self.mesh.indices[self.idx] as usize]
  }
  pub fn v1(&self) -> Point {
    self.mesh.vertices[self.mesh.indices[self.idx + 1] as usize]
  }
  pub fn v2(&self) -> Point {
    self.mesh.vertices[self.mesh.indices[self.idx + 2] as usize]
  }

  pub fn material(&self) -> Arc<dyn Material> {
    self.mesh.material.clone()
  }

  pub fn unit_normal_at(&self, b1: Float, b2: Float) -> Direction {
    if self.mesh.normals.is_empty() {
      self.face_unit_normal
    } else {
      let n0 = self.mesh.normals[self.mesh.indices[self.idx] as usize];
      let n1 = self.mesh.normals[self.mesh.indices[self.idx + 1] as usize];
      let n2 = self.mesh.normals[self.mesh.indices[self.idx + 2] as usize];
      let b0 = 1.0 - b1 - b2;
      (b0 * n0 + b1 * n1 + b2 * n2).normalize()
    }
  }
  pub fn uv_at(&self, b1: Float, b2: Float) -> UV {
    if self.mesh.tex_coords.is_empty() {
      UV::new(0.5, 0.5)
    } else {
      let uv0 = self.mesh.tex_coords[self.mesh.indices[self.idx] as usize];
      let uv1 = self.mesh.tex_coords[self.mesh.indices[self.idx + 1] as usize];
      let uv2 = self.mesh.tex_coords[self.mesh.indices[self.idx + 2] as usize];
      let b0 = 1.0 - b1 - b2;
      b0 * uv0 + b1 * uv1 + b2 * uv2
    }
  }

  /// If intersects, returns (t, b1, b2).
  /// which implies the intersection point is (1 - b1 - b2)v0 + b1v1 + b2v2,
  /// also known as ray.at(t).
  pub fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<(Float, Float, Float)> {
    let v0 = self.v0();
    let v1 = self.v1();
    let v2 = self.v2();

    let e1 = v1 - v0;
    let e2 = v2 - v0;

    let s = ray.origin - v0;
    let s1 = ray.direction.cross(e2);
    let s2 = s.cross(e1);

    let div = s1.dot(e1);
    if div.abs() < FLOAT_EPSILON {
      return None;
    }
    let inv = 1.0 / div;

    let t = s2.dot(e2) * inv;
    if t < t_min || t > t_max {
      return None;
    }
    let b1 = s1.dot(s) * inv;
    if b1 < 0.0 || b1 > 1.0 {
      return None;
    }
    let b2 = s2.dot(ray.direction) * inv;
    if b2 < 0.0 || b1 + b2 > 1.0 {
      return None;
    }

    Some((t, b1, b2))
  }
}

impl TriangleMesh {
  fn default_material() -> Arc<dyn Material> {
    material::Lambertian::new_arc(texture::SolidColorTexture::new_arc(ColorRgb::YELLOW))
    // Lambertian::new_arc(UVCheckerTexture::new_arc(10.0, 10.0, SolidColorTexture::new_arc(ColorRgb::WHITE), SolidColorTexture::new_arc(ColorRgb::MAGENTA)))
  }
  /// Use default material
  pub fn load_obj_ignore_material(path: &str) -> Vec<Arc<TriangleMesh>> {
    let (models, _materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)
      .expect(&format!("error loading .obj: {}", path));
    // let _materials = _materials.expect(&format!("error loading .obj materials: {}", path));
    let mut res: Vec<Arc<TriangleMesh>> = vec![];
    for m in models {
      let mesh = m.mesh;
      let vertices: Vec<Point> = mesh
        .positions
        .chunks_exact(3)
        .map(|p| Point::new(p[0], p[1], p[2]))
        .collect();
      let normals: Vec<Direction> = mesh
        .normals
        .chunks_exact(3)
        .map(|n| Direction::new(n[0], n[1], n[2]))
        .collect();
      let tex_coords: Vec<UV> = mesh
        .texcoords
        .chunks_exact(2)
        .map(|c| UV::new(c[0], c[1]))
        .collect();
      let material = Self::default_material();
      let indices = mesh.indices.clone();
      res.push(Arc::new(TriangleMesh {
        vertices,
        normals,
        tex_coords,
        material,
        indices,
      }))
    }
    res
  }

  pub fn triangles(self: Arc<Self>) -> Vec<Triangle> {
    let mut res: Vec<Triangle> = vec![];
    for idx in (0..self.indices.len()).step_by(3) {
      res.push(Triangle::new(self.clone(), idx));
    }
    res
  }
}

impl Hittable for Triangle {
  fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
    if let Some((t, b1, b2)) = self.intersect(ray, t_min, t_max) {
      let _v0 = self.v0();
      let _v1 = self.v1();
      let _v2 = self.v2();
      Some(HitRecord::from_ray(
        ray,
        self.unit_normal_at(b1, b2),
        t,
        self.material(),
        self.uv_at(b1, b2),
      ))
    } else {
      None
    }
  }
  fn bounding_box(&self) -> Aabb {
    self.bbox
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::{Arc, Mutex};

  // 辅助函数：快速创建一个渲染场景
  fn render_to_file(world: World, filename: &str, height_scale: Float) {
    let aspect_ratio = 1.0;
    let width = 400;
    let height = 400;

    // 相机放在正前方，看向原点
    let camera = camera::OrthographicCamera::new(
      Point::new(0.0, 0.0, 5.0),     // 相机位置
      Point::new(0.0, 0.0, 0.0),     // 看向原点
      Direction::new(0.0, 1.0, 0.0), // 向上向量
      height_scale,                  // 视口高度
      aspect_ratio,
    );

    let film = film::SimpleFilm::new(width, height);
    let filters = FilterList::new();

    let renderer = renderer::SimpleRenderer::new(50, 5); // 50spp 足够看到结果
    let film_arc = Arc::new(Mutex::new(film));

    renderer.render(RenderConfig {
      camera: Arc::new(camera),
      film: film_arc.clone(),
      filters: Arc::new(filters),
      world: Arc::new(world),
    });

    let output_path = std::path::Path::new("output").join(filename);
    std::fs::create_dir_all("output").ok();
    film_arc
      .lock()
      .unwrap()
      .to_image_srgb()
      .save(output_path)
      .unwrap();
  }

  #[test]
  fn test_single_triangle() {
    // 创建一个简单的红色材质
    let mat_red = material::Lambertian::new_arc(texture::SolidColorTexture::new_arc(
      ColorRgb::new(1.0, 0.2, 0.2),
    ));

    // 创建一个位于原点附近的三角形网格
    let mesh = Arc::new(TriangleMesh {
      vertices: vec![
        Point::new(-0.5, -0.5, 0.0),
        Point::new(0.5, -0.5, 0.0),
        Point::new(0.0, 0.5, 0.0),
      ],
      normals: vec![],
      tex_coords: vec![],
      indices: vec![0, 1, 2],
      material: mat_red,
    });

    let mut world = World::new(|_ray| ColorRgb::new(0.2, 0.2, 0.2)); // 灰色背景
    for tri in mesh.triangles() {
      world.add_object(Arc::new(tri));
    }
    world.bvh_finalize();

    render_to_file(world, "test_triangle.png", 2.0);
  }

  #[test]
  fn test_tetrahedron() {
    let mat_yellow =
      material::Lambertian::new_arc(texture::SolidColorTexture::new_arc(ColorRgb::YELLOW));

    // 正四面体顶点
    let s = 1.0;
    let mesh = Arc::new(TriangleMesh {
      vertices: vec![
        Point::new(s, s, s),
        Point::new(s, -s, -s),
        Point::new(-s, s, -s),
        Point::new(-s, -s, s),
      ],
      normals: vec![],
      tex_coords: vec![],
      indices: vec![
        0, 1, 2, // 面1
        0, 1, 3, // 面2
        0, 2, 3, // 面3
        1, 2, 3, // 面4
      ],
      material: mat_yellow,
    });

    let mut world = World::new(|_ray| ColorRgb::new(0.1, 0.1, 0.1));
    for tri in mesh.triangles() {
      world.add_object(Arc::new(tri));
    }
    world.bvh_finalize();

    render_to_file(world, "test_tetrahedron.png", 4.0);
  }

  #[test]
  fn test_stanford_bunny() {
    let working_dir = env!("CARGO_MANIFEST_DIR");
    let input_path = std::path::Path::new(working_dir).join("assets/stanford-bunny.obj");
    let meshes = TriangleMesh::load_obj_ignore_material(input_path.to_str().unwrap());

    let mut world = World::new(|_ray| ColorRgb::lerp(ColorRgb::WHITE, ColorRgb::BLACK, 0.3));

    // 1. 计算整个 Mesh 的 AABB
    let mut min_p = Point::new(Float::INFINITY, Float::INFINITY, Float::INFINITY);
    let mut max_p = Point::new(
      Float::NEG_INFINITY,
      Float::NEG_INFINITY,
      Float::NEG_INFINITY,
    );

    for mesh_ptr in meshes {
      for &v in &mesh_ptr.vertices {
        min_p.x = min_p.x.min(v.x);
        min_p.y = min_p.y.min(v.y);
        min_p.z = min_p.z.min(v.z);
        max_p.x = max_p.x.max(v.x);
        max_p.y = max_p.y.max(v.y);
        max_p.z = max_p.z.max(v.z);
      }
      for tri in mesh_ptr.triangles() {
        world.add_object(Arc::new(tri));
      }
    }

    let mat_light = Arc::new(material::DiffusionLight::from_color(ColorRgb::lerp(
      ColorRgb::CYAN,
      ColorRgb::WHITE,
      0.8,
    )));
    let unit_sphere_light = geometry::UnitSphere::new_arc(mat_light.clone());
    world.add_object(geometry::Instance::new_arc(
      unit_sphere_light.clone(),
      Mat4d::from_translation(Direction::new(3.0, 4.5, -1.0)) * Mat4d::from_scaling(4.0, 4.0, 4.0),
    ));

    world.bvh_finalize();

    // 2. 计算中心和缩放
    let center = (min_p + max_p) / 2.0;
    let size = max_p - min_p;
    let max_dim = size.x.max(size.y.max(size.z));

    println!("Bunny Box: Min {:?}, Max {:?}", min_p, max_p);
    println!("Bunny Center: {:?}, Max Dimension: {}", center, max_dim);

    // 3. 自动配置相机
    // 视口高度设为比最大尺寸稍大一点点（1.2倍），确保装得下
    let height_scale = max_dim * 1.2;
    let camera = camera::OrthographicCamera::new(
      center + Direction::new(0.0, 0.0, max_dim * 2.0), // 相机退后放置
      center,                                           // 盯着中心看
      Direction::new(0.0, 1.0, 0.0),
      height_scale,
      1.0,
    );

    // 4. 渲染
    let film = film::SimpleFilm::new(512, 512);
    let film_arc = Arc::new(Mutex::new(film));
    renderer::SimpleRenderer::new(200, 10).render(RenderConfig {
      camera: Arc::new(camera),
      film: film_arc.clone(),
      filters: Arc::new(FilterList::new()),
      world: Arc::new(world),
    });

    let output_path = std::path::Path::new(working_dir).join("output/stanford-bunny.png");
    film_arc
      .lock()
      .unwrap()
      .to_image_srgb()
      .save(output_path)
      .unwrap();
  }

  #[test]
  fn debug_bunny_intersection_isolation() {
    let working_dir = env!("CARGO_MANIFEST_DIR");
    let input_path = std::path::Path::new(working_dir).join("assets/stanford-bunny.obj");

    // 1. 原始加载
    let meshes = TriangleMesh::load_obj_ignore_material(input_path.to_str().unwrap());
    assert!(!meshes.is_empty(), "未能加载任何 Mesh");

    // 2. 这里的中心坐标是你之前打印出来的
    let bunny_center = Point::new(-0.0168405, 0.110154, -0.00153699);

    // 模拟从相机发射的射线 (Z=1.0 射向 Z=-1.0)
    let test_ray = Ray::new(
      Point::new(bunny_center.x, bunny_center.y, 1.0),
      Direction::new(0.0, 0.0, -1.0),
    );

    println!("--- 独立求交测试开始 ---");
    let mut hit_count = 0;
    let mut min_t = Float::INFINITY;

    for mesh in &meshes {
      // 暴力遍历所有三角形，绕过 World 和 BVH
      for idx in (0..mesh.indices.len()).step_by(3) {
        let tri = Triangle::new(mesh.clone(), idx);

        // 直接调用 intersect
        if let Some((t, b1, b2)) = tri.intersect(&test_ray, 0.0001, 1000.0) {
          hit_count += 1;
          min_t = min_t.min(t);
        }
      }
    }

    println!("射线起点: {:?}", test_ray.origin);
    println!("射线方向: {:?}", test_ray.direction);
    println!("总击中三角形数: {}", hit_count);

    if hit_count > 0 {
      println!("最近击中距离 t: {}", min_t);
      println!("相交点坐标: {:?}", test_ray.at(min_t));
    } else {
      // 如果没击中，随机抽查一个三角形的顶点，看看它们到底在哪
      println!("错误：未击中任何三角形！");
      if let Some(m) = meshes.first() {
        if m.indices.len() >= 3 {
          let v0 = m.vertices[m.indices[0] as usize];
          let v1 = m.vertices[m.indices[1] as usize];
          let v2 = m.vertices[m.indices[2] as usize];
          println!(
            "抽查第一个三角形顶点:\n  v0: {:?}\n  v1: {:?}\n  v2: {:?}",
            v0, v1, v2
          );
        }
      }
    }
    println!("--- 测试结束 ---");

    assert!(
      hit_count > 0,
      "即使是暴力搜索也未击中兔子，说明模型数据或求交算法有误"
    );
  }

  #[test]
  fn test_perspective_tetrahedron() {
    let mut world = World::new(|_ray| ColorRgb::new(0.2, 0.2, 0.2));

    // 材质与几何
    let mat = material::Lambertian::new_arc(texture::SolidColorTexture::new_arc(ColorRgb::new(
      0.7, 0.2, 0.2,
    )));
    let s = 1.0;
    let mesh = Arc::new(TriangleMesh {
      vertices: vec![
        Point::new(s, s, s),
        Point::new(s, -s, -s),
        Point::new(-s, s, -s),
        Point::new(-s, -s, s),
      ],
      normals: vec![],
      tex_coords: vec![],
      indices: vec![
        0, 1, 2, // 面1
        0, 1, 3, // 面2
        0, 2, 3, // 面3
        1, 2, 3, // 面4
      ],
      material: mat,
    });
    for tri in mesh.triangles() {
      world.add_object(Arc::new(tri));
    }
    world.bvh_finalize();

    // 相机：放在斜上方 (3, 3, 3) 看向原点 (0, 0, 0)
    let camera = PerspectiveCamera::new(
      Point::new(3.0, 3.0, 3.0),
      Point::new(0.0, 0.0, 0.0),
      Direction::new(0.0, 1.0, 0.0),
      45.0, // vfov
      1.0,  // aspect ratio
    );

    let film = Arc::new(Mutex::new(film::SimpleFilm::new(400, 400)));
    renderer::SimpleRenderer::new(50, 5).render(RenderConfig {
      camera: Arc::new(camera),
      film: film.clone(),
      filters: Arc::new(FilterList::new()),
      world: Arc::new(world),
    });

    let working_dir = env!("CARGO_MANIFEST_DIR");
    let output_path = std::path::Path::new(working_dir).join("output/persp-tetrahedron.png");
    film
      .lock()
      .unwrap()
      .to_image_srgb()
      .save(output_path)
      .unwrap();
  }
  #[test]
  fn test_perspective_bunny() {
    let working_dir = env!("CARGO_MANIFEST_DIR");
    let input_path = std::path::Path::new(working_dir).join("assets/stanford-bunny.obj");
    let meshes = TriangleMesh::load_obj_ignore_material(input_path.to_str().unwrap());

    let mut world = World::new(|_ray| ColorRgb::new(0.1, 0.1, 0.1));

    let mut min_p = Point::new(Float::INFINITY, Float::INFINITY, Float::INFINITY);
    let mut max_p = Point::new(
      Float::NEG_INFINITY,
      Float::NEG_INFINITY,
      Float::NEG_INFINITY,
    );

    for mesh_ptr in meshes {
      for &v in &mesh_ptr.vertices {
        min_p.x = min_p.x.min(v.x);
        min_p.y = min_p.y.min(v.y);
        min_p.z = min_p.z.min(v.z);
        max_p.x = max_p.x.max(v.x);
        max_p.y = max_p.y.max(v.y);
        max_p.z = max_p.z.max(v.z);
      }
      for tri in mesh_ptr.triangles() {
        world.add_object(Arc::new(tri));
      }
    }

    // 光源：侧后方强光
    let mat_light = Arc::new(material::DiffusionLight::from_color(ColorRgb::new(
      10.0, 10.0, 10.0,
    )));
    let sphere_light = geometry::UnitSphere::new_arc(mat_light);
    world.add_object(geometry::Instance::new_arc(
      sphere_light,
      Mat4d::from_translation(Direction::new(3.0, 4.5, 2.0)) * Mat4d::from_scaling(2.0, 2.0, 2.0),
    ));

    world.bvh_finalize();

    let center = (min_p + max_p) / 2.0;
    let max_dim = (max_p - min_p).x.max((max_p - min_p).y);

    // 透视相机：从 45 度侧面观察
    let camera = PerspectiveCamera::new(
      center + Direction::new(max_dim * 2.0, max_dim * 1.5, max_dim * 3.0),
      center,
      Direction::new(0.0, 1.0, 0.0),
      30.0,
      1.0,
    );

    let film = Arc::new(Mutex::new(film::SimpleFilm::new(512, 512)));
    let mut filter_list = FilterList::new();
    filter_list.add_filter(Arc::new(filter::ReinhardFilter)); // since the light source exceeds WHITE... 
    // 注意：spp = 100, depth = 8
    renderer::SimpleRenderer::new(100, 8).render(RenderConfig {
      camera: Arc::new(camera),
      film: film.clone(),
      filters: Arc::new(filter_list),
      world: Arc::new(world),
    });

    let output_path = std::path::Path::new(working_dir).join("output/persp-bunny.png");
    film
      .lock()
      .unwrap()
      .to_image_srgb()
      .save(output_path)
      .unwrap();
  }
}
