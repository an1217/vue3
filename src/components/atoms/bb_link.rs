use crate::router::Route;
use stylist::{yew::styled_component};
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::{prelude::*, html::onclick::Event};
use yew_router::prelude::*;
use gloo::console::log;

#[derive(Clone, PartialEq)]
pub enum LinkType {
    Link,
    Button,
    Logo,
    Mobile
}
impl Default for LinkType {
    fn default() -> Self {
        Self::Link
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub data_test: String,
    pub route: Route,
    pub link_type: Option<LinkType>,
    pub text_logo: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(BBLink)]
pub fn bb_link(propes: &Props) -> Html {
    //
    let link_type = propes.link_type.clone().unwrap_or_default();
    let stylesheet = choose_stylesheet(link_type);
    let router = propes.route.clone();
    
    
    html! {
        <span  data-test={propes.data_test.clone()}>
            <Link<Route> to={propes.route.clone()}>
                {propes.children.clone()}   
                if propes.children.clone().is_empty() {
                    <span class={stylesheet}>
                    {propes.text.clone()}
                </span>{propes.text_logo.clone()}
                } else {
                    <div class={stylesheet}>
                        {propes.text.clone()}
                    </div>
                }
                
                
            </Link<Route>>  
        </span>
    }
}

fn choose_stylesheet(link_type: LinkType) -> &'static str {
    // 鼠标放入改变颜色  缓慢出现 持续时间2秒
    let link_stylesheet = "hover:text-cyan-200 ease-in  duration-200";
    // 颜色 外边距x y 盒子 圆角 粗体 不透明度0就没了  
    let button_stylesheet = "text-cyan-200  px-9 py-3 border-4 rounded-md font-bold  hover:opacity-50 ease-in duration-200 bg-black";
    let logo_class = "text-white";
    // 中心对齐 沿容器主轴中心对齐项目
    let mobile_class = "text-center w-full";
    match link_type {
        LinkType::Button => button_stylesheet,
        LinkType::Link => link_stylesheet,
        LinkType::Logo => logo_class,
        LinkType::Mobile => mobile_class
    }
}
