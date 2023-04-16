use std::fs::{self, create_dir_all};

#[cfg(feature = "resize")]
use opencv::{
    core::{Size, Vector},
    imgcodecs, imgproc,
    prelude::Mat,
};
use tesseract::Tesseract;

const IMG: &str = "rust_lang_org_eng1.png";
const LANG: &str = "eng";

#[cfg(feature = "perf_logging")]
fn measure_millis<T>(msg: &str, f: impl FnOnce() -> T) -> T {
    let start = std::time::Instant::now();
    let t = f();
    let duration = std::time::Instant::now() - start;
    log::info!("{msg} took: {}ms", duration.as_millis());
    t
}

#[cfg(feature = "resize")]
fn resize(img: &str, scale: f64, read_flags: i32) -> eyre::Result<Mat> {
    let src = imgcodecs::imread(img, read_flags)?;
    let mut res = Mat::default();

    // interpolation is chosen based on the documentation of imgproc::resize
    let interpolation = if scale > 1. {
        imgproc::INTER_CUBIC
    } else {
        imgproc::INTER_AREA
    };
    // take the image (src), and scale it up by given scale
    imgproc::resize(&src, &mut res, Size::default(), scale, scale, interpolation)?;
    Ok(res)
}

fn save(name: &str, src: Mat) -> String {
    let out_img = format!("resized_{name}");
    match imgcodecs::imwrite(&out_img, &src, &Vector::default()) {
        Ok(true) => out_img,
        failed => panic!("no success on writing file: {failed:?}"),
    }
}

fn main() -> eyre::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let img = IMG.to_string();

    #[cfg(feature = "resize")]
    let img = {
        log::info!("resizing: {img}");
        let resize = || resize(&img, 3., imgcodecs::IMREAD_COLOR);

        #[cfg(feature = "perf_logging")]
        let res = measure_millis("resize", resize)?;
        #[cfg(not(feature = "perf_logging"))]
        let res = resize()?;

        let save = || save(&img, res);
        if cfg!(feature = "perf_logging") {
            measure_millis("save", save)
        } else {
            save()
        }
    };

    let tess = Tesseract::new(None, Some(LANG))?
        .set_variable("user_defined_dpi", "200")?
        .set_image(&img)?;

    // trigger the recognition so that we can measure it separated
    let recognize = || tess.recognize();
    #[cfg(feature = "perf_logging")]
    let mut tess = measure_millis("recognize", recognize)?;
    #[cfg(not(feature = "perf_logging"))]
    let mut tess = recognize()?;

    let text = ("text", "txt", tess.get_text()?);
    let hocr = ("hocr", "html", tess.get_hocr_text(0)?);
    let tsv = ("tsv", "csv", tess.get_tsv_text(0)?);

    let filename = img
        .split('.')
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .to_string();
    let dir = &format!("results/{LANG}/{filename}");
    create_dir_all(dir)?;

    vec![text, hocr, tsv]
        .into_iter()
        .for_each(|(name, ext, data)| {
            fs::write(format!("{dir}/{name}.{ext}"), data).expect("couldn't write file")
        });

    Ok(())
}
