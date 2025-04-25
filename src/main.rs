use dioxus::prelude::*;
use dioxus_router::prelude::*;
use rust_i18n::t;

const PICO_CSS: Asset = asset!("/node_modules/@picocss/pico/css/pico.min.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

rust_i18n::i18n!("locales", fallback = "en");

#[derive(Routable, PartialEq, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/?:time&:lang")]
    Settings { 
        time: String, 
        lang: String, 
    },
}

fn main() {
    #[cfg(feature = "desktop")]
    fn launch_app() {
        use dioxus::desktop::tao;
        let window = tao::window::WindowBuilder::new().with_resizable(true);
        dioxus::LaunchBuilder::new().with_cfg(dioxus::desktop::Config::new().with_window(window).with_menu(None)).launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    fn launch_app() {
        dioxus::launch(App);
    }

    launch_app();
}

#[component]
fn App() -> Element {
    rust_i18n::set_locale("en");
    rsx! {
        document::Link { rel: "stylesheet", href: PICO_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Title { "Dailybible Settings" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Meta { name: "color-scheme", content: "light dark" }
        main {  
            class: "container",
            Router::<Route> {}
        }       
    }
}

#[component]
fn Settings(time: String, lang: String) -> Element {

    let mut timer_activated = use_signal(|| false);
    let mut timer_time = use_signal(|| "".to_string());

    use_effect(move || {
        match time.is_empty() {
            true => {
                timer_activated.set(false);
                timer_time.set("12:00".to_string());
            },
            false => {
                timer_activated.set(true);
                timer_time.set(time.clone());
            }
        }
    });
    

    let mut lang = use_signal(|| match lang.is_empty() {
        true => "en".to_string(),
        false => lang,
    });

    rsx! {
        div {
            id: "settings",
            h3 { {t!("settings", locale = lang.read().as_str())} }
            p { {t!("settings_description", locale = lang.read().as_str())} }
            form {
                fieldset {
                    div {
                        label { {t!("language", locale = lang.read().as_str())} }
                        select {
                            onchange: move |lang_event| {
                                lang.set(lang_event.value());
                                rust_i18n::set_locale(lang.read().as_str());
                            },
                            value: lang,
                            option { value: "en", "English" }
                            option { value: "de", "Deutsch" },
                        }
                        
                    }
                    div {
                        label { {t!("timer", locale = lang.read().as_str())} }
                        input {  
                            name: "timer_activated",
                            type: "checkbox",
                            role: "switch",
                            checked: timer_activated,
                            oninput: move |checked_event| 
                                timer_activated.set(checked_event.checked()),
                        }
                        {t!("activate_timer", locale = lang.read().as_str())},
                    }
                    if *timer_activated.read() {
                        TimerSettings { 
                            time_signal: timer_time,
                            lang: lang.read().clone(),
                        } 
                    }
                }
            }
        }
    }
}

#[component]
fn TimerSettings(time_signal: Signal<String>, lang: String) -> Element {
    rsx! {
        div {
            class: "fade-in",
            label { {t!("time", locale = lang.as_str())} }
            input {
                name: "timer_time",
                type: "time",
                value: "{time_signal}",
                oninput: move |time_event| {
                    time_signal.set(time_event.value());
                },
            }
            {t!("reminder_notice", 
                locale = lang.as_str(),
                time = time_signal.read())},
        }
    }
}
