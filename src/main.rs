use std::fs::{self, create_dir_all};

use tesseract::Tesseract;

const IMG: &str = "rust_lang_org_eng1.png";
const LANG: &str = "eng";

fn main() -> eyre::Result<()> {
    let mut tes= Tesseract::new(None, Some(LANG))?.set_image(IMG)?;

    let text = ("text", "txt", tes.get_text()?);
    let hocr = ("hocr", "html", tes.get_hocr_text(0)?);
    let tsv = ("tsv", "csv", tes.get_tsv_text(0)?);

    let dir = &format!("results/{LANG}");
    create_dir_all(dir)?;

    vec![text, hocr, tsv]
        .into_iter()
        .for_each(|(name, ext, data)| {
            fs::write(format!("{dir}/{name}.{ext}"), data).expect("couldn't write file")
        });

    Ok(())
}
