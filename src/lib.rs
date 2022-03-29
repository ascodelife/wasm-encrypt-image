use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use js_sys::Promise;
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{HtmlElement, ImageData, Request, Response};

/// 加密密钥
static KEY: u8 = 123;
/// 字体地址
static FONT_URL: &str = "font/arial.ttf";

static IMAGE_ATTRIBUTES: ImageAttributes = ImageAttributes {
    width: 100,
    height: 100,
};

struct ImageAttributes {
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct FontStyle {
    size: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct RenderString {
    cipher: String,
    position: Position,
    font_style: FontStyle,
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    render_info: Vec<RenderString>,
    user_token: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn encrypt_image(params: JsValue) -> Promise {
    let main = async move {
        let font = load_font().await.unwrap();

        let Params {
            mut render_info,
            user_token,
        } = params.into_serde().unwrap();
        // 模拟解密过程
        decrypt_info(&mut render_info);
        // console_log!("render_info is {:?}", render_info);
        // 展示非付费用户能力
        let mut user = User::<FreePlan>::new(user_token, 0, render_info);
        user.render_as_img(&font)?;
        // 展示付费用户能力
        user.fetch_vip_level();
        if user.get_vip_level() > 0 {
            let vip_user: User<VipPlan> = user.into();
            vip_user.render_as_div()?;
        }
        Ok(JsValue::UNDEFINED)
    };

    future_to_promise(main)
}

struct User<T> {
    user_token: String,
    vip_level: usize,
    info: Vec<RenderString>,
    _type: PhantomData<T>,
}

/// 免费用户只能将信息渲染为 img
trait Free {
    fn fetch_vip_level(&mut self);
    fn render_as_img(&self, font: &Font) -> Result<(), JsValue>;
}

/// 给付费用户额外提供将信息渲染为 dom 的能力
trait Vip: Free {
    fn render_as_div(&self) -> Result<(), JsValue>;
}

struct FreePlan;
struct VipPlan;

impl<T> User<T> {
    fn new(user_token: String, vip_level: usize, info: Vec<RenderString>) -> Self {
        Self {
            user_token,
            vip_level,
            info,
            _type: PhantomData::default(),
        }
    }
    fn get_vip_level(&self) -> usize {
        self.vip_level
    }
    fn set_vip_level(&mut self, vip_level: usize) {
        self.vip_level = vip_level
    }
}

impl<T> Free for User<T> {
    fn fetch_vip_level(&mut self) {
        // 省略若干通过 user_token 获取用户信息的异步请求
        let vip_level = 3;
        self.set_vip_level(vip_level);
    }

    fn render_as_img(&self, font: &Font) -> Result<(), JsValue> {
        // init a canvas to get blank image and draw new image on it
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        canvas.set_width(IMAGE_ATTRIBUTES.width);
        canvas.set_height(IMAGE_ATTRIBUTES.height);
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        let blank_image = context.create_image_data_with_sw_and_sh(
            IMAGE_ATTRIBUTES.width.into(),
            IMAGE_ATTRIBUTES.height.into(),
        )?;
        let mut image: RgbaImage = ImageBuffer::from_vec(
            IMAGE_ATTRIBUTES.width,
            IMAGE_ATTRIBUTES.height,
            blank_image.data().to_vec(),
        )
        .unwrap();
        for render_string in &self.info {
            draw_text_in_image(&mut image, font, render_string);
        }
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&image.into_vec()),
            IMAGE_ATTRIBUTES.width,
            IMAGE_ATTRIBUTES.height,
        )?;
        context.put_image_data(&image_data, 0.0, 0.0)?;

        let p = get_paragraph("不可复制的图像")?;
        body.append_child(&p)?;
        body.append_child(&canvas)?;

        Ok(())
    }
}

impl Vip for User<VipPlan> {
    fn render_as_div(&self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let div_warp = document
            .create_element("div")?
            .dyn_into::<web_sys::HtmlElement>()?;
        div_warp.style().set_property("position", "relative")?;
        div_warp
            .style()
            .set_property("width", &format!("{}px", IMAGE_ATTRIBUTES.width))?;
        div_warp
            .style()
            .set_property("height", &format!("{}px", IMAGE_ATTRIBUTES.height))?;

        for item in &self.info {
            let div = document
                .create_element("div")?
                .dyn_into::<web_sys::HtmlElement>()?;
            div.set_inner_text(&item.cipher);
            div.style().set_property("position", "absolute")?;
            div.style()
                .set_property("left", &format!("{}px", item.position.x))?;
            div.style()
                .set_property("top", &format!("{}px", item.position.y))?;
            div.style()
                .set_property("font-size", &format!("{}px", item.font_style.size))?;
            div_warp.append_child(&div)?;
        }

        let p = get_paragraph("VIP 专用：可复制的普通元素")?;
        body.append_child(&p)?;
        body.append_child(&div_warp)?;

        Ok(())
    }
}

impl From<User<FreePlan>> for User<VipPlan> {
    fn from(user: User<FreePlan>) -> Self {
        Self::new(user.user_token, user.vip_level, user.info)
    }
}

fn get_paragraph(text: &str) -> Result<HtmlElement, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let p = document
        .create_element("p")?
        .dyn_into::<web_sys::HtmlElement>()?;
    p.set_inner_text(text);
    Ok(p)
}

async fn req_font() -> Result<Vec<u8>, JsValue> {
    let window = web_sys::window().unwrap();
    let request = Request::new_with_str(FONT_URL)?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let unit8 = js_sys::Uint8Array::new(&JsFuture::from(resp.array_buffer()?).await?);
    Ok(unit8.to_vec())
}

async fn load_font<'a>() -> Option<Font<'a>> {
    Font::try_from_vec(req_font().await.unwrap())
}

fn draw_text_in_image(image: &mut RgbaImage, font: &Font, render_string: &RenderString) {
    draw_text_mut(
        image,
        Rgba([0u8, 0u8, 0u8, u8::MAX]),
        render_string.position.x,
        render_string.position.y,
        Scale {
            x: render_string.font_style.size,
            y: render_string.font_style.size,
        },
        &font,
        &render_string.cipher,
    );
}

/// 解密信息
fn decrypt_info(info: &mut Vec<RenderString>) {
    for item in info {
        item.cipher = decrypt(&item.cipher);
    }
}

#[allow(dead_code)]
/// 简单模拟一个字符串加密
fn encrypt(str: &String) -> String {
    let u: Vec<u8> = (str
        .to_owned()
        .into_bytes()
        .iter()
        .map(|char| char ^ KEY)
        .collect::<Vec<u8>>())
    .to_vec();
    String::from_utf8_lossy(&u).to_string()
}

/// 简单模拟一个字符串解密
fn decrypt(str: &String) -> String {
    let u: Vec<u8> = (str
        .to_owned()
        .into_bytes()
        .iter()
        .map(|char| char ^ KEY)
        .collect::<Vec<u8>>())
    .to_vec();
    String::from_utf8_lossy(&u).to_string()
}

#[test]
fn test() {
    let str = String::from("bar");
    let cipher = encrypt(&str);
    let plaintext = decrypt(&cipher);
    println!("{:?}", cipher);
    assert_eq!(str, plaintext);
}
