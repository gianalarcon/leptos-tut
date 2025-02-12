use leptos::*;
fn main() {
    //console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <button on:click=move |_| {
            set_count.update(|n| *n += 1);
        }
        class:red=move || count() % 2 == 1
        >
        "Click me: "
        {count}
        </button>

        <br/>
        <ProgressBar max=50 progress=count/>
        <ProgressBar progress=count/>
        <ProgressBar max=50 progress=Signal::derive(double_count)/>
    }
}

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}
