use sycamore::prelude::*;

#[component]
pub fn Counter() -> View {
    let mut counter = create_signal(0);

    let increment = move |_| counter += 1;
    let decrement = move |_| counter -= 1;
    let reset = move |_| counter.set(0);

    view! {
        div(class="counters") {
            button(class="decrement", on:click=decrement) { "-" }
            span(class="value") { (counter.get()) }
            button(class="increment", on:click=increment) { "+" }
            button(class="reset", on:click=reset) { "[]" }
        }
    }
}
