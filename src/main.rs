// callback clicked;
// property <bool> open_curtain;
// property <bool> solved;
// property <image> icon;

// height: 64px;
// width: 64px;
// background: solved ? #34CE57 : #3960D5;
// animate background { duration: 800ms; }

// Image {
//     source: icon;
//     width: parent.width;
//     height: parent.height;
// }

// // Left curtain
// Rectangle {
//     background: #193076;
//     width: open_curtain ? 0px : (parent.width / 2);
//     height: parent.height;
//     animate width { duration: 250ms; easing: ease-in; }
// }

// // Right curtain
// Rectangle {
//     background: #193076;
//     x: open_curtain ? parent.width : (parent.width / 2);
//     width: open_curtain ? 0px : (parent.width / 2);
//     height: parent.height;
//     animate width { duration: 250ms; easing: ease-in; }
//     animate x { duration: 250ms; easing: ease-in; }
// }

// TouchArea {
//     clicked => {
//         // Delegate to the user of this element
//         root.clicked();
//     }
// }

slint::slint! {
    MainWindow := Window {
        width: 500px;
        height: 300px;

        Image {
            source: @image-url("res/bg.png");
            width: parent.width;
            height: parent.height;
        }

        GridLayout {
            y: 30px;
            x: parent.width / 2;
            width: parent.width / 2;
            height: parent.height;
            
            GridLayout{
                width: parent.width;
                height: parent.height / 2;
                
                Row {
                    Text { text: "1. 本软件为收费软件，费用为 0.00 美元。如果您以高于 0.00 美元的价格购买到本软件，请及时告知作者，让本作者嘲笑一下。"; }
                }
                Row {
                    Text { text: "2. 本软件会使用一定的技术手段以避免被封号/误封号。但您的账号不论出于何种原因被封禁，本软件不承担任何责任及赔偿。"; }
                }
            }
        }
    }
}
fn main() {
    let x = include_bytes!("C:\\Windows\\Fonts\\simsun.ttc");
    slint::register_font_from_memory(x).unwrap();
    MainWindow::new().run();
}
