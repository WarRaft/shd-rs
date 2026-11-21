use super::*;

#[test]
fn reads_and_draws() {
    let s = parse(&[0, 255, 0, 0, 0, 0, 255, 255], 4, 2).unwrap();
    assert_eq!(s.cell(1, 0), Some(255));
    assert_eq!(s.to_bytes(), [0, 255, 0, 0, 0, 0, 255, 255]);
    // North up: the second row first.
    let rgba = s.to_rgba();
    assert_eq!(&rgba[8..12], &[40, 40, 40, 255]);
    assert_eq!(&rgba[16..20], &[230, 230, 230, 255]);
    assert_eq!(size_for_terrain(97, 97), (384, 384));
    assert!(parse(&[0; 7], 4, 2).unwrap_err().to_string().contains("7 bytes for a 4x2 grid"));
}

/// Every map's shadows have their terrain's size: `SHD_CORPUS` is a
/// folder of `<map>/war3map.shd` and `war3map.w3e`.
#[test]
#[ignore]
fn corpus() {
    let dir = std::env::var("SHD_CORPUS").expect("SHD_CORPUS");
    let (mut n, mut bad) = (0, Vec::new());
    for entry in std::fs::read_dir(dir).unwrap() {
        let folder = entry.unwrap().path();
        let (Ok(bytes), Ok(w3e)) = (std::fs::read(folder.join("war3map.shd")), std::fs::read(folder.join("war3map.w3e"))) else { continue };
        n += 1;
        let u32_at = |i: usize| u32::from_le_bytes(w3e[i..i + 4].try_into().unwrap());
        let ground = u32_at(13) as usize;
        let cliffs = u32_at(17 + 4 * ground) as usize;
        let at = 21 + 4 * ground + 4 * cliffs;
        let (w, h) = size_for_terrain(u32_at(at), u32_at(at + 4));
        match parse(&bytes, w, h) {
            Ok(s) if s.to_bytes() == bytes => {}
            Ok(_) => bad.push(format!("{}: differs", folder.display())),
            Err(e) => bad.push(format!("{}: {e}", folder.display())),
        }
    }
    println!("{n} files");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
