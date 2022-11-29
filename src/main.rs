use log::info;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ListProps {
    fields: [u8; 10],
}

#[function_component(DummyList)]
fn dummy_list(ListProps { fields }: &ListProps) -> Html {
    let fields_html: Html = fields
        .into_iter()
        .enumerate()
        .map(|(idx, field)| {
            html! {
                <div key={*field} style={format!("position: absolute; left: {}rem; top: {}rem; background: lightblue; transition: all 1s;", idx, idx)}>
                    { field }
                </div>
            }
        })
        .collect();
    html! {
        <div class="wrapper" style="position: relative;">
            { fields_html }
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let fields = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let fields = use_state(|| fields);

    let swap_neighbors = {
        let fields = fields.clone();
        Callback::from(move |_| {
            let mut updated_fields = *fields;
            updated_fields.swap(2, 3);
            fields.set(updated_fields);
        })
    };

    let swap_far = {
        let fields = fields.clone();
        Callback::from(move |_| {
            let mut updated_fields = *fields;
            updated_fields.swap(2, 5);
            fields.set(updated_fields);
        })
    };

    html! {
        <>
            <button onclick={swap_neighbors.clone()}>{ "Swap neighbors" }</button>
            <button onclick={swap_far.clone()}>{ "Swap further apart" }</button>
            <DummyList fields={*fields} />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}
