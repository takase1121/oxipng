#![feature(test)]

extern crate oxipng;
extern crate test;

use std::path::PathBuf;
use test::Bencher;

use oxipng::internal_tests::*;

#[bench]
fn deflate_16_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgb_16_should_be_rgb_16.png"));
    let png = PngData::new(&input, false).unwrap();

    b.iter(|| {
        let min = AtomicMin::new(None);
        deflate(png.raw.data.as_ref(), 12, &min)
    });
}

#[bench]
fn deflate_8_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgb_8_should_be_rgb_8.png"));
    let png = PngData::new(&input, false).unwrap();

    b.iter(|| {
        let min = AtomicMin::new(None);
        deflate(png.raw.data.as_ref(), 12, &min)
    });
}

#[bench]
fn deflate_4_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from(
        "tests/files/palette_4_should_be_palette_4.png",
    ));
    let png = PngData::new(&input, false).unwrap();

    b.iter(|| {
        let min = AtomicMin::new(None);
        deflate(png.raw.data.as_ref(), 12, &min)
    });
}

#[bench]
fn deflate_2_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from(
        "tests/files/palette_2_should_be_palette_2.png",
    ));
    let png = PngData::new(&input, false).unwrap();

    b.iter(|| {
        let min = AtomicMin::new(None);
        deflate(png.raw.data.as_ref(), 12, &min)
    });
}

#[bench]
fn deflate_1_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from(
        "tests/files/palette_1_should_be_palette_1.png",
    ));
    let png = PngData::new(&input, false).unwrap();

    b.iter(|| {
        let min = AtomicMin::new(None);
        deflate(png.raw.data.as_ref(), 12, &min)
    });
}

#[bench]
fn inflate_generic(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgb_16_should_be_rgb_16.png"));
    let png = PngData::new(&input, false).unwrap();

    b.iter(|| inflate(png.idat_data.as_ref(), png.raw.ihdr.raw_data_size()));
}
