mod router;
mod pages;

use router::{Route, switch};
use yew::prelude::*;
use yew_router::prelude::*;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}






















// use std::{
//     fs::File,
//     io::{prelude::*, BufReader},
// };


// pub enum Msg {
//     AddOne,
//     SubOne,
// }

// pub struct CounterComponent {
//     count: i64,
// }

// impl Component for CounterComponent {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self { count: 0 }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::AddOne => {
//                 self.count += 1;
//                 true //re-render component
//             }
//             Msg::SubOne => {
//                 self.count -= 1;
//                 true //re-render component
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let link = ctx.link();
//         // html! {
//         //     <div class="container">
//         //         <button onclick = {link.callback(|_| Msg::AddOne)}>{"+1"}</button>
//         //         <p>{self.count}</p>
//         //         <button onclick = {link.callback(|_| Msg::SubOne)}>{"-1"}</button>
//         //     </div>
//         // }

//         // let file = File::open("setups.txt").expect("no such file");
//         // let buf = BufReader::new(file);
//         // let op: Vec<String> = buf
//         //     .lines()
//         //     .map(|l| l.expect("Could not parse line"))
//         //     .collect();
//         let op : Vec<String> = ["setup1".to_string(),"setup2".to_string()].to_vec();
//         html! {
//             

//         }
//     }
// }