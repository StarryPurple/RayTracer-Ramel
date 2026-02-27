use crate::prelude::*;
use crate::renderer::RenderConfig;
use rand::Rng;
use rayon::prelude::*;

pub struct SimpleRenderer {
  samples_per_pixel: u32,
  max_depth: u32,
}

impl SimpleRenderer {
  #[inline]
  pub fn new(samples_per_pixel: u32, max_depth: u32) -> SimpleRenderer {
    Self { samples_per_pixel, max_depth }
  }
  // depth decreases while function is recursively called.
  // When it reaches 0, terminate and return ColorRgb::BLACK.
  fn ray_color(ray: &Ray, world: &World, depth: u32) -> ColorRgb {
    if depth == 0 {
      return ColorRgb::BLACK;
    }

    if let Some(record) = world.hit(ray, FLOAT_EPSILON, Float::MAX) {
      let emitted = record.material.emitted(record.mat_uv, record.point);
      return if let Some((attenuation, scattered)) = record.material.clone().scatter(ray, &record) {
        emitted + attenuation * Self::ray_color(&scattered, world, depth - 1)
      } else {
        emitted
      };
    }

    // background color
    world.background_shader()(ray)
  }
}

impl Renderer for SimpleRenderer {
  fn render(&self, config: RenderConfig) {
    let RenderConfig { film, camera, filters, world } = config;

    let (width, height);
    {
      let film = film.lock().unwrap();
      width = film.width();
      height = film.height();
    }

    let bar = indicatif::ProgressBar::new(height as u64);

    let rows = (0..height)
      .into_par_iter()
      .map(|y| {
        let mut rng = rand::rng();
        let mut row = Vec::with_capacity(width as usize);

        for x in 0..width {
          let mut pixel_color = ColorRgb::BLACK;

          for _ in 0..self.samples_per_pixel {
            let u = (x as Float + rng.random::<Float>()) / (width - 1) as Float;
            let v = 1.0 - (y as Float + rng.random::<Float>()) / (height - 1) as Float;

            let ray = camera.get_ray(u, v);
            pixel_color += Self::ray_color(&ray, &world, self.max_depth);
          }

          pixel_color /= self.samples_per_pixel as Float;
          row.push(pixel_color);
        }
        bar.inc(1);
        row
      })
      .collect::<Vec<_>>();

    bar.finish_and_clear();

    let mut film = film.lock().unwrap();
    for (y, row) in rows.iter().enumerate() {
      for (x, pixel_color) in row.iter().enumerate() {
        film.set_pixel(x as u32, y as u32, *pixel_color);
      }
    }

    filters.process(&mut *film);
  }
}
