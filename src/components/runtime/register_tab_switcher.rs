use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct RegisterTabSwitcherProps {
    pub children: Children,
}

#[styled_component(RegisterTabSwitcher)]
pub fn three_tab_switcher(props: &RegisterTabSwitcherProps) -> Html {
    let displayed = use_state(|| 0);

    let click_callback = |index| {
        let displayed = displayed.clone();
        Callback::from(move |_| {
            displayed.set(index);
        })
    };

    // Note - this cannot be a separate function
    // or the Style is dropped
    let button_classes = |index| {
        let style = Style::new(
            r#"
            flex: 1 1 0px;
            border-radius: 0px;
            border: none;
            border-top: 1px solid black;
            
            &:hover {
                cursor: pointer;
                background-color: #f5c6c6;
            }
            
        "#,
        );

        let style = style.unwrap();
        let second_class = Style::new(if *displayed == index {
            "background-color: #f5c6c6;border-left: 1px solid black;border-right: 1px solid black;"
        } else {
            "background-color: #fee2e2;"
        })
        .unwrap();

        classes!(style, second_class)
    };

    html! {
        <div id="register-tab-switcher__container" class={css!(r#"
            display: flex;
            flex-direction: column;
            align-items: center;
            height: 100%;
            justify-content: space-between;
            border-radius: 5px;
        "#)}>
            <div id="register-tab-switcher__current_display" class={css!(r#"
                height: 94%;
                width: 100%;
            "#)}>
                {props.children.iter().nth(*displayed).clone()}
            </div>
            <div id="three-tab-switcher__buttons" class={css!(r#"
                display: flex;
                flex-direction: row;
                width: 100%;
                height: 6%;
            "#)}>
                <button
                    class={button_classes(0)}
                    id="register-tab-switcher__used"
                    onclick={click_callback(0)}
                >
                    {"Used Registers"}
                </button>
                <button
                    id="register-tab-switcher__decompiled"
                    onclick={click_callback(1)}
                    class={button_classes(1)}
                >
                    {"All Registers"}
                </button>
            </div>
        </div>
    }
}
