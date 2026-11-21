//! `shd-rs`: checks a shadow map against its terrain.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [path, w, h] = args.as_slice() else {
        eprintln!("usage: shd-rs <war3map.shd> <terrain points across> <terrain points up>");
        eprintln!("  (war3map.w3e's width and height: w3e-rs --info shows them)");
        return ExitCode::FAILURE;
    };
    let (Ok(w), Ok(h)) = (w.parse(), h.parse()) else {
        eprintln!("the terrain's size is two whole numbers");
        return ExitCode::FAILURE;
    };
    let (w, h) = shd_rs::size_for_terrain(w, h);
    match std::fs::read(path).map_err(|e| e.to_string()).and_then(|b| shd_rs::parse(&b, w, h).map_err(|e| e.to_string())) {
        Ok(s) => {
            let shadowed = s.cells.iter().filter(|&&c| c != 0).count();
            println!("{path}: {w}x{h} cells, {shadowed} in shadow");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{path}: {e}");
            ExitCode::FAILURE
        }
    }
}
