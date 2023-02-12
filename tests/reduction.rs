use indexmap::IndexSet;
use oxipng::{internal_tests::*, Interlacing, RowFilter};
use oxipng::{InFile, OutFile};
use std::fs::remove_file;
use std::path::Path;
use std::path::PathBuf;

fn get_opts(input: &Path) -> (OutFile, oxipng::Options) {
    let mut options = oxipng::Options {
        force: true,
        ..Default::default()
    };
    let mut filter = IndexSet::new();
    filter.insert(RowFilter::None);
    options.filter = filter;

    (
        OutFile::Path(Some(input.with_extension("out.png"))),
        options,
    )
}

fn test_it_converts(
    input: &str,
    optimize_alpha: bool,
    color_type_in: ColorType,
    bit_depth_in: BitDepth,
    color_type_out: ColorType,
    bit_depth_out: BitDepth,
) {
    let input = PathBuf::from(input);
    let (output, mut opts) = get_opts(&input);
    opts.optimize_alpha = optimize_alpha;
    let png = PngData::new(&input, opts.fix_errors).unwrap();

    assert_eq!(png.raw.ihdr.color_type, color_type_in);
    assert_eq!(png.raw.ihdr.bit_depth, bit_depth_in, "test file is broken");
    assert_eq!(png.raw.ihdr.interlaced, Interlacing::None);

    match oxipng::optimize(&InFile::Path(input), &output, &opts) {
        Ok(_) => (),
        Err(x) => panic!("{}", x),
    };
    let output = output.path().unwrap();
    assert!(output.exists());

    let png = match PngData::new(output, opts.fix_errors) {
        Ok(x) => x,
        Err(x) => {
            remove_file(output).ok();
            panic!("{}", x)
        }
    };

    assert_eq!(png.raw.ihdr.color_type, color_type_out);
    assert_eq!(png.raw.ihdr.bit_depth, bit_depth_out);

    remove_file(output).ok();
}

#[test]
fn rgba_16_should_be_rgba_16() {
    test_it_converts(
        "tests/files/rgba_16_should_be_rgba_16.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::RGBA,
        BitDepth::Sixteen,
    );
}

#[test]
fn rgba_16_should_be_rgba_8() {
    test_it_converts(
        "tests/files/rgba_16_should_be_rgba_8.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::RGBA,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_8_should_be_rgba_8() {
    test_it_converts(
        "tests/files/rgba_8_should_be_rgba_8.png",
        false,
        ColorType::RGBA,
        BitDepth::Eight,
        ColorType::RGBA,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_16_should_be_rgb_16() {
    test_it_converts(
        "tests/files/rgba_16_should_be_rgb_16.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::RGB,
        BitDepth::Sixteen,
    );
}

#[test]
fn rgba_16_should_be_rgb_8() {
    test_it_converts(
        "tests/files/rgba_16_should_be_rgb_8.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::RGB,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_8_should_be_rgb_8() {
    test_it_converts(
        "tests/files/rgba_8_should_be_rgb_8.png",
        false,
        ColorType::RGBA,
        BitDepth::Eight,
        ColorType::RGB,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_16_should_be_rgb_trns_16() {
    test_it_converts(
        "tests/files/rgba_16_should_be_rgb_trns_16.png",
        true,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::RGB,
        BitDepth::Sixteen,
    );
}

#[test]
fn rgba_8_should_be_rgb_trns_8() {
    test_it_converts(
        "tests/files/rgba_8_should_be_rgb_trns_8.png",
        true,
        ColorType::RGBA,
        BitDepth::Eight,
        ColorType::RGB,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_16_should_be_palette_8() {
    test_it_converts(
        "tests/files/rgba_16_should_be_palette_8.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::Indexed,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_8_should_be_palette_8() {
    test_it_converts(
        "tests/files/rgba_8_should_be_palette_8.png",
        false,
        ColorType::RGBA,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_16_should_be_palette_4() {
    test_it_converts(
        "tests/files/rgba_16_should_be_palette_4.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::Indexed,
        BitDepth::Four,
    );
}

#[test]
fn rgba_8_should_be_palette_4() {
    test_it_converts(
        "tests/files/rgba_8_should_be_palette_4.png",
        false,
        ColorType::RGBA,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::Four,
    );
}

#[test]
fn rgba_16_should_be_palette_2() {
    test_it_converts(
        "tests/files/rgba_16_should_be_palette_2.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::Indexed,
        BitDepth::Two,
    );
}

#[test]
fn rgba_8_should_be_palette_2() {
    test_it_converts(
        "tests/files/rgba_8_should_be_palette_2.png",
        false,
        ColorType::RGBA,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::Two,
    );
}

#[test]
fn rgba_16_should_be_palette_1() {
    test_it_converts(
        "tests/files/rgba_16_should_be_palette_1.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::Indexed,
        BitDepth::One,
    );
}

#[test]
fn rgba_8_should_be_palette_1() {
    test_it_converts(
        "tests/files/rgba_8_should_be_palette_1.png",
        false,
        ColorType::RGBA,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::One,
    );
}

#[test]
fn rgba_16_should_be_grayscale_alpha_16() {
    test_it_converts(
        "tests/files/rgba_16_should_be_grayscale_alpha_16.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::GrayscaleAlpha,
        BitDepth::Sixteen,
    );
}

#[test]
fn rgba_16_should_be_grayscale_alpha_8() {
    test_it_converts(
        "tests/files/rgba_16_should_be_grayscale_alpha_8.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::GrayscaleAlpha,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_8_should_be_grayscale_alpha_8() {
    test_it_converts(
        "tests/files/rgba_8_should_be_grayscale_alpha_8.png",
        false,
        ColorType::RGBA,
        BitDepth::Eight,
        ColorType::GrayscaleAlpha,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_16_should_be_grayscale_16() {
    test_it_converts(
        "tests/files/rgba_16_should_be_grayscale_16.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::Grayscale,
        BitDepth::Sixteen,
    );
}

#[test]
fn rgba_16_should_be_grayscale_8() {
    test_it_converts(
        "tests/files/rgba_16_should_be_grayscale_8.png",
        false,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::Grayscale,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_8_should_be_grayscale_8() {
    test_it_converts(
        "tests/files/rgba_8_should_be_grayscale_8.png",
        false,
        ColorType::RGBA,
        BitDepth::Eight,
        ColorType::Grayscale,
        BitDepth::Eight,
    );
}

#[test]
fn rgb_16_should_be_rgb_16() {
    test_it_converts(
        "tests/files/rgb_16_should_be_rgb_16.png",
        false,
        ColorType::RGB,
        BitDepth::Sixteen,
        ColorType::RGB,
        BitDepth::Sixteen,
    );
}

#[test]
fn rgb_16_should_be_rgb_8() {
    test_it_converts(
        "tests/files/rgb_16_should_be_rgb_8.png",
        false,
        ColorType::RGB,
        BitDepth::Sixteen,
        ColorType::RGB,
        BitDepth::Eight,
    );
}

#[test]
fn rgb_8_should_be_rgb_8() {
    test_it_converts(
        "tests/files/rgb_8_should_be_rgb_8.png",
        false,
        ColorType::RGB,
        BitDepth::Eight,
        ColorType::RGB,
        BitDepth::Eight,
    );
}

#[test]
fn rgb_16_should_be_palette_8() {
    test_it_converts(
        "tests/files/rgb_16_should_be_palette_8.png",
        false,
        ColorType::RGB,
        BitDepth::Sixteen,
        ColorType::Indexed,
        BitDepth::Eight,
    );
}

#[test]
fn rgb_8_should_be_palette_8() {
    test_it_converts(
        "tests/files/rgb_8_should_be_palette_8.png",
        false,
        ColorType::RGB,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::Eight,
    );
}

#[test]
fn rgb_16_should_be_palette_4() {
    test_it_converts(
        "tests/files/rgb_16_should_be_palette_4.png",
        false,
        ColorType::RGB,
        BitDepth::Sixteen,
        ColorType::Indexed,
        BitDepth::Four,
    );
}

#[test]
fn rgb_8_should_be_palette_4() {
    test_it_converts(
        "tests/files/rgb_8_should_be_palette_4.png",
        false,
        ColorType::RGB,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::Four,
    );
}

#[test]
fn rgb_16_should_be_palette_2() {
    test_it_converts(
        "tests/files/rgb_16_should_be_palette_2.png",
        false,
        ColorType::RGB,
        BitDepth::Sixteen,
        ColorType::Indexed,
        BitDepth::Two,
    );
}

#[test]
fn rgb_8_should_be_palette_2() {
    test_it_converts(
        "tests/files/rgb_8_should_be_palette_2.png",
        false,
        ColorType::RGB,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::Two,
    );
}

#[test]
fn rgb_16_should_be_palette_1() {
    test_it_converts(
        "tests/files/rgb_16_should_be_palette_1.png",
        false,
        ColorType::RGB,
        BitDepth::Sixteen,
        ColorType::Indexed,
        BitDepth::One,
    );
}

#[test]
fn rgb_8_should_be_palette_1() {
    test_it_converts(
        "tests/files/rgb_8_should_be_palette_1.png",
        false,
        ColorType::RGB,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::One,
    );
}

#[test]
fn rgb_16_should_be_grayscale_16() {
    test_it_converts(
        "tests/files/rgb_16_should_be_grayscale_16.png",
        false,
        ColorType::RGB,
        BitDepth::Sixteen,
        ColorType::Grayscale,
        BitDepth::Sixteen,
    );
}

#[test]
fn rgb_16_should_be_grayscale_8() {
    test_it_converts(
        "tests/files/rgb_16_should_be_grayscale_8.png",
        false,
        ColorType::RGB,
        BitDepth::Sixteen,
        ColorType::Grayscale,
        BitDepth::Eight,
    );
}

#[test]
fn rgb_8_should_be_grayscale_8() {
    test_it_converts(
        "tests/files/rgb_8_should_be_grayscale_8.png",
        false,
        ColorType::RGB,
        BitDepth::Eight,
        ColorType::Grayscale,
        BitDepth::Eight,
    );
}

#[test]
fn palette_8_should_be_palette_8() {
    test_it_converts(
        "tests/files/palette_8_should_be_palette_8.png",
        false,
        ColorType::Indexed,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::Eight,
    );
}

#[test]
fn palette_8_should_be_palette_4() {
    test_it_converts(
        "tests/files/palette_8_should_be_palette_4.png",
        false,
        ColorType::Indexed,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::Four,
    );
}

#[test]
fn palette_4_should_be_palette_4() {
    test_it_converts(
        "tests/files/palette_4_should_be_palette_4.png",
        false,
        ColorType::Indexed,
        BitDepth::Four,
        ColorType::Indexed,
        BitDepth::Four,
    );
}

#[test]
fn palette_8_should_be_palette_2() {
    test_it_converts(
        "tests/files/palette_8_should_be_palette_2.png",
        false,
        ColorType::Indexed,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::Two,
    );
}

#[test]
fn palette_4_should_be_palette_2() {
    test_it_converts(
        "tests/files/palette_4_should_be_palette_2.png",
        false,
        ColorType::Indexed,
        BitDepth::Four,
        ColorType::Indexed,
        BitDepth::Two,
    );
}

#[test]
fn palette_2_should_be_palette_2() {
    test_it_converts(
        "tests/files/palette_2_should_be_palette_2.png",
        false,
        ColorType::Indexed,
        BitDepth::Two,
        ColorType::Indexed,
        BitDepth::Two,
    );
}

#[test]
fn palette_8_should_be_palette_1() {
    test_it_converts(
        "tests/files/palette_8_should_be_palette_1.png",
        false,
        ColorType::Indexed,
        BitDepth::Eight,
        ColorType::Indexed,
        BitDepth::One,
    );
}

#[test]
fn palette_4_should_be_palette_1() {
    test_it_converts(
        "tests/files/palette_4_should_be_palette_1.png",
        false,
        ColorType::Indexed,
        BitDepth::Four,
        ColorType::Indexed,
        BitDepth::One,
    );
}

#[test]
fn palette_2_should_be_palette_1() {
    test_it_converts(
        "tests/files/palette_2_should_be_palette_1.png",
        false,
        ColorType::Indexed,
        BitDepth::Two,
        ColorType::Indexed,
        BitDepth::One,
    );
}

#[test]
fn palette_1_should_be_palette_1() {
    test_it_converts(
        "tests/files/palette_1_should_be_palette_1.png",
        false,
        ColorType::Indexed,
        BitDepth::One,
        ColorType::Indexed,
        BitDepth::One,
    );
}

#[test]
fn grayscale_alpha_16_should_be_grayscale_alpha_16() {
    test_it_converts(
        "tests/files/grayscale_alpha_16_should_be_grayscale_alpha_16.png",
        false,
        ColorType::GrayscaleAlpha,
        BitDepth::Sixteen,
        ColorType::GrayscaleAlpha,
        BitDepth::Sixteen,
    );
}

#[test]
fn grayscale_alpha_16_should_be_grayscale_alpha_8() {
    test_it_converts(
        "tests/files/grayscale_alpha_16_should_be_grayscale_alpha_8.png",
        false,
        ColorType::GrayscaleAlpha,
        BitDepth::Sixteen,
        ColorType::GrayscaleAlpha,
        BitDepth::Eight,
    );
}

#[test]
fn grayscale_alpha_8_should_be_grayscale_alpha_8() {
    test_it_converts(
        "tests/files/grayscale_alpha_8_should_be_grayscale_alpha_8.png",
        false,
        ColorType::GrayscaleAlpha,
        BitDepth::Eight,
        ColorType::GrayscaleAlpha,
        BitDepth::Eight,
    );
}

#[test]
fn grayscale_alpha_16_should_be_grayscale_16() {
    test_it_converts(
        "tests/files/grayscale_alpha_16_should_be_grayscale_16.png",
        false,
        ColorType::GrayscaleAlpha,
        BitDepth::Sixteen,
        ColorType::Grayscale,
        BitDepth::Sixteen,
    );
}

#[test]
fn grayscale_alpha_16_should_be_grayscale_8() {
    test_it_converts(
        "tests/files/grayscale_alpha_16_should_be_grayscale_8.png",
        false,
        ColorType::GrayscaleAlpha,
        BitDepth::Sixteen,
        ColorType::Grayscale,
        BitDepth::Eight,
    );
}

#[test]
fn grayscale_alpha_8_should_be_grayscale_8() {
    test_it_converts(
        "tests/files/grayscale_alpha_8_should_be_grayscale_8.png",
        false,
        ColorType::GrayscaleAlpha,
        BitDepth::Eight,
        ColorType::Grayscale,
        BitDepth::Eight,
    );
}

#[test]
fn grayscale_16_should_be_grayscale_16() {
    test_it_converts(
        "tests/files/grayscale_16_should_be_grayscale_16.png",
        false,
        ColorType::Grayscale,
        BitDepth::Sixteen,
        ColorType::Grayscale,
        BitDepth::Sixteen,
    );
}

#[test]
fn grayscale_16_should_be_grayscale_8() {
    test_it_converts(
        "tests/files/grayscale_16_should_be_grayscale_8.png",
        false,
        ColorType::Grayscale,
        BitDepth::Sixteen,
        ColorType::Grayscale,
        BitDepth::Eight,
    );
}

#[test]
fn grayscale_8_should_be_grayscale_8() {
    test_it_converts(
        "tests/files/grayscale_8_should_be_grayscale_8.png",
        false,
        ColorType::Grayscale,
        BitDepth::Eight,
        ColorType::Grayscale,
        BitDepth::Eight,
    );
}

#[test]
fn grayscale_8_should_be_grayscale_4() {
    test_it_converts(
        "tests/files/grayscale_8_should_be_grayscale_4.png",
        false,
        ColorType::Grayscale,
        BitDepth::Eight,
        ColorType::Grayscale,
        BitDepth::Four,
    );
}

#[test]
fn grayscale_8_should_be_grayscale_2() {
    test_it_converts(
        "tests/files/grayscale_8_should_be_grayscale_2.png",
        false,
        ColorType::Grayscale,
        BitDepth::Eight,
        ColorType::Grayscale,
        BitDepth::Two,
    );
}

#[test]
fn grayscale_4_should_be_grayscale_2() {
    test_it_converts(
        "tests/files/grayscale_4_should_be_grayscale_2.png",
        false,
        ColorType::Grayscale,
        BitDepth::Four,
        ColorType::Grayscale,
        BitDepth::Two,
    );
}

#[test]
fn grayscale_8_should_be_grayscale_1() {
    test_it_converts(
        "tests/files/grayscale_8_should_be_grayscale_1.png",
        false,
        ColorType::Grayscale,
        BitDepth::Eight,
        ColorType::Grayscale,
        BitDepth::One,
    );
}

#[test]
fn grayscale_4_should_be_grayscale_1() {
    test_it_converts(
        "tests/files/grayscale_4_should_be_grayscale_1.png",
        false,
        ColorType::Grayscale,
        BitDepth::Four,
        ColorType::Grayscale,
        BitDepth::One,
    );
}

#[test]
fn grayscale_2_should_be_grayscale_1() {
    test_it_converts(
        "tests/files/grayscale_2_should_be_grayscale_1.png",
        false,
        ColorType::Grayscale,
        BitDepth::Two,
        ColorType::Grayscale,
        BitDepth::One,
    );
}

#[test]
fn grayscale_alpha_16_should_be_grayscale_trns_16() {
    test_it_converts(
        "tests/files/grayscale_alpha_16_should_be_grayscale_trns_16.png",
        true,
        ColorType::GrayscaleAlpha,
        BitDepth::Sixteen,
        ColorType::Grayscale,
        BitDepth::Sixteen,
    );
}

#[test]
fn grayscale_alpha_8_should_be_grayscale_trns_8() {
    test_it_converts(
        "tests/files/grayscale_alpha_8_should_be_grayscale_trns_8.png",
        true,
        ColorType::GrayscaleAlpha,
        BitDepth::Eight,
        ColorType::Grayscale,
        BitDepth::Eight,
    );
}

#[test]
fn small_files() {
    let input = PathBuf::from("tests/files/small_files.png");
    let (output, opts) = get_opts(&input);

    let png = PngData::new(&input, opts.fix_errors).unwrap();

    assert_eq!(png.raw.ihdr.color_type, ColorType::Indexed);
    assert_eq!(png.raw.ihdr.bit_depth, BitDepth::Eight);

    match oxipng::optimize(&InFile::Path(input), &output, &opts) {
        Ok(_) => (),
        Err(x) => panic!("{}", x),
    };
    let output = output.path().unwrap();
    assert!(output.exists());

    let png = match PngData::new(output, opts.fix_errors) {
        Ok(x) => x,
        Err(x) => {
            remove_file(&output).ok();
            panic!("{}", x)
        }
    };

    assert_eq!(png.raw.ihdr.color_type, ColorType::Indexed);
    // depth varies depending on zlib implementation used

    remove_file(output).ok();
}

#[test]
fn palette_should_be_reduced_with_dupes() {
    let input = PathBuf::from("tests/files/palette_should_be_reduced_with_dupes.png");
    let (output, opts) = get_opts(&input);

    let png = PngData::new(&input, opts.fix_errors).unwrap();

    assert_eq!(png.raw.ihdr.color_type, ColorType::Indexed);
    assert_eq!(png.raw.ihdr.bit_depth, BitDepth::Eight);
    assert_eq!(png.raw.palette.as_ref().unwrap().len(), 43);

    match oxipng::optimize(&InFile::Path(input), &output, &opts) {
        Ok(_) => (),
        Err(x) => panic!("{}", x),
    };
    let output = output.path().unwrap();
    assert!(output.exists());

    let png = match PngData::new(output, opts.fix_errors) {
        Ok(x) => x,
        Err(x) => {
            remove_file(&output).ok();
            panic!("{}", x)
        }
    };

    assert_eq!(png.raw.ihdr.color_type, ColorType::Indexed);
    assert_eq!(png.raw.ihdr.bit_depth, BitDepth::Eight);
    assert_eq!(png.raw.palette.as_ref().unwrap().len(), 35);

    remove_file(output).ok();
}

#[test]
fn palette_should_be_reduced_with_unused() {
    let input = PathBuf::from("tests/files/palette_should_be_reduced_with_unused.png");
    let (output, opts) = get_opts(&input);

    let png = PngData::new(&input, opts.fix_errors).unwrap();

    assert_eq!(png.raw.ihdr.color_type, ColorType::Indexed);
    assert_eq!(png.raw.ihdr.bit_depth, BitDepth::Eight);
    assert_eq!(png.raw.palette.as_ref().unwrap().len(), 35);

    match oxipng::optimize(&InFile::Path(input), &output, &opts) {
        Ok(_) => (),
        Err(x) => panic!("{}", x),
    };
    let output = output.path().unwrap();
    assert!(output.exists());

    let png = match PngData::new(output, opts.fix_errors) {
        Ok(x) => x,
        Err(x) => {
            remove_file(&output).ok();
            panic!("{}", x)
        }
    };

    assert_eq!(png.raw.ihdr.color_type, ColorType::Indexed);
    assert_eq!(png.raw.ihdr.bit_depth, BitDepth::Eight);
    assert_eq!(png.raw.palette.as_ref().unwrap().len(), 33);

    remove_file(output).ok();
}

#[test]
fn palette_should_be_reduced_with_both() {
    let input = PathBuf::from("tests/files/palette_should_be_reduced_with_both.png");
    let (output, opts) = get_opts(&input);

    let png = PngData::new(&input, opts.fix_errors).unwrap();

    assert_eq!(png.raw.ihdr.color_type, ColorType::Indexed);
    assert_eq!(png.raw.ihdr.bit_depth, BitDepth::Eight);
    assert_eq!(png.raw.palette.as_ref().unwrap().len(), 43);

    match oxipng::optimize(&InFile::Path(input), &output, &opts) {
        Ok(_) => (),
        Err(x) => panic!("{}", x),
    };
    let output = output.path().unwrap();
    assert!(output.exists());

    let png = match PngData::new(output, opts.fix_errors) {
        Ok(x) => x,
        Err(x) => {
            remove_file(&output).ok();
            panic!("{}", x)
        }
    };

    assert_eq!(png.raw.ihdr.color_type, ColorType::Indexed);
    assert_eq!(png.raw.ihdr.bit_depth, BitDepth::Eight);
    assert_eq!(png.raw.palette.as_ref().unwrap().len(), 33);

    remove_file(output).ok();
}

#[test]
fn rgba_16_reduce_alpha() {
    test_it_converts(
        "tests/files/rgba_16_reduce_alpha.png",
        true,
        ColorType::RGBA,
        BitDepth::Sixteen,
        ColorType::RGBA,
        BitDepth::Eight,
    );
}

#[test]
fn rgba_8_reduce_alpha() {
    test_it_converts(
        "tests/files/rgba_8_reduce_alpha.png",
        true,
        ColorType::RGBA,
        BitDepth::Eight,
        ColorType::RGBA,
        BitDepth::Eight,
    );
}

#[test]
fn grayscale_alpha_16_reduce_alpha() {
    test_it_converts(
        "tests/files/grayscale_alpha_16_reduce_alpha.png",
        true,
        ColorType::GrayscaleAlpha,
        BitDepth::Sixteen,
        ColorType::GrayscaleAlpha,
        BitDepth::Eight,
    );
}

#[test]
fn grayscale_alpha_8_reduce_alpha() {
    test_it_converts(
        "tests/files/grayscale_alpha_8_reduce_alpha.png",
        true,
        ColorType::GrayscaleAlpha,
        BitDepth::Eight,
        ColorType::GrayscaleAlpha,
        BitDepth::Eight,
    );
}
