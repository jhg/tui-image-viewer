use std::io::Write;
use image;
use image::Pixel;
use image::GenericImageView;
use image::DynamicImage;
use image::imageops::FilterType;
use crossterm::style::{style, Color};
use structopt::StructOpt;
use std::path::PathBuf;

/// View images from your terminal.
#[derive(StructOpt)]
#[structopt(name = "tui-image-viewer")]
struct Opt {
    /// Use RGB colors (not all terminals support it).
    #[structopt(long)]
    rgb: bool,

    /// Limit width instead of use terminal width.
    #[structopt(long, short)]
    width: Option<usize>,

    /// Use gaussian algorithm to resize instead of nearest.
    #[structopt(long, short)]
    gaussian: bool,

    /// File to show.
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

type ImageMatrixRGB = Vec<Vec<(u8, u8, u8)>>;

fn main() {
    let opt = Opt::from_args();
    let img = image::open(opt.file).expect("It is NOT possible to open the file");
    let img: ImageMatrixRGB = convert(resize(img, opt.width, opt.gaussian));
    show(img, opt.rgb);
}

/// Show image in console.
fn show(img: ImageMatrixRGB, use_rgb: bool) {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for row in img {
        for pixel in row {
            let (r, g, b) = pixel;
            let color = if use_rgb {
                Color::Rgb { r, g, b}
            } else {
                gray_color(r, g, b)
            };
            let block = style("\u{2588}").with(color);
            write!(handle, "{}", block).expect("Is not possible write to stdout because an error");
        }
        write!(handle, "\n").expect("Is not possible write to stdout because an error");
    }
}

/// Convert the image to vector of vectors with RGB tuples.
/// 
/// This allow to show as RBG or in grayscale.
fn convert(img: DynamicImage) -> ImageMatrixRGB {
    img.into_rgb()
    .rows()
    .map(|row| {
        row.map(|pixel| {
            let channels = pixel.channels();
            (channels[0], channels[1], channels[2])
        }).collect()
    }).collect()
}

/// Resize image to avoid overflow terminal.
fn resize(img: DynamicImage, width: Option<usize>, use_gaussian: bool) -> DynamicImage {
    let width = if let Some(width) = width {
        width
    } else {
        let (width, _) = term_size::dimensions().unwrap_or((80, 80));
        width
    };
    let (_, height) = img.dimensions(); // resize method keep the ratio
    let filter = if use_gaussian { FilterType::Gaussian } else { FilterType::Nearest };
    img.resize(width as u32, height as u32, filter)
}

/// Convert RGB average to black, dark grey, grey or white.
fn gray_color(r: u8, g: u8, b: u8) -> Color {
    let luma: u8 = ((r as u16 + g as u16 + b as u16)/3) as u8;
    if luma < 64 {
        Color::Black
    } else if luma < 128 {
        Color::DarkGrey
    } else if luma < 192 {
        Color::Grey
    } else {
        Color::White
    }
}
