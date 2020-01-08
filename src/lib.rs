//! **fetchtest2**  

//needed for dodrio! macro (typed-html)
#![recursion_limit = "512"]
//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    //Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    //and key information related to it.
    clippy::cargo_common_metadata,
    //Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    //structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    //Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    //version of your dependency, and wildcard dependencies would cause unnecessary 
    //breakage in the ecosystem.
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    //Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    //Programmers coming from other languages might prefer the expressiveness of return. 
    //It’s possible to miss the last returning statement because the only difference 
    //is a missing ;. Especially in bigger code with multiple return paths having a 
    //return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export run not found 
    //Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    //as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    //cannot be inlined across crates. Certain types of crates might intend for most of the 
    //methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    //For these types of crates, enabling this lint might make sense. It allows the crate to 
    //require all exported methods to be #[inline] by default, and then opt out for specific 
    //methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    //Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    //clippy::integer_arithmetic,
    //Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    //Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,

    clippy::missing_docs_in_private_items,
)]
//endregion

//region: extern and use statements

use console_error_panic_hook;
use serde;
use web_sys;
use unwrap::unwrap;
use conv;
use conv::{ConvUtil};
use dodrio::builder::text;
use wasm_bindgen::prelude::*;
use dodrio::bumpalo::{self};
use dodrio::{Node, Render, RenderContext};
use typed_html::dodrio;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
use web_sys::{console};
//endregion

///simple console write with a string
fn log1(x: &str) {
    console::log_1(&JsValue::from_str(x));
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootRenderingComponent {
    pub respbody: String,
}

#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's `<body>`.
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let div_for_virtual_dom = unwrap!(document.get_element_by_id("div_for_virtual_dom"));

    // Construct a new rendering component.
    let rrc = RootRenderingComponent::new();

    // Mount the component to the `<body>`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    // Run the component forever.
    vdom.forget();
}

//I put this on a button to avoid confusio if the css initialization was already finished.
pub fn startfonts() {
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let window_width =
        unwrap!(unwrap!(JsValue::as_f64(&unwrap!(window.inner_width()))).approx_as::<usize>());

    let body = unwrap!(document.body());
    let body_width = body.client_width();
    let mut ss = "".to_owned();
    ss.push_str(&format!(
        "<p>width body {:?}px window {:?}px </p>",
        body_width, window_width
    ));
    //this is the target cpls I need for css
    let cpls = vec![60, 48, 40, 32, 26, 24, 20, 16, 13];
    let mut new_px: Vec<i32> = vec![];
    //region: find cpl for this device and body size
    //I tried the canvas context 2d metrics width, but the numbers looked wrong.
    //Start with 1 px and go up by 1 px.
    //write it on the screen.
    let mut j = 0;
    let div_for_fonts = unwrap!(document.get_element_by_id("div_for_fonts"));
    for i in 1..100 {
        let target_cpl = cpls[j];
        let s = format!(
            r#"<p><span id="fs{0}" style="font-size: {0}px !important;">abcdefghij</span></p>"#,
            i
        );
        ss.push_str(&s);
        div_for_fonts.set_inner_html(&s);

        let test_text_width = unwrap!(document.get_element_by_id(&format!("fs{}", i)));
        let html_element_test_width = unwrap!(test_text_width.dyn_into::<web_sys::HtmlElement>());
        let text_width = html_element_test_width.offset_width();
        let cpl = (body_width as f64 / (text_width as f64 / 10.0)).floor();
        log1(&format!("text_width {}", text_width));
        ss.push_str(&format!("<p>width {}  {}px  {}cpl</p>", i, text_width, cpl));

        if cpl <= target_cpl as f64 {
            new_px.push(i);
            j += 1;
            if j >= cpls.len() {
                break;
            }
        }
    }
    let mut s = "".to_owned();
    for x in &new_px {
        s.push_str(&format!(
            r#"<p>{0}px<span style="font-size: {0}px !important;">abcdefghij</span></p>"#,
            x
        ));
    }
    ss.push_str(&format!("<p>{:?}{}</p>", new_px, &s));
    div_for_fonts.set_inner_html(&ss);
    //end region
}

impl RootRenderingComponent {
    fn new() -> RootRenderingComponent {
        //return
        RootRenderingComponent {
            respbody: "".to_owned(),
        }
    }
}

// The `Render` implementation. It is called for every Dodrio animation frame to render the vdom.
impl Render for RootRenderingComponent {
    #[allow(clippy::panic)]
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        //create the virtual dom
        let bump = cx.bump;
        dodrio!(bump,
            <div>
            <button style= "margin:auto;display:block;" onclick={ move |root, vdom, _event| {
                startfonts();
                }}>"startfonts"
            </button>
                <pre>
                    {vec![text(
                        bumpalo::format!(in bump, "{}",
                        self.respbody)
                        .into_bump_str()
                    )]}
                </pre>
            </div>
        )
    }
}
