mod utils;

use cfg_if::cfg_if;
use lazy_static::lazy_static;
use worker::*;

use crate::utils::set_panic_hook;

lazy_static! {
    static ref SITE_URI: &'static str = "https://fnlog.dev";
    static ref KV_BINDING: &'static str = "PAGE_COUNTER";
}

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;

        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    set_panic_hook();

    if matches!(req.method(), Method::Options) {
        let mut cors_headers = Headers::new();

        cors_headers.set("Access-Control-Allow-Origin", &SITE_URI)?;
        cors_headers.set("Access-Control-Allow-Methods", "POST,OPTIONS")?;
        cors_headers.set("Access-Control-Max-Age", "86400")?;
        cors_headers.set(
            "Access-Control-Allow-Headers",
            &req.headers()
                .get("Access-Control-Request-Headers")
                .ok()
                .unwrap()
                .unwrap_or("".to_owned()),
        )?;

        return Ok(Response::ok("")?.with_headers(cors_headers));
    }

    if matches!(req.method(), Method::Post) {
        let kv = env.kv(&KV_BINDING)?;
        let path = req.path();

        let counter = kv
            .get(&path)
            .await?
            .map(|val| val.as_string())
            .and_then(|txt| txt.parse::<usize>().ok())
            .unwrap_or(0);

        let new_counter = (counter + 1).to_string();

        kv.put(&path, &new_counter)?.execute().await?;

        let mut cors_headers = Headers::new();
        cors_headers.set("Access-Control-Allow-Origin", &SITE_URI)?;

        return Ok(Response::ok(path)?.with_headers(cors_headers));
    }

    Response::error("Bad Request", 400)
}
