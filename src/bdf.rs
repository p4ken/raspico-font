use anyhow::Context;

// bdf_reader はMSX_FONTで様々なエラーが出て手に負えなかったので、手実装。
// 例えば METRICSSET 2 がエラーを出す。
pub fn convert(bdf: &[u8]) -> anyhow::Result<Vec<u8>> {
    let lines = std::str::from_utf8(bdf)
        .context("utf8")?
        .lines()
        .collect::<Vec<_>>();
    let header_size = lines
        .iter()
        .position(|l| l.starts_with("STARTCHAR "))
        .unwrap_or(lines.len());
    let header = Header::from_lines(&lines[..header_size])?;
    let line_byte_len = header.width / 8;

    let mut result = Vec::new();
    for glyph in lines[header_size..].split(|l| *l == "ENDCHAR") {
        // 例：「次」
        // ENCODING 27425 (0x6b21)
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
                // hex表現の文字列 → 01のビット列
                let line_bits = u32::from_str_radix(*line, 16).context("hex")?.to_le_bytes();
                let line_slice = line_bits.get(0..line_byte_len).context("slice")?;
                result.extend_from_slice(line_slice);
            }
        }
    }

    Ok(result)
}

struct Header {
    width: usize,
    height: usize,
}

impl Header {
    fn from_lines(lines: &[&str]) -> anyhow::Result<Self> {
        let mut font_bounding_box = lines
            .iter()
            .filter_map(|l| l.strip_prefix("FONTBOUNDINGBOX "))
            .next()
            .context("FONTBOUNDINGBOX")?
            .split_whitespace();

        let width = font_bounding_box.next().context("width")?.parse()?;
        let height = font_bounding_box.next().context("height")?.parse()?;

        Ok(Header { width, height })
    }
}
