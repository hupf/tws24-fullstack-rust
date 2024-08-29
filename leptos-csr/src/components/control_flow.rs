use leptos::*;

#[component]
pub fn ControlFlow(count: MaybeSignal<i32>) -> impl IntoView {
  view! {
    <section>
      if/else:
      {move ||
        if count() > 5 {
          view! {<p>Yeah!</p>}
        } else {
          view! {<p>Nope!</p>}
        }
      }
    </section>

    <section>
      if/optional else:
      {move ||
        if count() > 5 {
          Some( view! {<p>Yeah!</p>})
        } else {
          None
        }
      }
    </section>

    <section>
      .then:
      {move ||
        (count() > 5).then(|| view! { <p>Yeah!</p> })
      }
    </section>

    <section>
      .match:
      {move ||
        match count() {
          5 => view! { <p>Yeah!</p> },
          n if n % 2 != 0 => view! { <p>Huh?</p> },
          _ => view! { <p>Nope!</p> }
        }
      }
    </section>
  }
}