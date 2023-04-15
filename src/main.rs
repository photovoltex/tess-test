use std::fs::{self, create_dir_all};

use opencv::{imgcodecs, imgproc, prelude::Mat, core::{Size, Vector}};
use tesseract::Tesseract;

const IMG: &str = "rust_lang_org_eng1.png";
const LANG: &str = "eng";

const RESIZING: bool = false;

fn main() -> eyre::Result<()> {
    let resized_img = if RESIZING {
        let src = imgcodecs::imread(IMG, imgcodecs::IMREAD_COLOR)?;
        let mut res = Mat::default();
    
        imgproc::resize(&src, &mut res, Size::default(), 3., 3., imgproc::INTER_NEAREST)?;
    
        let out_img = format!("resized_{IMG}");
        match imgcodecs::imwrite(&out_img, &res,  &Vector::default()) {
            Ok(true) => out_img,
            failed => panic!("no success on writing file: {failed:?}")
        }
    } else {
        IMG.to_string()
    };

    let tess_process_img = resized_img;

    let mut tess= Tesseract::new(None, Some(LANG))?
        .set_variable("user_defined_dpi", "200")?
        .set_image(&tess_process_img)?;

    let text = ("text", "txt", tess.get_text()?);
    let hocr = ("hocr", "html", tess.get_hocr_text(0)?);
    let tsv = ("tsv", "csv", tess.get_tsv_text(0)?);

    let filename = tess_process_img.split('.').collect::<Vec<_>>().first().unwrap().to_string();
    let dir = &format!("results/{LANG}/{filename}");
    create_dir_all(dir)?;

    vec![text, hocr, tsv]
        .into_iter()
        .for_each(|(name, ext, data)| {
            fs::write(format!("{dir}/{name}.{ext}"), data).expect("couldn't write file")
        });

    Ok(())
}
