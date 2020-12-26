use fs::File;
use image::{DynamicImage, GenericImage, ImageError, ImageOutputFormat, Rgb, imageops::overlay, load_from_memory};
use imageproc::drawing::draw_text_mut;
use path::Path;
use std::panic;
use std::{fs, path};
// extern "C" {
//   #[wasm_bindgen(js_namespace=console, js_name=log)]
//   console_log("")
// }

pub struct Transformation {
    pub width: i8,
    pub height: i8,
}

#[derive(Default)]
pub struct WasinaryImage<'a> {
    url: &'a str,
    image: Option<Box<[u8]>>,
    output_image: Option<Box<[u8]>>,
}

fn download_image(url: &str) -> Option<Box<[u8]>> {
    let img_bytes = reqwest::blocking::get(url).ok()?.bytes().ok();
    let img: Result<DynamicImage, ImageError> = match img_bytes {
        Some(x) => image::load_from_memory(&x),
        None => Err(image::ImageError::Decoding(
            image::error::DecodingError::new(image::error::ImageFormatHint::Unknown, "OOps!!"),
        )),
    };
    let mut out_writer = Vec::new();
    img.unwrap()
        .write_to(&mut out_writer, image::ImageOutputFormat::Png)
        .unwrap();
    Some(out_writer.into_boxed_slice())
    // let img = image::load_from_memory(&img_bytes);
}

impl<'a> WasinaryImage<'a> {
    pub fn new(url: &'a str) -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        WasinaryImage {
            url,
            image: Some(Box::new([0])),
            output_image: Some(Box::new([0])),
        }
    }

    fn write_to_output(&mut self, src_img: Option<DynamicImage>) -> Self {
        let mut out_writer = Vec::new();
        src_img
            .unwrap()
            .write_to(&mut out_writer, image::ImageFormat::Png)
            .unwrap();
        self.output_image = Some(out_writer.into_boxed_slice());
        WasinaryImage {
            image: self.image.clone(),
            output_image: self.output_image.clone(),
            ..Default::default()
        }
    }

    pub fn download(&mut self) -> Self {
        let img = download_image(&self.url);
        let imgs = img.clone();
        self.image = imgs;
        Self {
            image: img,
            output_image: self.image.clone(),
            ..Default::default()
        }
    }

    pub fn monochrome(&mut self) -> Self {
        let img = &self.image;
        let mc = image::load_from_memory(&img.clone().unwrap().into_vec())
            .unwrap()
            .grayscale();
        self.write_to_output(Some(mc))
    }

    pub fn sepia(&mut self) -> Self {
        let img = image::load_from_memory(&self.output_image.clone().unwrap().into_vec())
            .unwrap()
            .to_rgba8();
        let (width, height) = img.dimensions();
        let mut output_img = img.clone();
        for x in 0..width {
            for y in 0..height {
                let pixel = img.get_pixel(x, y);
                let mut pixel_cp = *pixel;
                let r = (0.393 * pixel[0] as f64)
                    + (0.769 * pixel[1] as f64)
                    + (0.189 * pixel[0] as f64);
                let g = (0.349 * pixel[0] as f64)
                    + (0.686 * pixel[1] as f64)
                    + (0.168 * pixel[0] as f64);
                let b = (0.272 * pixel[0] as f64)
                    + (0.53 * pixel[1] as f64)
                    + (0.131 * pixel[0] as f64);

                if r > 255.0 {
                    pixel_cp[0] = 255;
                } else {
                    pixel_cp[0] = r as u8;
                }

                if g > 255.0 {
                    pixel_cp[1] = 255
                } else {
                    pixel_cp[1] = g as u8;
                }

                if b > 255.0 {
                    pixel_cp[2] = 255
                } else {
                    pixel_cp[2] = b as u8;
                }

                pixel_cp[3] = pixel[3];
                output_img.put_pixel(x, y, pixel_cp);
            }
        }

        let mut out_writer: Vec<u8> = Vec::new();
        let md = image::DynamicImage::ImageRgba8(output_img);
        md.write_to(&mut out_writer, image::ImageOutputFormat::Png)
            .unwrap();
        self.output_image = Some(out_writer.into_boxed_slice());
        WasinaryImage {
            output_image: self.output_image.clone(),
            image: self.image.clone(),
            ..Default::default()
        }
    }

    pub fn blur(&mut self, sigma: f32) -> Self {
        let img = image::load_from_memory(&self.output_image.clone().unwrap().into_vec())
            .unwrap()
            .blur(sigma);
        self.write_to_output(Some(img))
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Self {
        let img = image::load_from_memory(&self.output_image.clone().unwrap().into_vec())
            .unwrap()
            .resize(width, height, image::imageops::FilterType::Gaussian);
        self.write_to_output(Some(img))
    }

    pub fn crop(&mut self, width: u32, height: u32) -> Self {
        let img = image::load_from_memory(&self.output_image.clone().unwrap().into_vec())
            .unwrap()
            .crop_imm(0, 0, width, height);
        self.write_to_output(Some(img))
    }

    pub fn rotate(&mut self, degree: u32) -> Self {
        let img = match degree {
            90 => Some(
                image::load_from_memory(&self.output_image.clone().unwrap().into_vec())
                    .unwrap()
                    .rotate90(),
            ),
            180 => Some(
                image::load_from_memory(&self.output_image.clone().unwrap().into_vec())
                    .unwrap()
                    .rotate180(),
            ),
            270 => Some(
                image::load_from_memory(&self.output_image.clone().unwrap().into_vec())
                    .unwrap()
                    .rotate270(),
            ),
            360 => Some(
                image::load_from_memory(&self.output_image.clone().unwrap().into_vec()).unwrap(),
            ),
            _ => {
                unsafe {
                    web_sys::console::log_1(&"Cannot rotate to the degree".into());
                }
                None
            }
        };
        self.write_to_output(img)
    }

    pub fn overlay(&mut self, url: &'a str, x:u32, y:u32) -> Self {
      let downloaded = download_image(url);
      let mut overlay_img = image::load_from_memory(&downloaded.unwrap().into_vec()).unwrap();
      let out_img = image::load_from_memory(&self.output_image.clone().unwrap().into_vec()).unwrap();
      let fout = &mut File::create(&Path::new(&format!("{}.png", "overlay1"))).unwrap();
      overlay_img.write_to(fout, image::ImageOutputFormat::Png).unwrap();
      image::imageops::overlay(&mut overlay_img, &out_img, x, y);
      self.write_to_output(Some(overlay_img))
    }
    // for now, the watermark simply puts the image in the bottom right corner of the background image
    // and the image supplied has to have the "watermark effect" on it already
    // TODO: perhaps detect and apply "watermark effect"
    pub fn watermark(&mut self, url: &'a str) -> Self {
      let downloaded = download_image(url);
      let watermark_img = image::load_from_memory(&downloaded.unwrap().into_vec()).unwrap().resize(200, 200, image::imageops::FilterType::Gaussian).brighten(100);
      let mut out_img = image::load_from_memory(&self.output_image.clone().unwrap().into_vec()).unwrap();
      let (width, height) = out_img.clone().to_rgba8().dimensions();
      let (w, h) = watermark_img.clone().to_rgb8().dimensions();
      println!("Width is: {:?} and height is: {:?}", width, height);
      image::imageops::overlay(&mut out_img, &watermark_img,  width - (w+10), height - (h+10));
      self.write_to_output(Some(out_img))
    }

    pub fn write_text(&mut self, text: &'a str) -> Self {
      let mut out_img = image::RgbImage::new(200, 200);
      let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
      let font = rusttype::Font::try_from_vec(font).unwrap();
      let height = 12.4;
      let scale = rusttype::Scale{
        x: height*2.0,
        y: height,
      };
      draw_text_mut(&mut out_img, Rgb([0u8, 0u8, 255u8]), 0, 0, scale, &font, text);
      let out_img = DynamicImage::ImageRgb8(out_img);
      let fout = &mut File::create(&Path::new(&format!("{}.png", "dummy"))).unwrap();
      out_img.write_to(fout, ImageOutputFormat::Png).unwrap();
      let mut base_img = load_from_memory(&self.output_image.clone().unwrap().into_vec()).unwrap();
      let (w, h) = base_img.clone().into_rgba8().dimensions();
      overlay(&mut base_img, &out_img, w/2, h-100);
      self.write_to_output(Some(base_img))
    }


    pub fn done(&self) -> DynamicImage {
        let img = image::load_from_memory(&self.output_image.clone().unwrap().into_vec()).unwrap();
        return img;
    }
}
