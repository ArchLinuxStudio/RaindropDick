use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, NamedView, ResizedView, ScrollView, SelectView};
use cursive::view::{Nameable,Scrollable,Resizable};
use cursive::Cursive;
use spider::get_the_key;
use futures::executor::block_on;

use crate::abutton::mybutton::MyButton;
//#[derive(Clone)]
//pub struct MyButton {
//    pub name : String,
//}
//impl MyButton{
//    #[allow(dead_code)]
//    pub fn new () -> MyButton{
//        MyButton{
//            name: String::from("ssss"),
//        }
//    }
//    pub fn output(&self) -> Dialog {
//        Dialog::around(TextView::new("Hello Dialog!"))
//                .title("Cursive")
//                .button(self.name.clone(), |s| s.quit())
//    }
//}
//impl Drop for MyButton {
//    fn drop(&mut self) {
//        println!("sss");
//    }
//}
fn url_select() -> ResizedView<ScrollView<NamedView<SelectView<MyButton>>>> {
    let select = SelectView::<MyButton>::new()
        .on_submit(on_submit)
        .with_name("select")
        .scrollable()
        .scroll_y(true)
        .fixed_size((60, 25));
    return select;

}
pub fn url_buttons() -> Dialog {
    let select = url_select();
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", quit));
    let dialog = Dialog::around(LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons))
        .title("Fuck you GFW");
    return dialog;

}
fn delete_name(s: &mut Cursive) {
    let select = s.find_name::<SelectView<MyButton>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(_) => {
            s.add_layer(Dialog::around(LinearLayout::horizontal()
                    .child(Button::new("Sure", |s|{
                        let mut select = s.find_name::<SelectView<MyButton>>("select").unwrap();
                        match select.selected_id() {
                            None => {
                                s.add_layer(Dialog::info("No name to remove"));
                                s.pop_layer();
                                s.pop_layer();
                            },
                            Some(focus) =>{
                                select.remove_item(focus);
                                s.pop_layer();
                            }
                        }
                    }))
                    .child(Button::new("cancle", |s|{
                        s.pop_layer();
                    })))
                .title("Sure?"));
        }
    }
}

fn on_submit(s: &mut Cursive, name: &MyButton) {
    s.add_layer(name.output());
}
fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<MyButton>| {
            let mut temp : Vec<String> = vec![];
            temp.push(name.to_string());
            let future = get_the_key(temp);
            let output :Vec<Vec<String>> = block_on(future).unwrap();
            for urls in output.into_iter(){
                for url in urls.into_iter(){
                    let temp2 = MyButton::new(url);
                    let names = temp2.clone().ps;
                    view.add_item(names, temp2);
                }
            }
        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_name("name")
            .fixed_width(10))
        .title("Enter a new name")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}
fn quit(s: &mut Cursive){
    Cursive::quit(s);
}
