use dioxus::prelude::*;

use dioxus_icons::lucide::{Repeat2};


#[component]
pub fn Header(active: Signal<bool>) -> Element {

    

    rsx! {
        header { class: "flex items-center  w-full border-b border-solid border-[#2c2f3b] min-h-[70px]",

            div { class: "flex w-full gap-3 items-center",
                span { class: "flex items-center justify-center bg-[#875aee] p-[5px] rounded-[15px] text-[40px] font-bold w-[50px] h-[50px] relative",
                    "e"
                    span { class: "absolute text-[15px] top-[1px] right-[2px]", "✦" }
                }
                h1 { class: "text-[18px] font-bold", "English World" }
            }

            button {
                class: "bg-[#223232] p-2 rounded-[5px] cursor-pointer text-[#24b75c] hover:scale-120 transition-transform duration-300",
                onclick: move |_| { active.set(!active()) },
                Repeat2 {}
            }
        }
    }
}


