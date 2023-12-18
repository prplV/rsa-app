#![windows_subsystem = "windows"]
// mods
mod server;
mod client;

// uses

extern crate rand;
extern crate num;

pub use crate::rand::Rng;
pub use num::BigUint;
pub use crate::num::bigint::ToBigUint;
pub use crate::num::{FromPrimitive, ToPrimitive};
pub use server::Server;
pub use client::Client;

//#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}
fn app(cx: Scope) -> Element {
    cx.render(
        rsx! {
            style { include_str!("./style/rsa-app.css") }
            h1 { class: "h1-main",  onclick: move |_| {
                simple_message_box::create_message_box("RSA-app.rs было разработано студентом группы БСБО-01-21 Дроздовым Владиславом", "О программе");
                simple_message_box::create_message_box("Приложение полностью написано на языке Rust с использованием крейтов num, rand, dioxus, dioxus-desktop и конечно же simple_message_box", "О программе");
                simple_message_box::create_message_box("Основная задача - продемонстрировать работу RSA алгоритма аутентификации", "О программе");
            }, "RSA-app.rs"}
            div { class: "explaination-h1", "Это приложение-симуляция работы RSA-алгоритма аутентификации"}
            div { class: "explaination-h2","В роли проверяющего узла выступает структура Server с принадлежащим ей функционалом по генерированию ключей и т.д. В роли проверяемого узла выступает пользователь и функционал структуры Client."}
            div { class: "explaination-h3","От пользователя требуется только нажать на кнопку 'Start', все вычисления производятся автоматически и поэтапно"}
            div { class: "start-btn", onclick: move |_| {
                let mut server: Server = Server::new();
                let mut client: Client = Client::new();

                server.gen_key_pair();
                let publ = server.get_public_key();

                // message about getting public key
                let msg = String::from("Сервер (проверяющий узел) сгенерировал пару ключей и передал публичный. Полученный публичный ключ - (") + &publ[0].to_string() + &String::from("; ") + &publ[1].to_string() + &String::from(")");
                simple_message_box::create_message_box(&msg, "Первый этап");
                let temp = client.calculating_r_value(publ);

                // message about r calc
                simple_message_box::create_message_box("Клиент произвел расчеты r числа (зашифрованный вид случайного числа k), используя публичный ключ, полученный ранее. Это число передается проверяющему узлу", "Второй этап");

                let t = server.calculating_initial_kc_value(temp);
                match client.validation(t){
                    // message about validation result
                    true => simple_message_box::create_message_box("Сервер расшифровал r и вернул k'. Клиент сравнил k и k'. В данном случае аутентификация пройдена успешно и значения равны", "Третий этап"),
                    false => simple_message_box::create_message_box("Сервер расшифровал r и вернул k'. Клиент сравнил k и k'. В данном случае аутентификация не пройдена, значения не равны", "Третий этап"),
                };
            }, "● Start ●"}
        }
    )
}