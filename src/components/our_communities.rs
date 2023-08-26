use leptos::*;

use crate::components::icons::discord_icon::DiscordIcon;
use crate::components::icons::github_icon::GithubIcon;
use crate::components::icons::telegram_icon::TelegramIcon;
use crate::components::button_link::ButtonLink;

#[component]
pub fn OurCommunities(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section class="bg-orange-300/30">
            <div class="container mx-auto px-4">
                <div class="flex flex-col items-center py-20 gap-y-6">
                    <h2 class="text-4xl text-center mb-4">
                        <span class="font-work-sans font-light">"Unete a nuestra "</span>
                        <span class="font-alfa-slab text-orange-500">"Comunidad"</span>
                    </h2>
                    <div class="flex items-center gap-x-12">
                        <ButtonLink
                            href="https://discord.gg/4ng5HgmaMg".to_string()
                            color="white".to_string()
                            size="big".to_string()
                        >
                            <DiscordIcon size=30 />
                            "Discord"
                        </ButtonLink>
                        <ButtonLink
                            href="https://github.com/RustLangES".to_string()
                            color="white".to_string()
                            size="big".to_string()
                        >
                            <GithubIcon size=30 />
                            "Github"
                        </ButtonLink>
                        <ButtonLink
                            href="https://t.me/rust_es".to_string()
                            color="white".to_string()
                            size="big".to_string()
                        >
                            <TelegramIcon size=30 />
                            "Telegram"
                        </ButtonLink>
                    </div>
                </div>
            </div>
        </section>
    }
}
