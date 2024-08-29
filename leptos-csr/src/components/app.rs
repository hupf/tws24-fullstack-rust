use leptos::*;
// use leptos_router::*;
use stylers::style;

// use super::home::*;
use super::hello::*;
use super::iteration::*;
use super::control_flow::*;
use super::children::*;

#[component]
pub fn App() -> impl IntoView {
    let styler_class = style! { "App",
      button {
        background-color: blue;
        color: white;
      }
      .red {
        background-color: red;
      } 
    };

    let (count, set_count) = create_signal(0);

    view! {class = styler_class,
      // <Router>
      //   <nav>
      //     <ul>
      //       <li><A href="/">"Home"</A></li>
      //       <li><A href="/iteration">"Iteration"</A></li>
      //     </ul>
      //   </nav>
      //   <main>
      //     <Routes>
      //     <Route path="/" view=|| view! {} />
      //     // <Route path="/" view=Home />
      //     <Route path="/iteration" view=Iteration />
      //     </Routes>
      //   </main>
      // </Router>

      <button
          on:click=move |_| {
              set_count.update(|x| *x += 1);
          }
          class:red=move || count.get() % 2 == 1
      >
          "Click me: "
          {move || count.get()}
      </button>
      <Hello name="John".to_owned() />
      <Iteration />
      <ControlFlow count=count.into() />
      <TakesChildren render_prop=|| view! { <p>Prop content</p> }>
        <p>Children content</p>
      </TakesChildren>
    }
}

