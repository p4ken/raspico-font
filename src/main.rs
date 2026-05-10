use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let path_in = "/Users/p4/Downloads/jiskan24-UnicodeFull-rename-24.bdf";
    // let path_in = "/Users/p4/Downloads/MSX-FONTVer-0.8Jpn-16.bdf";
    let path_out = "/Users/p4/Documents/raspico-playground/src/font/jiskan24.bin";
    let contents_in = std::fs::read(path_in).context("read")?;
    let contents_out = bdf_to_rs(&contents_in).context("bdf")?;
    std::fs::write(path_out, contents_out).context("write")?;
    Ok(())
}

// fn bdf_to_rs(bdf: &[u8]) -> anyhow::Result<Vec<u8>> {
//     let font = bdf_reader::Font::read(bdf)?;
//     let glyph = font.glyph(27425).unwrap();
//     println!("{}", glyph.bitmap().ascii_art());
//     Ok(Vec::new())
// }

// bdf_reader はMSX_FONTで様々なエラーが出て手に負えなかったので、手実装。
// 例えば METRICSSET 2 がエラーを出す。
fn bdf_to_rs(bdf: &[u8]) -> anyhow::Result<Vec<u8>> {
    let lines = std::str::from_utf8(bdf)
        .context("utf8")?
        .lines()
        .collect::<Vec<_>>();

    // 例：「次」
    // ENCODING 27425 (0x6b21)
    for glyph in lines.split(|l| *l == "ENDCHAR") {
        if !glyph.contains(&"ENCODING 27425") {
            continue;
        }
        let mut found_bitmap = false;
        for line in glyph {
            if *line == "BITMAP" {
                found_bitmap = true;
                continue;
            }
            if found_bitmap {
                let line_bin = u32::from_str_radix(*line, 16).context("hex")?;
                println!("{:0>32b}", line_bin);
            }
        }
    }
    Ok(Vec::new())
}
