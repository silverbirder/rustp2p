use std::path;
use std::process::Command;

pub fn extract(f: &path::PathBuf, p: &path::PathBuf) -> path::PathBuf {
    println!("extract");
    Command::new("7z")
        .arg("x")
        .arg(format!("-o{}", p.to_str().unwrap()))
        .arg(f)
        .arg("-aoa") // https://sevenzip.osdn.jp/chm/cmdline/switches/overwrite.htm
        .output()
        .expect("Failed to execute command");
    return p.to_path_buf();
}

pub fn compress(f: &path::PathBuf, p: &path::PathBuf) -> path::PathBuf {
    Command::new("7z")
        .arg("a")
        .arg("-sdel")
        .arg(format!("{}", p.to_str().unwrap()))
        .arg(format!("{}/*", f.to_str().unwrap()))
        .arg("-mx9")
        .output()
        .expect("Failed to execute command");
    return p.to_path_buf();
}

/*
もともと、以下のcrateを使っていた。
https://crates.io/crates/unrar
日本語の名前がファイルに含まれていると、Dockerのdebian内でrarファイルを展開できなかった。

例
* a.rar
  * ほげ.txt

rarを展開できるクレートを探したが、特に他に使えそうなものがなかった。
https://crates.io/search?q=rar

仕方がないが、unrarを外部コマンドとして実行する方針で実装した。
外部コマンドは、7zを使う。
*/
