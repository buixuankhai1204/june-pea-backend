use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::api::client;
use crate::api::types::ProductWithVariants;
use crate::state::cart::{CartItem, CartState};

// ─── SVG Icons ─────────────────────────────────────────────────────────────

fn icon_chevron_left() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="15 18 9 12 15 6"></polyline>
        </svg>
    }
}

fn icon_chevron_right() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="9 18 15 12 9 6"></polyline>
        </svg>
    }
}

fn icon_chevron_down() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 flex-shrink-0 transition-transform duration-300" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="6 9 12 15 18 9"></polyline>
        </svg>
    }
}

fn icon_heart() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path>
        </svg>
    }
}

fn icon_help() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"></circle>
            <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
            <line x1="12" y1="17" x2="12.01" y2="17"></line>
        </svg>
    }
}

fn icon_eye() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
            <circle cx="12" cy="12" r="3"></circle>
        </svg>
    }
}

fn icon_check_small() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 flex-shrink-0" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
    }
}

fn icon_check_box() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="9 11 12 14 22 4"></polyline>
            <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"></path>
        </svg>
    }
}

fn icon_gift() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="20 12 20 22 4 22 4 12"></polyline>
            <rect x="2" y="7" width="20" height="5"></rect>
            <line x1="12" y1="22" x2="12" y2="7"></line>
            <path d="M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z"></path>
            <path d="M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z"></path>
        </svg>
    }
}

fn icon_map_pin() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"></path>
            <circle cx="12" cy="10" r="3"></circle>
        </svg>
    }
}

// ─── Accordion ─────────────────────────────────────────────────────────────

#[component]
fn Accordion(icon: impl IntoView + 'static, title: String, children: ChildrenFn) -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <div class="border-t border-gray-200 first:border-t-0">
            <button
                class="w-full flex items-center justify-between py-4 text-left text-sm font-medium text-gray-800 hover:text-black transition-colors duration-150 cursor-pointer focus:outline-none"
                on:click=move |_| open.update(|v| *v = !*v)
            >
                <span class="flex items-center gap-2.5 text-gray-700">
                    {icon}
                    {title}
                </span>
                <span class=move || format!("transition-transform duration-300 text-gray-400 {}", if open.get() { "rotate-180" } else { "" })>
                    {icon_chevron_down()}
                </span>
            </button>
            ||{move || if open.get() {
                view! {
                    <div class="pb-5 text-sm text-gray-600 leading-relaxed animate-fadeInUp">
                        {children()}
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}

// ─── Main Component ────────────────────────────────────────────────────────

#[component]
pub fn ProductDetailPage() -> impl IntoView {
    let params = use_params_map();
    let cart = expect_context::<CartState>();
    let added_msg = RwSignal::new(false);
    let selected_color = RwSignal::new(Option::<String>::None);
    let selected_size = RwSignal::new(Option::<String>::None);
    let active_thumb = RwSignal::new(0usize);
    let quantity = RwSignal::new(1i32);

    let product = LocalResource::new(move || {
        let slug = params.read().get("slug").unwrap_or_default();
        async move {
            client::get::<ProductWithVariants>(&format!("/api/v1/catalog/products/{}", slug)).await
        }
    });

    view! {
        <div class="bg-white min-h-screen font-[Montserrat,system-ui,sans-serif]">

            // ── Breadcrumb ───────────────────────────────────────────────
            <div class="max-w-screen-xl mx-auto px-4 sm:px-6 pt-3 pb-1">
                <nav class="text-xs text-gray-500 flex items-center gap-1.5">
                    <a href="/" class="hover:text-black transition-colors cursor-pointer">"Trang chủ"</a>
                    <span class="text-gray-300">"›"</span>
                    <a href="/products" class="hover:text-black transition-colors cursor-pointer">"Bộ sưu tập"</a>
                    <span class="text-gray-300">"›"</span>
                    <span class="text-gray-700">"Sản phẩm"</span>
                </nav>
            </div>

            // ── Main Content ─────────────────────────────────────────────
            <div class="max-w-screen-xl mx-auto px-4 sm:px-6 py-4">
                <Suspense fallback=move || view! { <ProductDetailSkeleton /> }>
                    {move || {
                        let cart = cart.clone();
                        Suspend::new(async move {
                            match product.await {
                                Ok(data) => {
                                    let product_name = data.product.name.clone();

                                    let mut colors = std::collections::HashSet::new();
                                    let mut sizes = std::collections::HashSet::new();

                                    for v in &data.variants {
                                        if let Some(c) = v.attributes.get("color").and_then(|val| val.as_str()) {
                                            colors.insert(c.to_string());
                                        }
                                        if let Some(s) = v.attributes.get("size").and_then(|val| val.as_str()) {
                                            sizes.insert(s.to_string());
                                        }
                                    }

                                    let mut colors: Vec<_> = colors.into_iter().collect();
                                    colors.sort();
                                    let mut sizes: Vec<_> = sizes.into_iter().collect();
                                    sizes.sort();

                                    if selected_color.get_untracked().is_none() && !colors.is_empty() {
                                        selected_color.set(Some(colors[0].clone()));
                                    }
                                    if selected_size.get_untracked().is_none() && !sizes.is_empty() {
                                        selected_size.set(Some(sizes[0].clone()));
                                    }

                                    let matching_variant = Memo::new({
                                        let variants = data.variants.clone();
                                        move |_| {
                                            let c = selected_color.get();
                                            let s = selected_size.get();
                                            if let (Some(c), Some(s)) = (c, s) {
                                                variants.iter().find(|v| {
                                                    let vc = v.attributes.get("color").and_then(|val| val.as_str()).unwrap_or_default();
                                                    let vs = v.attributes.get("size").and_then(|val| val.as_str()).unwrap_or_default();
                                                    vc == c && vs == s
                                                }).cloned()
                                            } else {
                                                None
                                            }
                                        }
                                    });

                                    let has_sale = Memo::new(move |_| {
                                        matching_variant.get().and_then(|v| v.sale_price).is_some()
                                    });

                                    view! {
                                        // ── Two-column grid (60 / 40) ──────────────────
                                        <div class="grid grid-cols-1 lg:grid-cols-[minmax(0,1fr)_420px] gap-8 xl:gap-14 items-start">

                                            // ──── LEFT: Gallery ────────────────────────
                                            <div class="flex flex-col gap-3">

                                                // Main image
                                                <div class="relative bg-gray-50 rounded-sm overflow-hidden aspect-[3/4]">
                                                    // Red badge (show only when sale)
                                                    {move || if has_sale.get() {
                                                        view! {
                                                            <div class="absolute top-4 left-4 z-10 w-16 h-16 bg-red-500 rounded-full flex items-center justify-center text-white text-xs font-bold text-center leading-tight">
                                                                "Giá Rất Tốt"
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        view! { <div></div> }.into_any()
                                                    }}
                                                    // Placeholder image
                                                    <div class="w-full h-full flex items-center justify-center">
                                                        <svg xmlns="http://www.w3.org/2000/svg" class="w-24 h-24 text-gray-200" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="0.75">
                                                            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
                                                            <circle cx="8.5" cy="8.5" r="1.5"></circle>
                                                            <polyline points="21 15 16 10 5 21"></polyline>
                                                        </svg>
                                                    </div>
                                                </div>

                                                // Thumbnail strip
                                                <div class="flex items-start gap-2">
                                                    // Prev arrow
                                                    <button class="mt-2 p-1 text-gray-400 hover:text-black transition-colors cursor-pointer flex-shrink-0" on:click=move |_| {
                                                        active_thumb.update(|i| { if *i > 0 { *i -= 1; } });
                                                    }>
                                                        {icon_chevron_left()}
                                                    </button>
                                                    // Thumbnails
                                                    <div class="flex gap-2 flex-wrap">
                                                        {(0..5usize).map(|i| {
                                                            view! {
                                                                <button
                                                                    class=move || format!(
                                                                        "w-[76px] h-[100px] bg-gray-100 rounded-sm overflow-hidden flex-shrink-0 border-2 transition-all duration-150 cursor-pointer {}",
                                                                        if active_thumb.get() == i { "border-black" } else { "border-transparent hover:border-gray-300" }
                                                                    )
                                                                    on:click=move |_| active_thumb.set(i)
                                                                >
                                                                    <div class="w-full h-full flex items-center justify-center">
                                                                        <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8 text-gray-300" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
                                                                            <rect x="3" y="3" width="18" height="18" rx="2"></rect>
                                                                            <polyline points="21 15 16 10 5 21"></polyline>
                                                                        </svg>
                                                                    </div>
                                                                </button>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                    // Next arrow
                                                    <button class="mt-2 p-1 text-gray-400 hover:text-black transition-colors cursor-pointer flex-shrink-0" on:click=move |_| {
                                                        active_thumb.update(|i| { if *i < 4 { *i += 1; } });
                                                    }>
                                                        {icon_chevron_right()}
                                                    </button>
                                                </div>
                                            </div>

                                            // ──── RIGHT: Sticky info panel ─────────────
                                            <div class="lg:sticky lg:top-4 flex flex-col gap-5">

                                                // ── Title ─────────────────────────────
                                                <h1 class="text-2xl font-normal text-gray-900 leading-snug">
                                                    {data.product.name.clone()}
                                                </h1>

                                                // ── Price row ──────────────────────────
                                                <div class="flex items-center gap-3 flex-wrap">
                                                    {move || match matching_variant.get() {
                                                        Some(variant) => {
                                                            let base = format!("{:.0} VND", variant.base_price * rust_decimal::Decimal::from(1000));
                                                            match variant.sale_price {
                                                                Some(sp) => {
                                                                    let sale = format!("{:.0} VND", sp * rust_decimal::Decimal::from(1000));
                                                                    view! {
                                                                        <span class="text-gray-400 line-through text-base">{base}</span>
                                                                        <span class="text-black font-semibold text-xl">{sale}</span>
                                                                        <span class="bg-blue-100 text-blue-700 text-xs font-medium px-2 py-0.5 rounded-sm">"Miễn Phí Ship"</span>
                                                                    }.into_any()
                                                                },
                                                                None => view! {
                                                                    <span class="text-black font-semibold text-xl">{base}</span>
                                                                    <span class="bg-blue-100 text-blue-700 text-xs font-medium px-2 py-0.5 rounded-sm">"Miễn Phí Ship"</span>
                                                                }.into_any(),
                                                            }
                                                        },
                                                        None => view! {
                                                            <span class="text-gray-400 text-sm">"Chọn biến thể để xem giá"</span>
                                                        }.into_any(),
                                                    }}
                                                </div>

                                                // ── Size Selector ─────────────────────
                                                {
                                                    let variants_for_sizes = data.variants.clone();
                                                    let sizes_clone = sizes.clone();
                                                    if !sizes_clone.is_empty() {
                                                        view! {
                                                            <div class="space-y-2.5">
                                                                <p class="text-sm text-gray-700">
                                                                    "Size: "
                                                                    <span class="font-semibold">{move || selected_size.get().unwrap_or_default()}</span>
                                                                </p>
                                                                <div class="flex flex-wrap gap-2">
                                                                    {sizes_clone.into_iter().map(move |s| {
                                                                        let s_sel = s.clone();
                                                                        let s_val = s.clone();
                                                                        let s_view = s.clone();
                                                                        let s_avail = s.clone();
                                                                        let is_selected = move || selected_size.get() == Some(s_sel.clone());
                                                                        let variants = variants_for_sizes.clone();
                                                                        let is_available = move || {
                                                                            let c = selected_color.get();
                                                                            if let Some(c) = c {
                                                                                variants.iter().any(|v| {
                                                                                    let vc = v.attributes.get("color").and_then(|val| val.as_str()).unwrap_or_default();
                                                                                    let vs = v.attributes.get("size").and_then(|val| val.as_str()).unwrap_or_default();
                                                                                    vc == c && vs == s_avail
                                                                                })
                                                                            } else { true }
                                                                        };
                                                                        view! {
                                                                            <button
                                                                                class=move || format!(
                                                                                    "min-w-[44px] h-[44px] px-3 border text-sm font-medium transition-all duration-150 cursor-pointer focus:outline-none rounded-sm {} {}",
                                                                                    if is_selected() { "bg-black border-black text-white" }
                                                                                    else { "bg-white border-gray-300 text-gray-800 hover:border-black" },
                                                                                    if !is_available() { "opacity-30 line-through cursor-not-allowed" } else { "" }
                                                                                )
                                                                                on:click=move |_| selected_size.set(Some(s_val.clone()))
                                                                            >
                                                                                {s_view}
                                                                            </button>
                                                                        }
                                                                    }).collect_view()}
                                                                </div>
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        view! { <div></div> }.into_any()
                                                    }
                                                }

                                                // ── Quantity ───────────────────────────
                                                <div class="space-y-2">
                                                    <p class="text-sm text-gray-700">"Số lượng"</p>
                                                    <div class="inline-flex items-center border border-gray-300 rounded-sm">
                                                        <button
                                                            class="w-10 h-11 flex items-center justify-center text-gray-600 hover:bg-gray-50 transition-colors cursor-pointer text-lg leading-none"
                                                            on:click=move |_| quantity.update(|q| { if *q > 1 { *q -= 1; } })
                                                        >"−"</button>
                                                        <span class="w-10 h-11 flex items-center justify-center text-sm font-medium border-x border-gray-300">{move || quantity.get()}</span>
                                                        <button
                                                            class="w-10 h-11 flex items-center justify-center text-gray-600 hover:bg-gray-50 transition-colors cursor-pointer text-lg leading-none"
                                                            on:click=move |_| quantity.update(|q| *q += 1)
                                                        >"+"</button>
                                                    </div>
                                                </div>

                                                // ── Add to Cart ────────────────────────
                                                {move || {
                                                    let cart = cart.clone();
                                                    match matching_variant.get() {
                                                        Some(variant) => {
                                                            let vid = variant.id;
                                                            let variant_name_for_cart = variant.name.clone();
                                                            let product_name_for_cart = product_name.clone();
                                                            let price = variant.sale_price.unwrap_or(variant.base_price);
                                                            view! {
                                                                <button
                                                                    class="w-full py-4 bg-black text-white text-sm font-semibold tracking-wide hover:bg-gray-900 active:bg-black transition-colors duration-150 cursor-pointer focus:outline-none rounded-sm"
                                                                    on:click={
                                                                        let cart = cart.clone();
                                                                        move |_| {
                                                                            let qty = quantity.get();
                                                                            cart.add_item(CartItem {
                                                                                variant_id: vid,
                                                                                product_name: product_name_for_cart.clone(),
                                                                                variant_name: variant_name_for_cart.clone(),
                                                                                unit_price: (price * rust_decimal::Decimal::from(1000)).to_string().parse::<i64>().unwrap_or(0),
                                                                                quantity: qty,
                                                                            });
                                                                            added_msg.set(true);
                                                                            wasm_bindgen_futures::spawn_local(async move {
                                                                                gloo_timers::future::TimeoutFuture::new(2_500).await;
                                                                                added_msg.set(false);
                                                                            });
                                                                        }
                                                                    }
                                                                >
                                                                    "Thêm vào giỏ hàng"
                                                                </button>
                                                            }.into_any()
                                                        },
                                                        None => view! {
                                                            <button disabled class="w-full py-4 bg-gray-300 text-gray-500 text-sm font-semibold cursor-not-allowed rounded-sm">
                                                                "Hết hàng"
                                                            </button>
                                                        }.into_any(),
                                                    }
                                                }}

                                                // Added confirmation
                                                {move || if added_msg.get() {
                                                    view! {
                                                        <div class="flex items-center gap-2 bg-green-50 border border-green-200 text-green-700 text-sm px-4 py-3 rounded-sm animate-fadeInUp">
                                                            {icon_check_small()}
                                                            "Đã thêm vào giỏ hàng!"
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }}

                                                // ── Color swatches ────────────────────
                                                {
                                                    let variants_for_colors = data.variants.clone();
                                                    let colors_clone = colors.clone();
                                                    if !colors_clone.is_empty() {
                                                        view! {
                                                            <div class="space-y-2.5">
                                                                <p class="text-sm text-gray-700">"Chọn màu khác"</p>
                                                                <div class="flex flex-wrap gap-2.5">
                                                                    {colors_clone.into_iter().map(move |c| {
                                                                        let c_sel = c.clone();
                                                                        let c_val = c.clone();
                                                                        let c_title = c.clone();
                                                                        let c_avail = c.clone();
                                                                        let is_selected = move || selected_color.get() == Some(c_sel.clone());

                                                                        let variants = variants_for_colors.clone();
                                                                        let is_available = move || {
                                                                            let s = selected_size.get();
                                                                            if let Some(s) = s {
                                                                                variants.iter().any(|v| {
                                                                                    let vc = v.attributes.get("color").and_then(|val| val.as_str()).unwrap_or_default();
                                                                                    let vs = v.attributes.get("size").and_then(|val| val.as_str()).unwrap_or_default();
                                                                                    vc == c_avail && vs == s
                                                                                })
                                                                            } else { true }
                                                                        };

                                                                        view! {
                                                                            <button
                                                                                class=move || format!(
                                                                                    "w-12 h-12 rounded-full border-2 overflow-hidden bg-gray-100 transition-all duration-150 cursor-pointer focus:outline-none flex items-center justify-center text-xs text-gray-500 font-medium {} {}",
                                                                                    if is_selected() { "border-black shadow-[0_0_0_2px_white,0_0_0_4px_black]" } else { "border-gray-200 hover:border-gray-400" },
                                                                                    if !is_available() { "opacity-30 cursor-not-allowed" } else { "" }
                                                                                )
                                                                                title=c_title
                                                                                on:click=move |_| selected_color.set(Some(c_val.clone()))
                                                                            >
                                                                                // In a real app this would be a color swatch image; we show first 2 chars
                                                                                {c.chars().take(2).collect::<String>().to_uppercase()}
                                                                            </button>
                                                                        }
                                                                    }).collect_view()}
                                                                </div>
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        view! { <div></div> }.into_any()
                                                    }
                                                }

                                                // ── Warranty / Discount box ───────────
                                                <div class="border border-gray-200 rounded-sm p-4 space-y-3 text-sm">
                                                    <p class="font-bold text-gray-900 text-[13px] uppercase tracking-wide">"BẢO HÀNH 365 NGÀY – HƯ SỬA – LỖI ĐỔI"</p>
                                                    <div class="space-y-1.5">
                                                        <p class="flex items-center gap-2 font-semibold text-gray-800 text-[13px]">
                                                            {icon_gift()}
                                                            "MUA CÀNG NHIỀU – GIẢM CÀNG SÂU"
                                                        </p>
                                                        <ul class="space-y-1 pl-6 text-gray-600">
                                                            <li class="flex items-center gap-2">{icon_check_small()} <span>"Giảm thêm " <strong>"10k"</strong> " cho đơn từ 380k"</span></li>
                                                            <li class="flex items-center gap-2">{icon_check_small()} <span>"Giảm thêm " <strong>"20k"</strong> " cho đơn từ 580k"</span></li>
                                                            <li class="flex items-center gap-2">{icon_check_small()} <span>"Giảm thêm " <strong>"35k"</strong> " cho đơn từ 780k"</span></li>
                                                            <li class="flex items-center gap-2">{icon_check_small()} <span>"Giảm thêm " <strong>"50k"</strong> " cho đơn từ 980k"</span></li>
                                                        </ul>
                                                    </div>
                                                </div>

                                                // ── Accordions ────────────────────────
                                                <div class="border-t border-gray-200 divide-y divide-gray-200">
                                                    <Accordion icon=icon_heart() title="Ưu điểm của phom dáng".to_string()>
                                                        <p>"Vải modal mỏng nhẹ, nhanh khô được giữ nguyên phom dáng rộng rãi, đảm bảo sự thoải mái tuyệt đối, 'cân' mọi hoạt động trong ngày hè nóng bức."</p>
                                                    </Accordion>

                                                    <Accordion icon=icon_help() title="Câu hỏi thường gặp (FAQ)".to_string()>
                                                        <div class="space-y-4">
                                                            <div>
                                                                <p class="font-semibold text-gray-800 mb-1">"1. Có nên đầu tư chiếc áo này không?"</p>
                                                                <p>"Nếu bạn cần áo mặc thường xuyên đi làm hoặc gặp khách, đây là khoản đầu tư đáng giá. Bền – dễ phối – ít nhăn – mặc được quanh năm."</p>
                                                            </div>
                                                            <div>
                                                                <p class="font-semibold text-gray-800 mb-1">"2. Mình đổ mồ hôi nhiều, áo có bị dính lưng không?"</p>
                                                                <p>"Modal 'thở' rất tốt, nên lưng không bị bí. Khi đổ mồ hôi, áo khô nhanh hơn cotton truyền thống."</p>
                                                            </div>
                                                        </div>
                                                    </Accordion>

                                                    <Accordion icon=icon_eye() title="Thông tin sản phẩm".to_string()>
                                                        <ul class="space-y-1.5 list-none">
                                                            <li><strong>"Chất liệu: "</strong>"Modal Fabric – 12% Modal, 88% Polyester"</li>
                                                            <li><strong>"Kỹ thuật: "</strong>"Dáng rộng, cổ bẻ cổ điển, thanh lịch"</li>
                                                            <li><strong>"Phù hợp: "</strong>"Người trẻ yêu phong cách đơn giản, năng động"</li>
                                                            <li><strong>"Xuất xứ: "</strong>"Việt Nam"</li>
                                                        </ul>
                                                    </Accordion>

                                                    <Accordion icon=icon_check_box() title="Đặc điểm nổi bật".to_string()>
                                                        <div class="space-y-4">
                                                            <div class="flex gap-4 items-start">
                                                                <div class="w-20 h-24 flex-shrink-0 bg-gray-100 rounded-sm flex items-center justify-center">
                                                                    <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8 text-gray-300" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><rect x="3" y="3" width="18" height="18" rx="2"></rect><polyline points="21 15 16 10 5 21"></polyline></svg>
                                                                </div>
                                                                <div><p class="font-bold text-gray-800 text-[13px] uppercase mb-1">"FRIENDZONE CỦA LÀN DA"</p><p>"Áo mặc nhẹ tênh, chill cả ngày dài"</p></div>
                                                            </div>
                                                            <div class="flex gap-4 items-start">
                                                                <div class="w-20 h-24 flex-shrink-0 bg-gray-100 rounded-sm flex items-center justify-center">
                                                                    <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8 text-gray-300" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><rect x="3" y="3" width="18" height="18" rx="2"></rect><polyline points="21 15 16 10 5 21"></polyline></svg>
                                                                </div>
                                                                <div><p class="font-bold text-gray-800 text-[13px] uppercase mb-1">"BAY HƠI TRONG 1 NỐT NHẠC"</p><p>"Vải mỏng nhẹ nhanh khô. Giặt tối sáng có đồ mặc cân mọi kèo gấp"</p></div>
                                                            </div>
                                                            <div class="flex gap-4 items-start">
                                                                <div class="w-20 h-24 flex-shrink-0 bg-gray-100 rounded-sm flex items-center justify-center">
                                                                    <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8 text-gray-300" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><rect x="3" y="3" width="18" height="18" rx="2"></rect><polyline points="21 15 16 10 5 21"></polyline></svg>
                                                                </div>
                                                                <div><p class="font-bold text-gray-800 text-[13px] uppercase mb-1">"BẢNG MÀU CÂN MỌI GU"</p><p>"7 màu tha hồ lựa. Cứu tinh cho những ngày lười auto-đẹp trai"</p></div>
                                                            </div>
                                                        </div>
                                                    </Accordion>
                                                </div>

                                                // ── Store locator ─────────────────────
                                                <button class="w-full py-3.5 border border-black text-black text-sm font-semibold hover:bg-gray-50 transition-colors duration-150 cursor-pointer flex items-center justify-center gap-2 rounded-sm">
                                                    {icon_map_pin()}
                                                    "Tìm cửa hàng gần bạn"
                                                </button>

                                            </div>
                                        </div>
                                    }.into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <div class="py-16 text-center space-y-3">
                                            <p class="text-gray-800 font-medium">"Không thể tải sản phẩm"</p>
                                            <p class="text-sm text-gray-500">{e.user_message().to_string()}</p>
                                            <a href="/products" class="inline-block mt-2 text-sm underline text-gray-500 hover:text-black">"Quay lại danh sách"</a>
                                        </div>
                                    }.into_any()
                                }
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}

// ─── Skeleton ──────────────────────────────────────────────────────────────

#[component]
fn ProductDetailSkeleton() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 lg:grid-cols-[minmax(0,1fr)_420px] gap-8 animate-pulse">
            // Gallery
            <div class="flex flex-col gap-3">
                <div class="bg-gray-100 rounded-sm aspect-[3/4]"></div>
                <div class="flex gap-2">
                    {(0..5usize).map(|_| view! { <div class="w-[76px] h-[100px] bg-gray-100 rounded-sm flex-shrink-0"></div> }).collect_view()}
                </div>
            </div>
            // Info
            <div class="space-y-5">
                <div class="h-8 bg-gray-100 rounded w-3/4"></div>
                <div class="h-8 bg-gray-100 rounded w-1/2"></div>
                <div class="h-4 bg-gray-100 rounded w-16"></div>
                <div class="flex gap-2">
                    {(0..4usize).map(|_| view! { <div class="w-11 h-11 bg-gray-100 rounded-sm"></div> }).collect_view()}
                </div>
                <div class="h-4 bg-gray-100 rounded w-20"></div>
                <div class="w-32 h-11 bg-gray-100 rounded-sm"></div>
                <div class="h-14 bg-gray-100 rounded-sm w-full"></div>
                <div class="h-32 bg-gray-100 rounded-sm w-full"></div>
            </div>
        </div>
    }
}
