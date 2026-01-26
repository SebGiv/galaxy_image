pub mod png_loader;
pub mod bmp_loader;
pub mod jpeg_loader;

pub use png_loader::{load_png, save_png};
pub use bmp_loader::{load_bmp, save_bmp};
pub use jpeg_loader::{load_jpeg, save_jpeg};
