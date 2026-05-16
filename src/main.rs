mod bdf;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let name = "jiskan24";
    let contents_in = read_origin(name)?;
    let contents_out = bdf::convert(&contents_in)?;
    write_rom(name, &contents_out)?;
    Ok(())
}

fn read_origin(name: &str) -> anyhow::Result<Vec<u8>> {
    let path = format!("./origin/{}.bdf", name);
    std::fs::read(&path).context(path)
}

fn write_rom(name: &str, bin: &[u8]) -> anyhow::Result<()> {
    let path = format!("../raspico-playground/rom/{}.bin", name);
    std::fs::write(&path, bin).context(path)
}
