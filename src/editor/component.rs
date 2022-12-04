use crate::editor::get_options;
use monaco::yew::CodeEditor;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Editor)]
pub fn editor() -> Html {
    html! {
        <CodeEditor
            classes={css!(r#"
                height: 80vh;
                width: 80vw;
            "#)}
            options={get_options()}
        />
    }
}
