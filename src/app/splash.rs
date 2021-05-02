use anyhow::{anyhow, Result};
use image::{DynamicImage, RgbImage};
use rust_embed::RustEmbed;
use std::cmp::Ordering;
use std::collections::HashMap;

/// RGB values.
pub type ColorTuple = (u8, u8, u8);
/// X and Y coordinates.
pub type Point = (f64, f64);

/// A directory of binary assets.
#[derive(Debug, RustEmbed)]
#[folder = "assets/"]
struct Assets;

/// Splash screen data.
#[derive(Debug)]
pub struct SplashScreen {
	/// Image that will be used for constructing the color data.
	pub image: DynamicImage,
	/// Color data that consists of RGB values and coordinates.
	data: HashMap<ColorTuple, Vec<Point>>,
	/// Rendering step of the splash screen.
	pub step: i32,
	/// Number of the rendering steps.
	steps: i32,
}

impl SplashScreen {
	/// Constructs a new instance of `SplashScreen`.
	pub fn new(image_path: &str, steps: i32) -> Result<Self> {
		match Assets::get(image_path) {
			Some(asset) => Ok(Self {
				image: image::load_from_memory(asset.as_ref())?,
				data: HashMap::new(),
				step: steps,
				steps,
			}),
			None => Err(anyhow!(
				"cannot find the splash screen asset: {}",
				image_path
			)),
		}
	}

	/// Returns the color data based on the rendering step.
	///
	/// At the 12th render step, the image is at the darkest and has the max blurriness.
	/// Between the render steps 6-12, the image is getting brighter and less blurry.
	/// From rendering step 0 to 6, the image is returned without any additional effects.
	/// Image is returned as grayscale if `colored` argument is `false`.
	pub fn get(&mut self, colored: bool) -> HashMap<ColorTuple, Vec<Point>> {
		self.step -= 1;
		match self.step.cmp(&(self.steps / 2)) {
			Ordering::Greater => {
				if !colored {
					self.image = self.image.grayscale()
				}
				let value = self.step - (self.steps / 2);
				self.group_image_colors(
					self.image
						.brighten(value * -20)
						.blur((value * 2) as f32)
						.to_rgb8(),
				)
			}
			Ordering::Equal => self.group_image_colors(if colored {
				self.image.to_rgb8()
			} else {
				self.image.grayscale().to_rgb8()
			}),
			Ordering::Less => self.data.clone(),
		}
	}

	/// Groups the colors based on their RGB values and coordinates.
	fn group_image_colors(
		&mut self,
		image: RgbImage,
	) -> HashMap<ColorTuple, Vec<Point>> {
		let mut data = HashMap::<ColorTuple, Vec<Point>>::new();
		for (x, y, color) in image.enumerate_pixels() {
			let x = f64::from(x);
			let y = f64::from(
				image.height().checked_sub(y + 1).unwrap_or_default(),
			);
			let color = (color[0], color[1], color[2]);
			if let Some(points) = data.get(&color) {
				let mut points = points.clone();
				points.push((x, y));
				data.insert(color, points);
			} else {
				data.insert(color, vec![(x, y)]);
			}
		}
		self.data = data.clone();
		data
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use image::{Rgb, RgbImage};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_app_splash() {
		let image = RgbImage::from_fn(2, 2, |x, y| {
			if (x + y) % 2 == 0 {
				Rgb([0, 0, 0])
			} else {
				Rgb([128, 64, 32])
			}
		});
		let mut splash_screen = SplashScreen {
			image: DynamicImage::ImageRgb8(image),
			data: HashMap::new(),
			step: 4,
			steps: 4,
		};
		assert_eq!(
			[
				((28, 28, 28), vec![(1.0, 1.0), (0.0, 0.0)]),
				((27, 27, 27), vec![(0.0, 1.0), (1.0, 0.0,)]),
			]
			.iter()
			.cloned()
			.collect::<HashMap<ColorTuple, Vec<Point>>>(),
			splash_screen.get(false)
		);
		assert_eq!(
			[
				((75, 75, 75), vec![(1.0, 1.0), (0.0, 0.0)]),
				((0, 0, 0), vec![(0.0, 1.0), (1.0, 0.0,)]),
			]
			.iter()
			.cloned()
			.collect::<HashMap<ColorTuple, Vec<Point>>>(),
			splash_screen.get(false)
		);
	}
}
