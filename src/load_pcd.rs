use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rgb: u32, // 内部的な色データ（RGBがpackedされたu32）
}

// PCDファイルを読み込み、Point構造体のVecにパースする関数
fn read_pcd_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Point>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut points = Vec::new();
    let mut data_section = false;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // ヘッダー部分は "DATA" 行が出るまでスキップする
        if !data_section {
            if line.starts_with("DATA") {
                data_section = true;
            }
            continue;
        }

        // データ行：空白で区切られているので、4つの値を想定（x, y, z, rgb）
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() < 4 {
            continue;
        }

        // 各値をパース
        let x: f64 = tokens[0].parse().unwrap_or(0.0);
        let y: f64 = tokens[1].parse().unwrap_or(0.0);
        let z: f64 = tokens[2].parse().unwrap_or(0.0);
        let rgb_float: f32 = tokens[3].parse().unwrap_or(0.0);
        // rgbはf32として読み込まれているが、内部的にはu32のビットパターン
        let rgb = rgb_float.to_bits();

        points.push(Point { x, y, z, rgb });
    }

    Ok(points)
}

// 色データを分解するヘルパー関数（必要に応じて）
fn extract_rgb_components(rgb: u32) -> (u8, u8, u8) {
    let r = ((rgb >> 16) & 0xFF) as u8;
    let g = ((rgb >> 8) & 0xFF) as u8;
    let b = (rgb & 0xFF) as u8;
    (r, g, b)
}

pub fn load_pcd() -> std::io::Result<(Vec<Point>)> {
    // 読み込み対象のPCDファイルパス（適宜変更してください）
    let pcd_file = "/home/kenji/workspace/research/pcd-viewer-server-02/data/ndt_result.pcd";
    let points = read_pcd_file(pcd_file)?;

    // 読み込んだ点群データの先頭の数点を表示
    for (i, p) in points.iter().enumerate().take(10) {
        let (r, g, b) = extract_rgb_components(p.rgb);
        println!(
            "Point {}: x = {}, y = {}, z = {}, rgb = 0x{:06X} (r={}, g={}, b={})",
            i, p.x, p.y, p.z, p.rgb, r, g, b
        );
    }
    println!("Total {} points loaded.", points.len());

    Ok(points)
}
