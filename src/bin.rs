use ut_dialog::ut::Dialog;

fn main() {
    let dialog = Dialog::new(
        "* This cheese has been here so  long, a magical crystal has  grown around it."
    );

    // dialog.next();

    dialog.image.save("test_dialog.png")
        .expect("couldn't save image");

    // assert_eq!('*', dialog.current_char);
}
