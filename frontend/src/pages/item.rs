use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::router::Route;

#[derive(Debug, Clone, PartialEq, Deserialize, Properties)]
pub struct InstallationItem {
    pub id: i32,
    pub order_number: String,
    pub setup_name: String,
    pub order_status: String,
    pub username: String,
    pub domain: String,
}
#[derive(PartialEq, Properties)]
pub struct ItemComponentProps {
    pub period: InstallationItem,
}

#[function_component(Item)]
pub fn item(prop: &ItemComponentProps) -> Html {
    // let ItemComponentProps { period } = prop;

    let id = prop.period.id;
    let history = use_history().unwrap();
    let ondelete = Callback::once(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let url = format!("/api/delete/{}", id);
            let _req = Request::delete(&url).send().await;
        });

        history.push(Route::AfterDelete)
    });
    // let ItemComponentProps { period } = prop;

    html! {

        <div class="p-2 my-3 max-w-sm bg-white border border-gray-300 shadow-md item ">
            <div class= "flex ">

                if prop.period.order_status == "done" {
                    <span class=" h-3 w-3 mt-1 ml-1 rounded-full bg-green-600 opacity-90"></span>
                }
                else {
                    <span class=" h-3 w-3 mt-1 ml-1 rounded-full bg-orange-500 opacity-90"></span>
                }
                
                <a href="#">
                    <p class="mb-1 px-2 text-l text-blue-500 ">{prop.period.order_number.to_owned()}</p>
                </a>
                <p class="mb-1 text-l text-gray-900">{prop.period.setup_name.to_owned()}</p>
                <p class="mb-1 text-l text-gray-900">{"-----"}{prop.period.order_status.to_owned()}</p>
            </div>
            <div class="flex-grow border-t border-gray-200"></div>

            <div class= "m-1">
                <p class="mb-1 font-normal text-gray-700 ">{prop.period.username.to_owned()}{" started this"}{"__ minutes ago"}</p>
                <p class="mb-1 font-normal text-gray-700 ">{"It is ended in "}{"__ minutes"}</p>
                <p class="mb-1 font-normal text-blue-500 ">{prop.period.domain.to_owned()}</p>
            </div>

            <div class= "flex">
                <button class="bg-orange-500 hover:bg-orange-600 text-white h-7 m-1 text-sm leading-4 border border-gray-300 shadow-md font-bold pt-1 px-3 mb-2 rounded">{"Check Logs"}</button>
                <button onclick={ondelete} class="bg-red-500 hover:bg-red-600 text-white border h-7 m-1 text-sm leading-4 border-gray-300 shadow-md font-bold pt-1 px-3 mb-2 rounded">{"DELETE"}</button>
            </div>
        </div>
    }
}

#[function_component(ViewItems)]
pub fn ins() -> Html {
    let installations = use_state(|| None);
    let error = use_state(|| None);
    {
        let installations = installations.clone();
        let error = error.clone();
        use_effect_with_deps(
            move |_| {
                let installations = installations.clone();
                let error = error.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_installations = Request::get("/api/homeee").send().await;

                    match fetched_installations {
                        Ok(response) => {
                            let json: Result<Vec<InstallationItem>, _> = response.json().await;
                            match json {
                                Ok(f) => {
                                    installations.set(Some(f));
                                }
                                Err(e) => error.set(Some(e.to_string())),
                            }
                        }
                        Err(e) => error.set(Some(e.to_string())),
                    }
                });
                || ()
            },
            (),
        );
    }
    match installations.as_ref() {
        Some(f) => f
            .iter()
            .map(|installation_item| {
                html! {
                    <Item period={installation_item.clone()} />
                }
            })
            .collect(),
        None => {
            return html! {
                <p>{format!("Loading......")}</p>
            }
        }
    }
}
