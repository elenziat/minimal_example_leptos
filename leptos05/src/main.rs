use gloo_timers::future::TimeoutFuture;
use leptos::html::Div;
use leptos::*;

#[component]
fn Child(forwarded_data: ReadSignal<Option<()>>) -> impl IntoView {
    let div_ref = create_node_ref::<Div>();

    log::debug!("     running Child 'constructor'");

    create_effect(move |_| {
        log::debug!("          running Child effect");
        let Some(el) = div_ref.get() else {
            log::debug!("          div is still None");
            return;
        };

        let data_available = forwarded_data.get().is_some();

        if data_available {
            log::debug!("          child received new data");
        }

        if !el.is_mounted() {
            log::debug!("          div is Some, but unmounted");
            return;
        }

        log::debug!("          div is Some and mounted");

        if data_available {
            el.set_text_content(Some("and here is the data"));
        }
    });

    view! {
        <div node_ref=div_ref><p>here i am</p></div>
    }
}

#[component]
fn App() -> impl IntoView {
    let query_data = create_local_resource(
        || (),
        |_| async move {
            log::debug!("loading data ...");
            TimeoutFuture::new(2000).await;
            log::debug!("data loaded");

            Some(())
        },
    );
    let (forward_data, set_forward_data) = create_signal::<Option<()>>(None);

    view! {
        <Suspense fallback=move || view! {  <p>"wait for me ..."</p> }>
            // Query of resource and Child are siblings
            // {move || query_data.get().map(|_| set_forward_data.set(Some(())))}
            // <Child forwarded_data=forward_data/>

            // Query of resource and Child are part of one single child of Suspense/Transition
            {move || query_data.get().map(|_| {set_forward_data.set(Some(())); view!{ <Child forwarded_data=forward_data/>}})}
        </Suspense>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("starting app");

    mount_to_body(|| view! {  <App/> })
}
