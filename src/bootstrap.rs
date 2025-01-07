use std::process::Command;

use crate::{bot, caller::Mode, consts::QRCODE, errors::Error};

pub async fn run() -> Result<(), Error> {
    let mut bot = bot::Bot::default();

    // let mut bot = bot.lock().await;

    bot.set_uuid_callback(println_qrcode_url);
    bot.set_mode(Mode::Desktop);

    bot.hot_login().await?;

    bot.message_loop().await
}

/// 打印登录二维码
fn println_qrcode_url(uuid: &str) {
    println!("访问下面网址扫描二维码登录");
    let qrcode_url = format!("{}{}", QRCODE, uuid);
    println!("{}", qrcode_url);

    let os_name = std::env::consts::OS;

    let (command, args): (&str, Vec<&str>) = match os_name {
        "macos" => ("open", vec![&qrcode_url]),
        "windows" => ("cmd", vec!["/c", "start", &qrcode_url]),
        "linux" => ("xdg-open", vec![&qrcode_url]),
        _ => {
            panic!("未支持当前操作系统: {}", os_name);
        }
    };

    let res = Command::new(command)
        .args(args)
        .output()
        .expect("打开二维码失败");

    match res.status.success() {
        true => {
            println!("{}", String::from_utf8_lossy(&res.stdout));
        }
        false => panic!("执行命令失败: {}", String::from_utf8_lossy(&res.stderr)),
    }
}
