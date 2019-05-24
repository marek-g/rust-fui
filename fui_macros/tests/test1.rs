use fui_macros::ui;

#[test]
fn test1() {
    let x = 4;
    let a = ui! {
        5 + x
    };
    println!("{}", a);
}
