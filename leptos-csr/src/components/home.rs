use leptos::*;
use leptos_router::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <ul>
        <li><A href="/">"Home"</A></li>
        <li><A href="/iteration">"Iteration"</A></li>
      </ul>
    }
}