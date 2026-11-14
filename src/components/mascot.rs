use leptos::*;

/// Vaulty, the animated mascot.
/// `mood` changes the colour accents. Supported values: "happy" and "thinking".
#[component]
pub fn Mascot(#[prop(into)] mood: String) -> impl IntoView {
    let class_name = format!("mascot mood-{mood}");

    view! {
        <div class=class_name aria-hidden="true">
            <svg class="mascot-svg" viewBox="0 0 200 220" xmlns="http://www.w3.org/2000/svg">
                <line class="mascot-antenna" x1="100" y1="46" x2="100" y2="24"></line>
                <circle class="mascot-antenna-ball" cx="100" cy="16" r="11"></circle>
                <rect class="mascot-arm mascot-arm-left" x="22" y="110" width="26" height="14" rx="7"></rect>
                <rect class="mascot-arm mascot-arm-right" x="152" y="110" width="26" height="14" rx="7"></rect>
                <rect class="mascot-body" x="42" y="46" width="116" height="120" rx="34"></rect>
                <rect class="mascot-visor" x="62" y="78" width="76" height="46" rx="18"></rect>
                <ellipse class="mascot-eye" cx="84" cy="101" rx="8" ry="10"></ellipse>
                <ellipse class="mascot-eye" cx="116" cy="101" rx="8" ry="10"></ellipse>
                <path class="mascot-mouth" d="M 86 142 Q 100 152 114 142"></path>
                <rect class="mascot-foot" x="60" y="166" width="32" height="18" rx="9"></rect>
                <rect class="mascot-foot" x="108" y="166" width="32" height="18" rx="9"></rect>
            </svg>
        </div>
    }
}