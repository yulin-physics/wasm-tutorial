use seed::{prelude::*, *};

struct Model {
    counter: i32,
}

enum Msg {
    Increment,
    Decrement,
    Reset,
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
        Msg::Reset => model.counter = 0,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![div![
        style! {
            St::Display => "flex",
            St::Padding => px(50),
            St::FontSize => px(20),
        },
        div![
            style! {
                St::Padding => px(20),
                St::FontSize => px(20),
            },
            button![
                "RESET",
                style! {
                    St::Padding => px(20),
                    St::FontSize => px(20),
                    St::BackgroundColor => "grey",
                },
                ev(Ev::Click, |_| Msg::Reset)
            ]
        ],
        div![
            style! {
                St::Padding => px(20),
                St::FontSize => px(20),
            },
            button![
                "ADD",
                style! {
                    St::Padding => px(20),
                    St::FontSize => px(20),
                    St::BackgroundColor => "green",
                },
                ev(Ev::Click, |_| Msg::Increment)
            ]
        ],
        div![
            style! {
                St::Padding => px(20),
                St::FontSize => px(20),
            },
            button![
                "SUBTRACT",
                style! {
                    St::Padding => px(20),
                    St::FontSize => px(20),
                    St::BackgroundColor => "red",
                },
                ev(Ev::Click, |_| Msg::Decrement)
            ]
        ],
        div![
            style! {
                St::Padding => px(20),
                St::FontSize => px(20),
            },
            "COUNTER: ",
            model.counter,
        ],
    ]]
}

fn main() {
    // MVC model
    App::start("app", init, update, view);
}
