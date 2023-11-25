use std::{rc::Rc, cell::RefCell};
use futures::stream::SplitSink;

use leptos::*;
use leptos_meta::*;

mod components;
use components::chat_area::ChatArea;
use components::type_area::TypeArea;

use crate::{model::conversation::{Conversation, Message}};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    use gloo_net::websocket::futures::WebSocket;
    use gloo_net::websocket::Message::Text as Txt;
    use futures::SinkExt;
    let client: Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>> = Default::default();

    // (read, write)
    let (conversation, set_conversation) = create_signal(Conversation::new());
    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        // send convo to server (api.rs)
        // conver(conversation.get())
        
        let client2 = client.clone();
        let msg = new_message.to_string();
        async move {
            client2
                .borrow_mut()
                .as_mut()
                .unwrap()
                .send(Txt(msg.to_string()))
                .await
                .map_err(|_| ServerFnError::ServerError("WebSocket issue".to_string()))
        }
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            })
        }
    });

    // create_effect(move |c| {
    //     if let Some(Ok(response)) = send.value().get() {
    //         set_conversation.update(move |c| {
    //             c.messages.last_mut().unwrap().text = response;
    //         })
    //     }
    // });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="loser, second try"/>
        // {conversation.get()}
        <ChatArea conversation/>
        <TypeArea send/>
    }
}

