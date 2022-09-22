use crate::pages::item::ViewItems;
use crate::router::Route;

use yew::prelude::*;
use yew_router::prelude::*;



#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let clicklogin = Callback::once(move |_| history.clone().push(Route::Login));
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::SetupPage));

    

    html! {
        <div>
            <nav class="bg-zinc-100">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex items-center justify-between h-16">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <img class="h-8 w-8" src="https://github.com/yewstack.png" alt="LOGO" />
                            </div>
                            <div class="hidden md:block">
                                <div class="ml-10 flex items-baseline space-x-4">
                                    <a href="/" class="bg-gray-900 text-white px-3 py-2 rounded-md text-sm font-medium" aria-current="page">{"Setup Tool v0.1"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
            <main>
                <div class ="flex justify-center ">
                    <div class=" py-6 sm:px-6 lg:px-4 " >
                    // <!-- Replace with your content -->

                        <div class="border-0 border-b border-t border-solid border-gray-900 px-9 py-4 mb-9 overflow-y-auto h-[30rem]"  >
                        // inside border
                            <div class="w-96 " >
                                //cards
                                <ViewItems />

                            </div>

                        </div>
                        //outside border
                        <div class = "flex justify-center space-x-8">
                            <button onclick={clicklogin} class="bg-gray-50 hover:bg-orange-500 text-black hover:text-white border border-gray-300 shadow-md font-bold py-2 px-4 rounded">{"Logout"}</button>
                            <button {onclick} class="bg-orange-500 hover:bg-gray-50 text-white hover:text-black border border-gray-300 shadow-md font-bold py-2 px-4 rounded">{"Start Installation"}</button>
                        </div>
                    // <!-- /End replace -->
                    </div>
                </div>
            </main>
        </div>

    }
}
