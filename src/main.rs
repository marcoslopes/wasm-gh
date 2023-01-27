mod link;

use self::link::Link;
use yew::prelude::*;

#[function_component(Hello)]
fn hello() -> Html {
    let links = vec![
        Link {
            title: "Google".to_string(),
            href: "https://www.google.com".to_string(),
        },
        Link {
            title: "HN".to_string(),
            href: "https://news.ycombinator.com".to_string(),
        },
    ];

    let links = links
        .iter()
        .map(|link| {
            html! {
                <>
                <a href={link.href.clone()}>{format!("Follow: {}", link.title)}</a>
                <br/>
                </>
            }
        })
        .collect::<Html>();
    html! {
        <>
        <h1>{"Hello Marcos"}</h1>
        {links}
        </>
    }
}

fn main() {
    yew::Renderer::<Hello>::new().render();
}
