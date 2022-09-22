use crate::router::Route;
use wasm_bindgen::JsCast;
// use gloo::console::log;
use crate::pages::store::YewduxStore;
use reqwasm::http::Request;
// use serde::{Deserialize, Serialize};
use serde_json::json;
use web_sys::*;
use yew::prelude::*;
use yew_router::prelude::{History, *};
use yewdux::prelude::*;
use yewdux_functional::use_store;

#[function_component(SetupPage)]
pub fn setup() -> Html {
    let history = use_history().unwrap();
    // let history = history.clone();
    let onclick = Callback::once(move |_| history.push(Route::Home));
    let history = use_history().unwrap();

    let store = use_store::<PersistentStore<YewduxStore>>();
    let handle_form_submit = {
        store
            .dispatch()
            .reduce_callback_with(move |state, event: MouseEvent| {
                event.prevent_default();
                let option = state.option.clone();
                let textarea = state.textarea.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    // let _res = api_login(option, textarea).await;
                    let body = json!({
                      "option": option,
                      "textarea": textarea
                    });
                    let _response = Request::post("/api/ffff")
                        .header("content-type", "application/json")
                        .body(body.to_string())
                        .send()
                        .await
                        .unwrap();
                });
                history.push(Route::AfterRun)
            })
    };
    let handle_option = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let option = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.option = option;
        });

    let handle_textarea = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let textarea = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.textarea = textarea;
        });

    html! {

        <div class ="flex justify-center ">

            <div class=" py-6 sm:px-6 lg:px-4 grid grid-cols-1" >
            // <!-- Replace with your content -->

                <h1 class="text-2xl text-orange">{"START AN INSTALLATION"}</h1>
                <form class="space-y-4 md:space-y-6" >
                    <label for="cars" class="text-xl orange my-3">{"SELECT A SERVICE:"}</label>
                    <br/>
                    <select onchange={handle_option} name="option" id="option" class="w-[500px] h-8 border border-gray-300 shadow-md pr-32 ">
                        <option value="setup1">{"Setup1"}</option>
                        <option value="setup2">{"Setup2"}</option>
                        <option value="setup3">{"Setup3"}</option>
                        <option value="setup4">{"Setup4"}</option>
                    </select>

                    <h1 class= "my-3">{"Enter setup data here:"}</h1>
                    <textarea onchange={handle_textarea} id="textarea" name="textarea" rows="20" cols="70" class = "border border-gray-300 shadow-md pt-2 p-1 text-sm">{""}</textarea>

                    <div class = "flex justify-center space-x-8 mt-5">
                        <button {onclick} class="bg-gray-50 hover:bg-orange-500 text-black hover:text-white border border-gray-300 shadow-md font-bold py-2 px-4 rounded">{"Cancel"}</button>
                        <button type="submit" onclick={handle_form_submit}  class="bg-orange-500 hover:bg-gray-50 text-white hover:text-black border border-gray-300 shadow-md font-bold py-2 px-4 rounded">{"Start now"}</button>
                        // <input type="submit" value="Start now" class="bg-orange-500 hover:bg-gray-50 text-white hover:text-black border border-gray-300 shadow-md font-bold py-2 px-4 rounded" />

                    </div>
                </form>
            // <!-- /End replace -->
            </div>
        </div>

    }
}

// pub async fn api_login(option: String, textarea: String) {
//     let body = json!({
//       "option": option,
//       "textarea": textarea
//     });
//     let _response = Request::post("/api/ffff")
//         .header("content-type", "application/json")
//         .body(body.to_string())
//         .send()
//         .await
//         .unwrap();

// }
