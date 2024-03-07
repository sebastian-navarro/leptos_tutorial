use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button 
            on:click = move |_| {
                set_count.update(|n| *n +=2);
            }>
            "Click me: "
            {move || count}
        </button>
        <p>
            <strong>"Reactive: "</strong>
            // you can insert Rust expressions as values in the DOM
            // by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            // signals are functions, so we can remove the wrapping closure
            {count}
        </p>
        
    }
}
fn main() {
    mount_to_body(|| view! { <App /> })
}


