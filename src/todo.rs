use super::Styles;
use dioxus::prelude::*;

use std::{rc::Rc, sync::RwLock};

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {class: "container mx-auto",
            Todos {}
        }
        footer {class: "footer p-10 bg-neutral mt-4",
            "The quick brown fox jumps over the lazy dog."
        }
        Styles {}
    })
}

fn Todos(cx: Scope) -> Element {
    let todos = use_state(&cx, Rc::<RwLock<Vec<String>>>::default);

    cx.render(rsx!(
        h1 {class: "text-3xl my-4", "Plans"}
        form {
            prevent_default: "onsubmit",
            onsubmit: |ev| {
                todos.with_mut(|todos| todos.write().unwrap().push(ev.values["new"].to_string()));
            },
            input {class: "input input-bordered input-primary w-full m-2",
                name: "new",
                placeholder: "What'd you like do next?"
            }
        }
        div {class: "card card-body shadow-xl bg-base-200",
            todos.read().unwrap().iter().enumerate().map(|(idx, todo)| rsx! {
                div {class: "bg-slate-700 hover:bg-slate-800 p-2 border-b border-slate-600 cursor-pointer",
                    onclick: move |_| {
                        todos.with_mut(|todos| { todos.write().unwrap().remove(idx); });
                    },

                    format!("{}. {todo}", idx + 1)
                }
            })
            if todos.read().unwrap().is_empty() {
                rsx!(p {class: "text-3xl p-2",
                    "You're all done!"
                })
            }
        }
    ))
}
