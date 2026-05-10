mod bdf;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let path_in = "/Users/p4/Downloads/jiskan24-UnicodeFull-rename-24.bdf";
    // let path_in = "/Users/p4/Downloads/MSX-FONTVer-0.8Jpn-16.bdf";
    let path_out = "/Users/p4/Documents/raspico-playground/src/font/jiskan24.bin";
    let contents_in = std::fs::read(path_in).context("read")?;
    let contents_out = bdf::convert(&contents_in).context(path_in)?;
    std::fs::write(path_out, contents_out).context("write")?;
    Ok(())
}
