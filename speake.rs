use std::process::Command;
fn main() {
    Command::new("espeak-ng")
        .arg("-v")
        .arg("en+f4")
        .arg("-s175")
        .arg("-p75")
        .arg("-a300")
        .arg("-k0")
        .arg("hello world,this is luna")
        .output()
        .expect("error");
}
