use crate::components::mascot::Mascot;
use crate::config::APP_NAME;
use crate::content;
use leptos::*;

/// Public landing page: hero, banner, feature cards, FAQ accordion and footer.
#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <header class="topbar">
            <a class="brand" href="#/">
                <span class="brand-dot" aria-hidden="true"></span>
                <span>{APP_NAME}</span>
            </a>
            <nav class="topbar-actions" aria-label="Account">
                <a class="btn btn-ghost" href="#/signin">"Sign In"</a>
                <a class="btn btn-primary" href="#/signup">"Create Account"</a>
            </nav>
        </header>

        <section class="hero">
            <div class="hero-copy">
                <p class="eyebrow">{content::HERO_EYEBROW}</p>
                <h1>{content::HERO_HEADLINE}</h1>
                <p class="lead">{content::HERO_SUBTITLE}</p>
                <div class="hero-buttons">
                    <a class="btn btn-primary btn-lg" href="#/signup">"Create Account"</a>
                    <a class="btn btn-ghost btn-lg" href="#/signin">"Sign In"</a>
                </div>
            </div>
            <div class="hero-mascot">
                <div class="speech-bubble">{content::WELCOME_MESSAGE}</div>
                <Mascot mood="happy"/>
            </div>
        </section>

        <div class="banner" role="note">{content::BANNER_TEXT}</div>

        <section class="section" aria-labelledby="features-title">
            <h2 id="features-title">"Why people use My Personal Vault"</h2>
            <div class="card-grid">
                {content::FEATURES
                    .iter()
                    .map(|&(title, text)| {
                        view! {
                            <article class="card">
                                <h3>{title}</h3>
                                <p>{text}</p>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        <section class="section" aria-labelledby="faq-title">
            <h2 id="faq-title">"Frequently asked questions"</h2>
            {content::FAQ_ITEMS
                .iter()
                .map(|&(question, answer)| {
                    view! {
                        <details class="faq-item">
                            <summary>{question}</summary>
                            <p>{answer}</p>
                        </details>
                    }
                })
                .collect_view()}
        </section>

        <footer class="footer">
            <p>{content::FOOTER_TEXT}</p>
            <p>{content::FOOTER_NOTE}</p>
        </footer>
    }
}