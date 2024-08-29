use leptos::*;

#[component]
pub fn Hello(name: String) -> impl IntoView {
    view! { <p>Hello: {name}</p> }
}