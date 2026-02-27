use rand::Rng;
use raytracer_ramel::prelude::*;

fn main() {
  let path = std::path::Path::new("output/image.png");
  let prefix = path.parent().unwrap();
  std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

  let aspect_ratio = 16.0 / 9.0;
  let width = 400;
  let height = (width as Float / aspect_ratio) as u32;

  let film = film::SimpleFilm::new(width, height);

  let mut filters = FilterList::new();
  filters.add_filter(Arc::new(filter::ExposureFilter::new(-1.2)));
  filters.add_filter(Arc::new(filter::AcesFilmicFilter));
  filters.add_filter(Arc::new(filter::VignetteFilter::new(0.4)));
  // filters.add_filter(Arc::new(filter::GammaFilter::new(2.2)));

  let background_color = ColorRgb::lerp(ColorRgb::BLACK, ColorRgb::SKYBLUE, 0.2);
  let mut world = World::new(move |_ray| background_color);

  let mat_light = Arc::new(material::DiffusionLight::from_color(ColorRgb::lerp(
    ColorRgb::YELLOW,
    ColorRgb::WHITE,
    0.8,
  )));
  let mat_mirror1 = Arc::new(material::Metal::new(ColorRgb::WHITE, 0.1));
  let mat_mirror2 = Arc::new(material::Metal::new(ColorRgb::GREEN, 0.1));

  let mat_error = Arc::new(material::DiffusionLight::new(Arc::new(
    texture::UVCheckerTexture::new(
      10.0,
      10.0,
      Arc::new(texture::SolidColorTexture::new(ColorRgb::MAGENTA)),
      Arc::new(texture::SolidColorTexture::new(ColorRgb::SKYBLUE)),
    ),
  )));

  let input_path = std::path::Path::new("input/girl-car-blue.jpg");
  let tex_center = Arc::new(texture::ImageTexture::new(
    input_path.to_str().unwrap(),
    true,
  ));
  let mat_center = Arc::new(material::Lambertian::new(tex_center));

  let unit_sphere_light = Arc::new(geometry::UnitSphere::new(mat_light.clone()));
  let unit_sphere_m1 = Arc::new(geometry::UnitSphere::new(mat_mirror1.clone()));
  let unit_sphere_m2 = Arc::new(geometry::UnitSphere::new(mat_mirror2.clone()));
  let unit_sphere_center = Arc::new(geometry::UnitSphere::new(mat_center.clone()));
  let unit_cube = Arc::new(geometry::UnitCube::from_one(mat_center.clone()));
  let unit_quad_ground = Arc::new(geometry::UnitQuad::new(mat_error.clone()));

  
  world.add_object(Arc::new(geometry::Instance::new(
    unit_sphere_light.clone(),
    Mat4d::from_translation(Direction::new(3.0, 4.5, -1.0)) * Mat4d::from_scaling(4.0, 4.0, 4.0),
  )));

  world.add_object(geometry::Instance::new_arc(
    unit_sphere_m1.clone(),
    Mat4d::from_translation(Direction::new(-1.2, 0.0, -1.0)) * Mat4d::from_scaling(0.5, 0.5, 0.5),
  ));

  world.add_object(geometry::Instance::new_arc(
    unit_sphere_m2.clone(),
    Mat4d::from_translation(Direction::new(-0.6, 1.2, -1.0)) * Mat4d::from_scaling(0.5, 0.5, 0.5),
  ));

  world.add_object(geometry::Instance::new_arc(
    unit_sphere_center.clone(),
    Mat4d::from_translation(Direction::new(0.0, 0.0, -1.0))
      * Mat4d::from_rotation_y(-PI / 2.0)
      * Mat4d::from_scaling(0.5, 0.5, 0.5),
  ));

  world.add_object(geometry::Instance::new_arc(
    unit_cube.clone(),
    Mat4d::from_translation(Direction::new(1.1, 0.0, 0.0))
      * Mat4d::from_rotation_y(PI / 4.0)
      * Mat4d::from_rotation_x(PI / 4.0)
      * Mat4d::from_scaling(0.7, 0.7, 0.7),
  ));

  world.add_object(geometry::Instance::new_arc(
    unit_quad_ground.clone(),
    Mat4d::from_translation(Direction::new(0.0, -1.0, 0.0))
      * Mat4d::from_rotation_x(-PI / 6.0)
      * Mat4d::from_scaling(10.0, 1.0, 10.0),
  ));

  let mut rng = rand::rng();
  for _ in 0..500 {
    let x = rng.random_range(-3.0..3.0);
    let y = rng.random_range(-0.5..0.5);
    let z = rng.random_range(-20.0..-5.0);
    let r = rng.random_range(0.0..1.0);
    let g = rng.random_range(0.0..1.0);
    let b = rng.random_range(0.0..1.0);
    let radius = rng.random_range(0.0..0.1);
    world.add_object(geometry::Instance::new_arc(
      Arc::new(geometry::UnitSphere::new(Arc::new(
        material::Lambertian::new(Arc::new(texture::SolidColorTexture::new(ColorRgb::new(
          r, g, b,
        )))),
      ))),
      Mat4d::from_translation(Direction::new(x, y, z))
        * Mat4d::from_scaling(radius, radius, radius),
    ));
  }

  world.bvh_finalize();

  let camera = camera::OrthographicCamera::new(
    Point::new(0.0, 0.0, 10.0),
    Point::new(0.0, 0.0, -10.0),
    Direction::new(0.0, 1.0, 0.0),
    2.0,
    aspect_ratio,
  );

  let renderer = renderer::SimpleRenderer::new(200, 200);

  let camera = Arc::new(camera);
  let film = Arc::new(Mutex::new(film));
  let filters = Arc::new(filters);
  let world = Arc::new(world);

  let render_start = std::time::Instant::now();

  renderer.render(RenderConfig {
    camera,
    film: film.clone(),
    filters,
    world,
  });

  let render_elapsed = render_start.elapsed();

  let image = film.lock().unwrap().to_image_srgb();
  image.save(path).expect("Cannot save the image to file.");
  println!("Finished. .Image saved to {}", console::style("output.png").green().bright());
  println!("Elapsed time: {}ms", render_elapsed.as_millis());
}
