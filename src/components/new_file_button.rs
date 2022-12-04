use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(NewFileButton)]
pub fn render() -> Html {
    let onclick = Callback::from(|_| {
        log::info!("New file button clicked");
    });

    html! {
        <>
        <svg {onclick} version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px"
           viewBox="0 0 485 485" style="enable-background:new 0 0 485 485;" space="preserve"
            class={css!(r#"
                width: 1rem;
                height: 1rem;
                cursor: pointer;
                transition: fill 0.2s ease-in-out;
                &:hover {
                    fill: #000;
        }"#)}>
            <g>
              <rect x="85.285" y="192.5" width="200" height="30"/>
              <path d="M350.285,227.015V108.787L241.499,0H20.285v445h221.522c23.578,24.635,56.766,40,93.478,40
                c71.368,0,129.43-58.062,129.43-129.43C464.715,289.276,414.612,234.474,350.285,227.015z M250.285,51.213L299.072,100h-48.787
                V51.213z M50.285,415V30h170v100h100v97.015c-28.908,3.352-54.941,16.262-74.846,35.485H85.285v30h137.014
                c-6.865,12.25-11.802,25.719-14.382,40H85.285v30h120.757c0.999,18.842,6.049,36.626,14.289,52.5H50.285z M335.285,455
                c-54.826,0-99.43-44.604-99.43-99.43s44.604-99.429,99.43-99.429s99.43,44.604,99.43,99.429S390.111,455,335.285,455z"/>
              <polygon points="350.285,293.955 320.285,293.955 320.285,340.57 273.669,340.57 273.669,370.57 320.285,370.57 320.285,417.187
                350.285,417.187 350.285,370.57 396.901,370.57 396.901,340.57 350.285,340.57 	"/>
            </g>
        </svg>
        </>
    }
}
