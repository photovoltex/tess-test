use std::fs;

use opencv::prelude::MatTraitConst;
#[cfg(feature = "resize")]
use opencv::{
    core::{Size, Vector},
    imgcodecs, imgproc,
    prelude::Mat,
};
#[cfg(feature = "generate_pdf")]
use printpdf::{Image, ImageTransform, Mm, PdfDocument};
use tesseract::Tesseract;

const IMG: &str = "test2.png";
const LANG: &str = "eng";

#[cfg(feature = "perf_logging")]
fn measure_millis<T>(msg: &str, f: impl FnOnce() -> T) -> T {
    let start = std::time::Instant::now();
    let t = f();
    let duration = std::time::Instant::now() - start;
    log::info!("{msg} took: {}ms", duration.as_millis());
    t
}

fn log_img(msg: &str, img: Option<&str>, x: i32, y: i32) {
    let img = img.map(|img| format!("(img: {img})")).unwrap_or_default();
    log::info!("{msg}{img}: x({x}):y({y})");
}

#[cfg(feature = "resize")]
fn resize(img: &str, scale: f64, read_flags: i32) -> eyre::Result<Mat> {
    let src = imgcodecs::imread(img, read_flags)?;
    log_img("source", Some(img), src.cols(), src.rows());

    let mut res = Mat::default();

    // interpolation is chosen based on the documentation of imgproc::resize
    let interpolation = if scale > 1. {
        imgproc::INTER_CUBIC
    } else {
        imgproc::INTER_AREA
    };
    // take the image (src), and scale it up by given scale
    imgproc::resize(&src, &mut res, Size::default(), scale, scale, interpolation)?;
    log_img("resized", None, res.cols(), res.rows());
    Ok(res)
}

#[cfg(feature = "opencv")]
fn save(name: &str, src: Mat) -> String {
    match imgcodecs::imwrite(name, &src, &Vector::default()) {
        Ok(true) => name.to_string(),
        failed => panic!("no success on writing file: {failed:?}"),
    }
}

fn main() -> eyre::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let img = IMG.to_string();

    // create directory to save the results
    let split_file = img.split('.').collect::<Vec<_>>();

    let (filename, ext) = (
        split_file.first().unwrap().to_string(),
        split_file.last().unwrap().to_string(),
    );
    let dir = &format!("results/{LANG}/{filename}");
    fs::create_dir_all(dir)?;

    #[cfg(feature = "resize")]
    let img = {
        log::info!("resizing: {img}");
        let resize = || resize(&img, 3., imgcodecs::IMREAD_COLOR);

        #[cfg(feature = "perf_logging")]
        let res = measure_millis("resize", resize)?;
        #[cfg(not(feature = "perf_logging"))]
        let res = resize()?;

        let resized_img = format!("{dir}/resized.{ext}");
        let save = || save(&resized_img, res);

        #[cfg(feature = "perf_logging")]
        let out = measure_millis("save", save);
        #[cfg(not(feature = "perf_logging"))]
        let out = save();

        out
    };

    // init tesseract
    let tess = Tesseract::new(None, Some(LANG))?
        .set_variable("user_defined_dpi", "200")?
        .set_image(&img)?;

    // trigger the recognition so that we can measure it separated
    let recognize = || tess.recognize();
    #[cfg(feature = "perf_logging")]
    let mut tess = measure_millis("recognize", recognize)?;
    #[cfg(not(feature = "perf_logging"))]
    let mut tess = recognize()?;

    // retrieve recognized data from tesseract (filename, ext, data)
    let text = ("text", "txt", tess.get_text()?);
    let hocr = ("hocr", "html", tess.get_hocr_text(0)?);
    let tsv = ("tsv", "csv", tess.get_tsv_text(0)?);

    // save the recognized data from tesseract
    vec![text, hocr, tsv]
        .into_iter()
        .for_each(|(filename, ext, data)| {
            fs::write(format!("{dir}/{filename}.{ext}"), data).expect("couldn't write file")
        });

    #[cfg(feature = "generate_pdf")]
    let generate_pdf = || -> eyre::Result<()> {
        let mut img_file = fs::File::open(&img)?;
        let image = Image::try_from(image::codecs::png::PngDecoder::new(&mut img_file)?)?;

        let width = image.image.width.0.try_into()?;
        let height = image.image.height.0.try_into()?;
        log_img("source", Some(&img), width, height);

        let width = Mm(width.try_into()?);
        let height = Mm(height.try_into()?);

        let (doc, page1, layer1) = PdfDocument::new(&img, width, height, &img);
        let layer = doc.get_page(page1).get_layer(layer1);

        image.add_to_layer(
            layer,
            ImageTransform {
                // todo: no magic number plz
                dpi: Some(25.4),
                ..Default::default()
            },
        );

        // todo: embed the extracted ocr data into the pdf
        doc.save(&mut std::io::BufWriter::new(
            fs::File::create(format!("{dir}/{filename}.pdf")).unwrap(),
        ))?;

        Ok(())
    };

    #[cfg(feature = "generate_pdf")]
    {
        #[cfg(feature = "perf_logging")]
        measure_millis("generate pdf", generate_pdf)?;
        #[cfg(not(feature = "perf_logging"))]
        generate_pdf()?;
    }

    Ok(())
}
