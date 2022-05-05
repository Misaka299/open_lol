use std::env;
use std::env::{args, current_dir, current_exe};
use std::ffi::OsStr;
use std::fs::remove_file;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::process::exit;
use std::ptr::null_mut;

use ini::{Ini};
use itertools::Itertools;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::UINT;
use winapi::um::winuser::{IDCANCEL, MB_OK, MB_OKCANCEL};

use crate::open_file_dialog::FileFilter;
use crate::open_file_dialog::open_file_dialog;

mod open_file_dialog;

slint::slint! {
    import { CheckBox,Button } from "std-widgets.slint";

    MainWindow := Window {
        title: "LOL打开器";
        width: 500px;
        height: 300px;

        callback click();

        Image {
            source: @image-url("src/bg.jpg");
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
                CheckBox {
                    text:"删除WGTinyDL文件";
                    checked: true;
                }
                CheckBox {
                    text:"设置用户组禁止写入";
                    checked: false;
                }
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

fn main() {
    let main = MainWindow::new();
    let lol_path = read_lol_path();
    main.on_click(move || {
        // lol_path;

        check_ini(&lol_path);
    });
    main.run();
}

const LOL_EXE: &str = "Client.exe";
const START_TIP: &str = r#"本程序有两种打开方式:
最简单的办法就是把LOL的快捷方式或者是启动程序“Client.exe”拖放到本程序的图标上，本程序会把自身拷贝到C盘根目录下，并创建桌面快捷方式。需要删除时只需要删除桌面快捷方式和C盘根目录下的文件就可以完全删除;
如果你不喜欢文件在你的C盘根目录下创建文件，你才需要在你认为合适的目录下直接打开本程序，进行手动指定LOL启动程序文件路径该方式会在本程序文件同目录下创建"\#open_lol.ini"配置文件;
按下确定键，程序讲继续运行，你需要手动选择LOL启动程序文件路径；
按下取消键，程序将退出，你可以将LOL的快捷方式或者是启动程序“Client.exe”拖放到本程序的图标上以重新打开本程序；
\n\n
祝你游戏愉快！"#;

fn read_lol_path() -> String {
    if args().count() > 0 {
        let path = env::args().find_or_first(|arg| {
            arg.contains(LOL_EXE)
        }).unwrap_or_else(|| {
            message_box("未识别的文件或命令行", MB_OK);
            exit(0);
        });
        check_current_exe_path();
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
    }
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

fn message_box(msg: &str, btn_type: UINT) -> c_int {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    return unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), btn_type)
    };
}

/// 检查当前exe目录是否正确
fn check_current_exe_path() {
    println!("{:?}",current_exe().unwrap());
    println!("{:?}",PathBuf::from("c:\\".to_owned() + current_exe().unwrap().file_name().unwrap().to_str().unwrap()));
    if current_exe().unwrap() == PathBuf::from("c:\\".to_owned() + current_exe().unwrap().file_name().unwrap().to_str().unwrap()) {
        message_box("exe==", MB_OK);
    } else {
        message_box("exe!=", MB_OK);
    }
}

fn check_ini(path: &String) {
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
    conf.with_section(Some("")).set("data_name", "lol");
    conf.write_to_file(ini_path);
}