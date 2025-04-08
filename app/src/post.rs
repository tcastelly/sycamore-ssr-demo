use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::{thread, time};
use sycamore::rt::Suspense;
use sycamore::{futures::spawn_local_scoped, prelude::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostData {
    title: String,
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostList {
    posts: Vec<PostData>,
}

async fn getPostList() -> Option<PostList> {
    let resp = Request::get(&format!("/posts")).send().await.unwrap();

    println!("response status: {:?}", resp.status());

    Some(resp.json().await.expect("cannot parse post list"))
}

async fn getPost(path: String) -> Option<String> {
    let resp = Request::get(&format!("/posts/{}", path))
        .send()
        .await
        .unwrap();

    Some(resp.text().await.expect("cannot parse post text"))
}

#[component(inline_props)]
async fn DisplayPost(path: String) -> View {
    let html = create_signal(String::new());
    let mut post = "".to_string();

    is_ssr! {
       post = getPost(path.clone()).await.unwrap();
    }

    html.set(post);

    let container_ref = create_node_ref();

    create_effect(move || {
        if let Some(dom_node) = container_ref.try_get() {
            dom_node.set_text_content(Some(html.get_clone().as_str()));
        }
    });

    view! {
        div(class="container", ref=container_ref)
    }
}

#[component()]
async fn DisplayPostList() -> View {
    let post_list = create_signal(None::<PostList>);

    is_ssr! {
      post_list.set(getPostList().await);
    }

    if let Some(post_list) = post_list.get_clone() {
        let templates: Vec<_> = post_list
            .posts
            .iter()
            .cloned()
            .map(|post| {
                let PostData { title, path } = post;
                view! {
                  li {
                    a(href=format!("/blog/{}", path)) { (title) }
                  }
                }
            })
            .collect();

        view! {
          ul {
            (templates)
          }
        }
    } else {
        view! { "Loading ..." }
    }
}

#[component]
pub fn PostList() -> View {
    view! {
      Suspense(fallback=|| view! { p { "Loading..." } }) {
        DisplayPostList()
      }
    }
}

#[component]
pub fn Post(path: String) -> View {
    view! {
      Suspense(fallback=|| view! { p { "Loading..." } }) {
        DisplayPost(path=path)
      }
    }
}
