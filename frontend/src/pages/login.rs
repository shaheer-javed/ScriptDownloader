use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
            
            <section class="bg-gray-50 ">
                <div class="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
                    <a href="/login" class="flex items-center mb-6 text-2xl font-semibold text-gray-900 ">
                        <img class="h-8 w-8 mr-2" src="https://github.com/yewstack.png" alt="LOGO" />
                        {"Setup Tool"}
                    </a>
                    <div class="w-full bg-white rounded-lg shadow  md:mt-0 sm:max-w-md xl:p-0 ">
                        <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
                            <h1 class="text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl ">
                                {"Login"}
                            </h1>
                            <form class="space-y-4 md:space-y-6" action="/">
                                <div>
                                    <input type="username" name="username" id="username" placeholder="Enter Username" class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5"  />
                                </div>
                                <div>
                                    <input type="password" name="password" id="password" placeholder="Enter Password" class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 "  />
                                </div>

                                <button type="submit" class="w-full text-white bg-gray-700 hover:bg-gray-500 font-medium rounded-lg text-sm px-5 py-2.5 text-center">{"Sign in"}</button>

                            </form>
                        </div>
                    </div>
                </div>
            </section>
        }
}
