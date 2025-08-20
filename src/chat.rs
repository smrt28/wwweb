use yew::prelude::*;

/// Verdict shown as the leading badge.
#[derive(Clone, PartialEq)]
pub enum Verdict {
    Yes,
    No,
    Wrong,
    Other(String), // e.g. "Maybe", "N/A", "Hmm"
}

impl Verdict {
    fn label(&self) -> &str {
        match self {
            Verdict::Yes => "Yes",
            Verdict::No => "No",
            Verdict::Wrong => "Wrong",
            Verdict::Other(s) => s.as_str(),
        }
    }
    /// CSS class controlling the badge colors.
    fn class(&self) -> &'static str {
        match self {
            Verdict::Yes => "badge yes",
            Verdict::No => "badge no",
            Verdict::Wrong => "badge wrong",
            Verdict::Other(_) => "badge other",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct QaItem {
    pub question: AttrValue,
    pub verdict: Verdict,
    pub explanation: AttrValue, // keep it short; paragraph-ish
}

#[derive(Properties, PartialEq)]
pub struct QaListProps {
    pub items: Vec<QaItem>,
    /// Fired when user submits the ask box; payload is the raw text.
    #[prop_or_default]
    pub on_submit: Callback<String>,

    #[prop_or(AttrValue::from("Guesses"))]
    pub heading: AttrValue,
}

#[function_component(QaList)]
pub fn qa_list(props: &QaListProps) -> Html {
    let text = use_state(|| String::new());
    let on_change = {
        let text = text.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
            text.set(target.value());
        })
    };
    let on_submit_click = {
        let text = text.clone();
        let cb = props.on_submit.clone();
        Callback::from(move |_| {
            let val = (*text).trim().to_owned();
            if !val.is_empty() {
                cb.emit(val.clone());
                text.set(String::new());
            }
        })
    };

    html! {
        <div class="qa-wrap">
            <style>{r#"
                .qa-wrap {
                    font-family: system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif;
                    color: #222;
                }
                .qa-heading { margin: 24px 0 12px; font-size: 1.5rem; font-weight: 650; }
                .qa-list { display: flex; flex-direction: column; gap: 16px; margin-bottom: 24px; }
                .qa-item {
                    background: #fff;
                    border: 1px solid #ddd;
                    border-radius: 8px;
                    padding: 16px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
                }
                .qa-item .question { font-weight: 600; margin-bottom: 8px; }
                .answer-line { display: flex; align-items: flex-start; gap: 8px; }


.badge {
    font-weight: 700;
    text-transform: uppercase;
    margin-right: .4em;
}

.badge.yes   { color: #111; font-weight: 800; }
.badge.no    { color: #333; font-weight: 800; }
.badge.wrong { color: #555; font-style: italic; }
.badge.other { color: #444; font-weight: 700; }


                .explanation { color: #555; }
                .ask-box { background:#fff; border:1px solid #ddd; border-radius:8px; padding:16px; }
                .ask-label { display:block; margin-bottom:8px; font-weight:600; }
                .ask-input { width:100%; padding:10px; border:1px solid #ccc; border-radius:4px; font:inherit; resize: vertical; }
                .ask-btn { margin-top:10px; padding:10px 16px; border:none; border-radius:4px; background:#0077cc; color:white; cursor:pointer; font:inherit; }
                .ask-btn:hover { background:#005fa3; }
            "#}</style>

            <h1 class="qa-heading">{ props.heading.clone() }</h1>

            <div class="qa-list">
                { for props.items.iter().map(|it| html!{
                    <div class="qa-item">
                        <div class="question">{ it.question.clone() }</div>
                        <div class="answer-line">
                            <span class={it.verdict.class()}>{ it.verdict.label() }</span>
                            <span class="explanation">{ it.explanation.clone() }</span>
                        </div>
                    </div>
                })}
            </div>

            <div class="ask-box">
                <label class="ask-label" for="ask-input">{ "Ask a new question:" }</label>
                <textarea
                    id="ask-input"
                    class="ask-input"
                    rows={3}
                    placeholder="Type your question here…"
                    value={(*text).clone()}
                    oninput={on_change}
                />
                <br/>
                <button type="button" class="ask-btn" onclick={on_submit_click}>{ "Submit" }</button>
            </div>
        </div>
    }
}
