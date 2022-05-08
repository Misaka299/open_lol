#![windows_subsystem = "windows"]

use std::env::{args, current_dir, current_exe};
use std::fs::{File, remove_file};
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, exit};

use encoding::{EncoderTrap, Encoding};
use encoding::all::GBK;
use ini::Ini;
use winapi::um::winuser::{IDCANCEL, MB_OK, MB_OKCANCEL};

use crate::win::*;

mod win;

slint::slint! {
    import { CheckBox,Button } from "std-widgets.slint";

    MainWindow := Window {
        title: "LOL打开器";
        width: 500px;
        height: 300px;

        property <bool> delete_dl_file;
        //property <bool> disable_user_write;

        callback click();

        Image {
            source: @image-url("src/bg.png");
            width: parent.width;
            height: parent.height;
        }

        GridLayout {
            x: parent.width / 2 - 40px;
            width: parent.width / 2 + 40px;
            height: parent.height;
            
            VerticalLayout {
                padding: 20px;

                Text { text: "1. 本软件为收费软件，费用为 0.00 美元。如果你以高于 0.00 美元的价格购得本软件，请及时告知作者，让本作者嘲笑一下。"; wrap: word-wrap; }
                Text { text: "2. 本软件会使用一定的技术手段以避免被封号/误封号。但你的账号不论出于何种原因被封禁，本软件不承担任何责任及赔偿。"; wrap: word-wrap; }
                Text { text: "3. 本软件使用共识式条款确认。即：你打开软件使用本软件视为同意本协议/条款。"; wrap: word-wrap; }
                deldl := CheckBox {
                    text:"删除WGTinyDL文件";
                    checked: true;
                    toggled => {
                        delete_dl_file = self.checked;
                    }
                }
                // CheckBox {
                //     text:"设置用户组禁止写入";
                //     checked: false;
                //     toggled => {
                //         disable_user_write = self.checked;
                //     }
                // }
                open_btn := Button {
                    text:"打开LOL";
                    clicked => {
                        root.click();
                    }
                }
            }

        }
    }
}

const LOL_EXE: &str = "Client.exe";
const START_TIP: &str = r#"本程序有两种打开方式:
最简单的办法就是把LOL的快捷方式或者是启动程序“Client.exe”拖放到本程序的图标上，本程序会把自身拷贝到C盘根目录下，并创建桌面快捷方式。需要删除时只需要删除桌面快捷方式和C盘根目录下的文件就可以完全删除;
如果你不喜欢文件在你的C盘根目录下创建文件，你才需要在你认为合适的目录下直接打开本程序，进行手动指定LOL启动程序文件路径该方式会在本程序文件同目录下创建"\#open_lol.ini"配置文件;
按下确定键，程序讲继续运行，你需要手动选择LOL启动程序文件路径；
按下取消键，程序将退出，你可以将LOL的快捷方式或者是启动程序“Client.exe”拖放到本程序的图标上以重新打开本程序；
\n\n
祝你游戏愉快！"#;

fn main() {
    let main = MainWindow::new();
    let lol_path = read_lol_path();

    let delete_dl_file = main.get_delete_dl_file();
    // let disable_user_write = main.get_disable_user_write();

    main.on_click(move || {
        let dir_path = &lol_path[0..lol_path.len() - LOL_EXE.len()];
        if delete_dl_file {
            remove_file(dir_path.to_owned() + &"WGTinyDL.dll");
            remove_file(dir_path.to_owned() + &"WGTinyDL.exe");
        }
        check_ini(dir_path);
        check_tmp(dir_path);

        if let Err(e) = Command::new("cmd").creation_flags(0x08000000).arg("/c").arg(&lol_path).spawn() {
            message_box(format!("检测到错误，下面是程序捕获的异常信息(仅供参考，未必有用)： {}", e.to_string()).as_str(), MB_OK);
        }
        exit(0);
    });
    main.run();
}

fn read_lol_path() -> String {
    // return r"C:\Program Files\腾讯游戏\英雄联盟\TCLS\Client.exe".to_string();
    if args().count() > 0 {
        println!("{:?}", args());
        let path = args().find(|arg| {
            arg.contains(LOL_EXE)
        }).unwrap_or_else(|| {
            message_box("未识别的文件或命令行", MB_OK);
            exit(0);
        });
        check_current_exe_path(&path);
        message_box(&path, MB_OK);
        return path;
    }
    let conf_path = current_dir().unwrap().join("open_lol.ini");
    return match conf_path.exists() {
        true => {
            let mut conf = Ini::load_from_file(&conf_path).unwrap();
            if let Some(prop) = conf.section(Some("lol")) {
                if let Some(path) = prop.get("path") {
                    return path.to_string();
                }
            }
            // 没有读入就是无效数据段
            remove_file(&conf_path);
            check_path_ini(&conf_path).unwrap()
        }
        false => {
            check_path_ini(&conf_path).unwrap()
        }
    };
}

fn check_path_ini(conf_path: &PathBuf) -> Option<String> {
    let message_code = message_box(START_TIP, MB_OKCANCEL);
    if message_code == IDCANCEL {
        exit(0);
    }
    let path = match open_file_dialog("请选择LOL启动程序", vec![FileFilter::new("请选择LOL启动程序(Client.exe)", vec![LOL_EXE])]) {
        Some(path) => { path.to_str()?.trim_end_matches(LOL_EXE).to_string() }
        None => {
            message_box("玩不起就别玩！！！", MB_OK);
            exit(0);
        }
    };
    let mut conf = if conf_path.exists() { Ini::load_from_file(conf_path).unwrap() } else { Ini::new() };
    conf.with_section(Some("lol")).set("path", &path);
    conf.write_to_file(conf_path);
    return Some(path);
}

/// 检查当前exe目录是否正确
fn check_current_exe_path(path: &String) {
    let current_exe_path = current_exe().unwrap();
    let target_exe_path = PathBuf::from("c:\\".to_owned() + current_exe().unwrap().file_name().unwrap().to_str().unwrap());
    println!("{:?}", current_exe_path);
    println!("{:?}", target_exe_path);
    if current_exe().unwrap() != target_exe_path {
        println!("执行cmd");
        let current_exe_path = current_exe_path.to_str().unwrap();
        let target_exe_path = target_exe_path.to_str().unwrap();

        let copy = "copy ".to_owned() + &current_exe_path + &" " + &target_exe_path;
        let start = r"start %USERPROFILE%\Desktop\open_lol.lnk".to_owned();
        let link = format!(
            r#"mshta VBScript:Execute("Set a=CreateObject(""WScript.Shell""):Set b=a.CreateShortcut(a.SpecialFolders(""Desktop"") & ""\\open_lol.lnk""):b.TargetPath=""{}"":b.Arguments=Chr(34) & ""{}"" & Chr(34):b.WorkingDirectory=""%~dp0"":b.Save:close")"#
            , &target_exe_path, path);
        let delete_exe = "rm ".to_owned() + current_exe_path;
        let delete_bat = "rm c:\\open_lol.bat";
        let cmd = format!("{} && {} && {} && sleep 2 && {} && {}\npause", copy, link,start, delete_exe, delete_bat );
        println!("{}", cmd);
        let mut file = File::create("c:\\open_lol.bat").unwrap();
        let bytes = GBK.encode(cmd.as_str(), EncoderTrap::Strict).unwrap();
        file.write(bytes.as_ref());
        file.flush();
        Command::new("cmd")
            // .creation_flags(0x08000000)
            .creation_flags(0x00000010)
            .arg("/c")
            .arg("c:\\open_lol.bat")
            .spawn()
            .is_ok();
        exit(0);
    }
}

/// 检查ini文件
fn check_ini(path: &str) {
    let ini_path = path.to_owned() + "wegame_launch.ini";
    println!("{:?}", &ini_path);
    let mut conf = if PathBuf::from(&ini_path).exists() { Ini::load_from_file(&ini_path).unwrap() } else { Ini::new() };
    if let Some(prop) = conf.section(Some("TCLS")) {
        if let Some(data_name) = prop.get("data_name") {
            if data_name == "lol" {
                return;
            }
        }
    }
    conf.with_section(Some("TCLS")).set("data_name", "lol");
    conf.write_to_file(ini_path);
}

/// 检查tmp文件
fn check_tmp(path: &str) -> std::io::Result<()> {
    let tmp_path = path.to_owned() + "wegame_launch.tmp";
    println!("{:?}", &tmp_path);

    let mut conf;
    if PathBuf::from(&tmp_path).exists() {
        conf = Ini::load_from_file(&tmp_path).unwrap();
        if let Some(prop) = conf.section(Some("TCLS")) {
            if let Some(data_name) = prop.get("LastLoginMethod") {
                if data_name == "1" {
                    en_readonly(&tmp_path);
                    return Ok(());
                }
            }
        }
        un_readonly(&tmp_path);
        conf = Ini::load_from_file(&tmp_path).unwrap();
    } else {
        conf = Ini::new();
    }
    conf.with_section(Some("TCLS")).set("LastLoginMethod", "1");
    conf.write_to_file(&tmp_path);

    en_readonly(&tmp_path);
    Ok(())
}

fn un_readonly(path: &String) {
    let mut file = File::open(&path).unwrap();
    let mut permissions = file.metadata().unwrap().permissions();
    if permissions.readonly() {
        permissions.set_readonly(false);
        file.set_permissions(permissions);
        file.flush();
    }
}

fn en_readonly(path: &String) {
    let mut file = File::open(&path).unwrap();
    let mut permissions = file.metadata().unwrap().permissions();
    if !permissions.readonly() {
        permissions.set_readonly(true);
        file.set_permissions(permissions);
        file.flush();
    }
}