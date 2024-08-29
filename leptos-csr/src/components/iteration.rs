use leptos::*;

#[component]
pub fn Iteration() -> impl IntoView {
    let (values, set_values) = create_signal(vec![0, 1, 2]);
    view! {
      <button
          on:click=move |_| {
              set_values.update(|v|
                if let Some(&last) = v.last() {
                  v.push(last + 1);
                } else {
                  v.push(0);
                }
              )
          }
      >
        Add
      </button>
      <ul>
        {move ||
          values().into_iter()
            .map(|value| view! { <li>{value}</li> })
            .collect_view()
        }
      </ul>
      <ul>
        <For
          each=values
          key=|v| *v
          children=|value| view! { <li>{value}</li> }
        />
      </ul>
    }
}