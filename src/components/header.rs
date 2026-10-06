use dioxus::prelude::*;



#[component]
pub fn Header() -> Element {


    rsx! {
        header { class: "flex flex-col  w-full border-b border-solid border-[#2c2f3b] min-h-[70px]",

            div { class: "flex w-full gap-3 items-center",
                span { class: "flex items-center justify-center bg-[#875aee] p-[5px] rounded-[15px] text-[40px] font-bold w-[50px] h-[50px] relative",
                    "e"
                    span { class: "absolute text-[15px] top-[1px] right-[2px]", "✦" }
                }
                h1 { class: "text-[18px] font-bold", "English World" }
            }
        }
    }
}


