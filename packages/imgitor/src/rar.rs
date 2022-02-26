use std::path;
use std::process::Command;

pub fn extract(f: &str, p: &path::PathBuf) -> path::PathBuf {
    Command::new("7z")
        .arg("x")
        .arg("-o./lake/")
        .arg(f.to_string())
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
