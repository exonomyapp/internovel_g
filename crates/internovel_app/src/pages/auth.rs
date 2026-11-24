use internovel_auth::wallet::{
    create_wallet, gen_shuffled_pazzle, unlock_with_password, CreateWalletParams, KdfProfile,
    WalletEnvelope,
};
use leptos::prelude::*;

use crate::{AppState, AuthorAccount};

#[component]
pub fn AuthPage() -> impl IntoView {
    let state = expect_context::<AppState>();

    let display_name = RwSignal::new("Alex Rivera".to_string());
    let security_text = RwSignal::new("amber-lantern-harbor".to_string());
    let password = RwSignal::new(String::new());
    let in_memory = RwSignal::new(false);
    let import_code = RwSignal::new(String::new());
    let qr_svg = RwSignal::new(Option::<String>::None);
    let generated_mnemonic = RwSignal::new(Option::<String>::None);
    let active_did_long = RwSignal::new(Option::<String>::None);
    let status_msg = RwSignal::new(Option::<String>::None);
    let shuffled = RwSignal::new(gen_shuffled_pazzle(4));
    let confirm_delete_did = RwSignal::new(Option::<String>::None);

    let on_create = move |_| {
        match create_wallet(CreateWalletParams {
            display_name: display_name.get(),
            security_text: security_text.get(),
            password: Some(password.get()),
            enable_mnemonic: true,
            pazzle: Some(vec![0, 1, 2, 3]),
            pin: Some([1, 2, 3, 4]),
            in_memory_only: in_memory.get(),
            profile: KdfProfile::MobileConstrained,
        }) {
            Ok(created) => {
                let bundle = created.sensitive.to_peer_did4("local-iroh-endpoint");
                let text_code = created.envelope.to_text_code().unwrap_or_default();
                qr_svg.set(created.envelope.to_pairing_qr_svg().ok());
                generated_mnemonic.set(created.mnemonic_words);
                active_did_long.set(Some(bundle.long_form.clone()));
                import_code.set(text_code);

                let acc = AuthorAccount {
                    display_name: created.sensitive.display_name.clone(),
                    bio: "Collaborative novelist.".into(),
                    avatar_initials: created
                        .sensitive
                        .display_name
                        .chars()
                        .take(2)
                        .collect::<String>()
                        .to_uppercase(),
                    avatar_url: String::new(),
                    did_short: bundle.short_form,
                    did_long: bundle.long_form,
                    security_text: security_text.get(),
                    in_memory: in_memory.get(),
                };
                state.accounts.update(|list| list.push(acc.clone()));
                state.active_account.set(Some(acc));
                status_msg.set(Some(
                    "Wallet created locally with Argon2id + Aes256GcmSiv & bound to did:peer:4."
                        .into(),
                ));
            }
            Err(e) => status_msg.set(Some(format!("Wallet creation error: {e}"))),
        }
    };

    let on_update_active = move |_| {
        let name_val = display_name.get().trim().to_string();
        let sec_val = security_text.get().trim().to_string();
        let mem_val = in_memory.get();
        let active_did = state.active_account.get().map(|a| a.did_short);
        if let Some(did) = active_did {
            state.active_account.update(|slot| {
                if let Some(acc) = slot {
                    if !name_val.is_empty() {
                        acc.display_name = name_val.clone();
                        acc.avatar_initials = name_val.chars().take(2).collect::<String>().to_uppercase();
                    }
                    if !sec_val.is_empty() {
                        acc.security_text = sec_val.clone();
                    }
                    acc.in_memory = mem_val;
                }
            });
            state.accounts.update(|list| {
                if let Some(acc) = list.iter_mut().find(|a| a.did_short == did) {
                    if !name_val.is_empty() {
                        acc.display_name = name_val.clone();
                        acc.avatar_initials = name_val.chars().take(2).collect::<String>().to_uppercase();
                    }
                    if !sec_val.is_empty() {
                        acc.security_text = sec_val.clone();
                    }
                    acc.in_memory = mem_val;
                }
            });
            status_msg.set(Some("Updated active identity name and anti-phishing phrase.".into()));
        }
    };

    let on_import_unlock = move |_| {
        match WalletEnvelope::from_text_code(&import_code.get()) {
            Ok(env) => {
                let banner = env.content.security_banner.security_text.clone();
                match unlock_with_password(&env, &password.get(), KdfProfile::MobileConstrained) {
                    Ok(sensitive) => {
                        let bundle = sensitive.to_peer_did4("local-iroh-endpoint");
                        active_did_long.set(Some(bundle.long_form.clone()));
                        let acc = AuthorAccount {
                            display_name: sensitive.display_name.clone(),
                            bio: "Collaborative novelist.".into(),
                            avatar_initials: sensitive.display_name.chars().take(2).collect(),
                            avatar_url: String::new(),
                            did_short: bundle.short_form,
                            did_long: bundle.long_form,
                            security_text: banner,
                            in_memory: sensitive.in_memory_only,
                        };
                        state.accounts.update(|list| {
                            if !list.iter().any(|existing| existing.did_short == acc.did_short) {
                                list.push(acc.clone());
                            }
                        });
                        state.active_account.set(Some(acc));
                        status_msg.set(Some("Wallet signature verified and unlocked locally.".into()));
                    }
                    Err(e) => status_msg.set(Some(format!(
                        "Unlock failed (Security Banner '{banner}' verified): {e}"
                    ))),
                }
            }
            Err(e) => status_msg.set(Some(format!("Invalid TextCode / QR payload: {e}"))),
        }
    };

    view! {
        <div class="grid gap-6 md:grid-cols-2">
            <section class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-5 space-y-4">
                <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">
                    "Author Identity & Local Wallet"
                </h1>
                <p class="text-xs opacity-80">
                    "Your stories and identity keys stay locally on your device. Create a new author wallet or unlock an existing transfer code."
                </p>

                <div class="space-y-2 text-xs">
                    <label class="block">
                        <span class="font-medium">"Author Display Name"</span>
                        <input
                            type="text"
                            class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                            prop:value=move || display_name.get()
                            on:input=move |ev| display_name.set(event_target_value(&ev))
                        />
                    </label>
                    <label class="block">
                        <span class="font-medium">"Anti-Phishing Security Phrase (verified pre-unlock)"</span>
                        <input
                            type="text"
                            class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                            prop:value=move || security_text.get()
                            on:input=move |ev| security_text.set(event_target_value(&ev))
                        />
                    </label>
                    <label class="block">
                        <span class="font-medium">"Passphrase"</span>
                        <input
                            type="password"
                            class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                            prop:value=move || password.get()
                            on:input=move |ev| password.set(event_target_value(&ev))
                        />
                    </label>
                    <label class="flex items-center gap-2 pt-1">
                        <input
                            type="checkbox"
                            prop:checked=move || in_memory.get()
                            on:change=move |ev| in_memory.set(event_target_checked(&ev))
                        />
                        <span>"Ephemeral In-Memory Session (zero disk persistence)"</span>
                    </label>
                </div>

                <div class="rounded-xl bg-[#f5f5f0] dark:bg-[#252522] p-3 text-xs space-y-2">
                    <div class="flex items-center justify-between">
                        <div class="font-medium">"Shuffled PIN Pad Preview (Shoulder-Surfing Resistant)"</div>
                        <button
                            type="button"
                            class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                            on:click=move |_| shuffled.set(gen_shuffled_pazzle(4))
                        >"Shuffle"</button>
                    </div>
                    <div class="flex flex-wrap gap-1.5">
                        {move || shuffled.get().pin_digits.iter().map(|d| view! {
                            <span class="px-2 py-0.5 rounded bg-white dark:bg-[#1a1a18] border border-[#e5e5e0] dark:border-[#2d2d2a] font-mono">{d.to_string()}</span>
                        }).collect_view()}
                    </div>
                </div>

                <div class="flex flex-wrap gap-2">
                    <button
                        type="button"
                        class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white text-xs font-medium"
                        on:click=on_create
                    >
                        "Create Local Wallet & did:peer:4"
                    </button>
                    <Show when=move || state.active_account.get().is_some()>
                        <button
                            type="button"
                            class="px-3 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] text-xs font-medium"
                            on:click=on_update_active
                        >
                            "Update Active Identity"
                        </button>
                    </Show>
                    <button
                        type="button"
                        class="px-3 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] text-xs font-medium"
                        on:click=on_import_unlock
                    >
                        "Verify & Unlock TextCode"
                    </button>
                </div>

                <Show when=move || status_msg.get().is_some()>
                    <p class="text-xs font-medium text-[#5A5A40] dark:text-[#d0d0c8]">
                        {move || status_msg.get().unwrap_or_default()}
                    </p>
                </Show>
            </section>

            <section class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-5 space-y-4 text-xs">
                <h2 class="font-serif text-lg font-bold text-[#5A5A40] dark:text-[#e6e6de]">
                    "Saved Identities & Cross-Device Pairing"
                </h2>

                <div class="space-y-2">
                    <For
                        each=move || state.accounts.get()
                        key=|a| a.did_short.clone()
                        children=move |acc| {
                            let did = acc.did_short.clone();
                            let did_use = did.clone();
                            let did_confirm = did.clone();
                            let did_del_btn = did.clone();
                            let acc_use = acc.clone();
                            view! {
                                <div class="rounded-xl bg-[#f5f5f0] dark:bg-[#252522] p-3 flex items-center justify-between gap-2">
                                    <div class="min-w-0 flex-1">
                                        <div class="font-medium">{acc.display_name}</div>
                                        <div class="text-[11px] opacity-80">{format!("Phrase: {}", acc.security_text)}</div>
                                        <div class="font-mono text-[10px] opacity-65 truncate">{did.clone()}</div>
                                    </div>
                                    <div class="flex items-center gap-1.5">
                                        <button
                                            type="button"
                                            class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                            on:click=move |_| state.active_account.set(Some(acc_use.clone()))
                                        >"Use"</button>
                                        {move || if confirm_delete_did.get().as_deref() == Some(did_confirm.as_str()) {
                                            let did_do = did_confirm.clone();
                                            view! {
                                                <button
                                                    type="button"
                                                    class="px-2 py-1 rounded bg-red-700 text-white"
                                                    on:click=move |_| {
                                                        state.accounts.update(|list| list.retain(|a| a.did_short != did_do));
                                                        if state.active_account.get().map(|a| a.did_short) == Some(did_do.clone()) {
                                                            state.active_account.set(state.accounts.get().first().cloned());
                                                        }
                                                        confirm_delete_did.set(None);
                                                    }
                                                >"Confirm"</button>
                                                <button
                                                    type="button"
                                                    class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                    on:click=move |_| confirm_delete_did.set(None)
                                                >"Cancel"</button>
                                            }.into_any()
                                        } else {
                                            let d_trig = did_del_btn.clone();
                                            view! {
                                                <button
                                                    type="button"
                                                    class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400"
                                                    on:click=move |_| confirm_delete_did.set(Some(d_trig.clone()))
                                                >"Delete"</button>
                                            }.into_any()
                                        }}
                                        <span class="hidden">{did_use.clone()}</span>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>

                <label class="block">
                    <span class="font-medium">"Portable Encrypted Wallet TextCode (inw4:...)"</span>
                    <textarea
                        rows="4"
                        class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] p-2 font-mono text-[11px]"
                        prop:value=move || import_code.get()
                        on:input=move |ev| import_code.set(event_target_value(&ev))
                    />
                </label>

                <Show when=move || generated_mnemonic.get().is_some()>
                    <div class="rounded-xl border border-amber-300 dark:border-amber-800 bg-amber-50 dark:bg-amber-950/30 p-3">
                        <div class="font-semibold mb-1">"12-Word BIP-39 Recovery Phrase"</div>
                        <p class="font-mono text-[11px]">{move || generated_mnemonic.get().unwrap_or_default()}</p>
                    </div>
                </Show>

                <Show when=move || active_did_long.get().is_some()>
                    <div class="rounded-xl bg-[#f5f5f0] dark:bg-[#252522] p-3">
                        <div class="font-semibold mb-1">"Long-Form did:peer:4 (Initial Invite & Iroh Service)"</div>
                        <p class="font-mono text-[10px] break-all">{move || active_did_long.get().unwrap_or_default()}</p>
                    </div>
                </Show>

                <Show when=move || qr_svg.get().is_some()>
                    <div class="flex flex-col items-center p-3 rounded-xl bg-white border border-[#e5e5e0]" inner_html=move || qr_svg.get().unwrap_or_default()/>
                </Show>
            </section>
        </div>
    }
}
