use leptos::{IntoView, IntoAttribute, component, tracing, view};

#[component]
pub fn LocationIcon(
    #[prop(default = 40)] size: u32,
    #[prop(default = "fill-black")] class: &'static str,
) -> impl IntoView {
    view! {
        <svg
            width=size
            height=size
            viewBox="0 0 16 16"
            class=class
            xmlns="http://www.w3.org/2000/svg"
        >
            <path d="m12.596 11.596-3.535 3.536a1.5 1.5 0 0 1-2.122 0l-3.535-3.536a6.5 6.5 0 1 1 9.192-9.193 6.5 6.5 0 0 1 0 9.193Zm-1.06-8.132v-.001a5 5 0 1 0-7.072 7.072L8 14.07l3.536-3.534a5 5 0 0 0 0-7.072ZM8 9a2 2 0 1 1-.001-3.999A2 2 0 0 1 8 9Z"></path>
        </svg>
    }
}