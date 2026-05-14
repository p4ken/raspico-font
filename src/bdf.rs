use anyhow::Context;

// bdf_reader はMSX_FONTで様々なエラーが出て手に負えなかったので、手実装。
// 例えば METRICSSET 2 がエラーを出す。
pub fn convert(bdf: &[u8]) -> anyhow::Result<Vec<u8>> {
    Font::from_bdf(bdf).context("parse")?.to_p4()
}

struct Font {
    header: Header,
    glyphs: Vec<Glyph>,
}

impl Font {
    fn from_bdf(bdf: &[u8]) -> anyhow::Result<Self> {
        let lines = std::str::from_utf8(bdf)
            .context("utf8")?
            .lines()
            .collect::<Vec<_>>();
        anyhow::ensure!(lines.first().is_some_and(|l| l.starts_with("STARTFONT ")));

        let mut split_lines = lines
            .strip_suffix(&["ENDFONT"])
            .context("no ENDFONT")?
            .splitn(2, |l| l.starts_with("CHARS "));
        let header = Header::from_lines(split_lines.next().unwrap())?;
        let chars_lines = split_lines.next().context("no CHARS")?;

        let mut glyphs = Vec::new();
        for glyph_lines in chars_lines.split_inclusive(|l| *l == "ENDCHAR") {
            let glyph = Glyph::from_lines(glyph_lines).context("glyph")?;
            glyphs.push(glyph);
        }

        Ok(Self { header, glyphs })
    }

    fn to_p4(&self) -> anyhow::Result<Vec<u8>> {
        let mut out = Vec::new();
        for glyph in &self.glyphs {
            out.extend(glyph.to_p4(&self.header)?);
        }
        Ok(out)
    }
}

struct Header {
    width: usize,
    height: usize,
}

impl Header {
    fn from_lines(lines: &[&str]) -> anyhow::Result<Self> {
        let mut bbox = lines
            .iter()
            .filter_map(|l| l.strip_prefix("FONTBOUNDINGBOX "))
            .next()
            .context("FONTBOUNDINGBOX")?
            .split_whitespace();

        let width = bbox.next().context("width")?.parse()?;
        let height = bbox.next().context("height")?.parse()?;

        Ok(Header { width, height })
    }
}

#[derive(Default)]
struct Glyph {
    encoding: u16,
    bitmap: Vec<[u8; 4]>,
}

impl Glyph {
    fn from_lines(lines: &[&str]) -> anyhow::Result<Self> {
        anyhow::ensure!(lines.first().is_some_and(|l| l.starts_with("STARTCHAR ")));
        let lines = lines.strip_suffix(&["ENDCHAR"]).context("no ENDCHAR")?;
        let mut split_lines = lines.splitn(2, |l| *l == "BITMAP");
        let meta_lines = split_lines.next().unwrap();
        let bitmap_lines = split_lines.next().context("no BITMAP")?;
        let encoding = meta_lines
            .iter()
            .find_map(|l| l.strip_prefix("ENCODING "))
            .context("no ENCODING")?
            .parse::<u16>()
            .context("ENCODING value")?;

        // 例：「次」
        // ENCODING 27425 (0x6b21) = UTF-16
        if encoding != 27425 {
            return Ok(Glyph::default());
        }

        let mut bitmap = Vec::new();
        for line in bitmap_lines {
            // hex表現の文字列 → 0,1のビット列
            bitmap.push(u32::from_str_radix(*line, 16).context("hex")?.to_be_bytes());
        }

        Ok(Glyph { encoding, bitmap })
    }

    fn to_p4(&self, header: &Header) -> anyhow::Result<Vec<u8>> {
        let mut out = Vec::new();
        let line_byte_offset = 4 - (header.width / 8);
        for line_bytes in &self.bitmap {
            let line_slice = line_bytes.get(line_byte_offset..4).context("slice")?;

            #[cfg(debug_assertions)]
            {
                for byte in line_slice {
                    print!("{:08b}", byte);
                }
                println!();
            }

            out.extend_from_slice(&line_slice);
        }
        Ok(out)
    }
}
