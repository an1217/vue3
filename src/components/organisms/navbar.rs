use crate::components::atoms::bb_link::{BBLink, LinkType};
use crate::router::Route;
use crate::store::Store;
use gloo::console::log;
use stylist::{css, yew::styled_component};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlLiElement, HtmlElement, Document};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    
    let (state, _dispatch) = use_store::<Store>();
    let username = state.username.clone();
    let token = state.token.clone();
    let onclick = {
        Callback::from(|event:MouseEvent| {
            let name = event.target().unwrap().unchecked_into::<HtmlElement>();
            let a = Document::new().unwrap().get_elements_by_tag_name("div");
            log!(a);
            log!(name);
        })
    };

    html! {
    <>
        <header>
            <nav class="container p-6 mx-auto  justify-between items-center hidden lg:flex ">
                <div class="w-20 py-5 text-cyan-200 font-bold text-3xl">
                    <BBLink text={"PP".to_owned()} data_test={"logo".to_owned()} route={Route::Home} link_type={LinkType::Logo}  text_logo={"LiveliHood".to_string()}/>
                </div>
                <div>
                
                    <ul class="items-center space-x-6 flex">
                        <li>
                            <BBLink text={"注册".to_owned()} data_test={"创建".to_owned()} route={Route::CreateAccount} link_type={LinkType::Link}/>
        
                        </li>
                        <li>
                            <BBLink text={"登录".to_owned()} data_test={"login".to_owned()} route={Route::Login} link_type={LinkType::Button}/>
                        </li>
                        <li></li>
                        <li></li>
                    </ul>
                </div>
            </nav>
            // 手机端  剪辑元素溢出边界的内容 100% 优先级50 背景模糊
            <nav  class="lg:hidden fixed bottom-2 overflow-hidden w-full z-50 ">
                <div class="container mx-auto" >
                    <ul class="w-full bg-black/20 h-[96px] rounded-full backdrop-blur-2xl
                     mx-auto px-5 flex items-center text-white/50 justify-around " {onclick} id = "mousemoveme" >
                     <li name="name">
                     <BBLink text={"注册".to_owned()} data_test={"创建".to_owned()} route={Route::CreateAccount} link_type={LinkType::Mobile} >
                        <img  src="img_girl.jpg" alt="Girl in a jacket" width="55" height="50" />
                     </BBLink>
                     </li>
                     <BBLink text={"登录".to_owned()} data_test={"login".to_owned()} route={Route::Login} link_type={LinkType::Mobile}/>
                    </ul>
                </div>
                
            </nav>
        </header>

        // <section >
            
        //     if state.token.clone().is_empty() {
        //         <div class="nav-right">
        //             <BBLink text={"注册".to_owned()} data_test={"创建".to_owned()} route={Route::CreateAccount} link_type={LinkType::Button}/>
        //             <BBLink text={"登录".to_owned()} data_test={"login".to_owned()} route={Route::Login} link_type={LinkType::Button}/>
        //         </div>
        //     } else {
        //         <div>{format!("welcome, {}", state.username.clone())}</div>
        //     }
        // </section>
        </>
    }
}
