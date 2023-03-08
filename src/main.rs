use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        view! {
            cx,
            button { "Hello, world!" }
        }
    });
}
