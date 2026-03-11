use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::api::client;
use crate::api::types::ProductWithVariants;
use crate::state::cart::{CartItem, CartState};

#[component]
pub fn ProductDetailPage() -> impl IntoView {
    let params = use_params_map();
    let cart = expect_context::<CartState>();
    let added_msg = RwSignal::new(false);
    let selected_color = RwSignal::new(Option::<String>::None);
    let selected_size = RwSignal::new(Option::<String>::None);

    let product = LocalResource::new(move || {
        let slug = params.read().get("slug").unwrap_or_default();
        async move {
            client::get::<ProductWithVariants>(&format!("/api/v1/catalog/products/{}", slug)).await
        }
    });

    view! {
        <div class="max-w-5xl mx-auto px-4 py-8">
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

                            view! {
                                <div class="grid grid-cols-1 md:grid-cols-12 gap-8">
                                    // Left: Image gallery (placeholder)
                                    <div class="md:col-span-7">
                                        <div class="bg-gray-100 rounded-lg aspect-[3/4] flex flex-col items-center justify-center relative overflow-hidden">
                                            <span class="text-gray-400 text-lg">"Product Image"</span>
                                            <div class="absolute bottom-4 left-4 flex gap-2">
                                                <div class="w-16 h-20 bg-white border-2 border-indigo-600 rounded"></div>
                                                <div class="w-16 h-20 bg-white border border-gray-200 rounded"></div>
                                                <div class="w-16 h-20 bg-white border border-gray-200 rounded"></div>
                                            </div>
                                        </div>
                                    </div>

                                    // Right: Details
                                    <div class="md:col-span-5 space-y-6">
                                        <div>
                                            <h1 class="text-2xl font-bold text-gray-900 uppercase tracking-tight">{data.product.name.clone()}</h1>
                                            <p class="mt-2 text-sm text-gray-500">"ID sản phẩm: Non Branded 033"</p>
                                        </div>

                                        // Colors
                                        <div class="mt-6">
                                            <h3 class="text-sm font-semibold text-gray-900 mb-3">"Màu sắc"</h3>
                                            <div class="flex flex-wrap gap-3">
                                                {
                                                    let variants_for_colors = data.variants.clone();
                                                    colors.into_iter().map(move |c| {
                                                    let c_for_selected = c.clone();
                                                    let c_for_val = c.clone();
                                                    let c_for_view = c.clone();
                                                    let c_for_title = c.clone();
                                                    let c_for_avail = c.clone();
                                                    let is_selected = move || selected_color.get() == Some(c_for_selected.clone());

                                                    let variants = variants_for_colors.clone();
                                                    let is_available = move || {
                                                        let s = selected_size.get();
                                                        if let Some(s) = s {
                                                            variants.iter().any(|v| {
                                                                let vc = v.attributes.get("color").and_then(|val| val.as_str()).unwrap_or_default();
                                                                let vs = v.attributes.get("size").and_then(|val| val.as_str()).unwrap_or_default();
                                                                vc == c_for_avail && vs == s
                                                            })
                                                        } else {
                                                            true
                                                        }
                                                    };

                                                    view! {
                                                        <button
                                                            class=move || format!("px-4 py-2 rounded-full border text-sm font-medium focus:outline-none transition-colors {} {}",
                                                                if is_selected() { "border-indigo-600 bg-indigo-50 text-indigo-700 ring-2 ring-indigo-500" } else { "border-gray-300 text-gray-700 bg-white hover:bg-gray-50" },
                                                                if !is_available() { "opacity-40 line-through" } else { "" }
                                                            )
                                                            on:click=move |_| selected_color.set(Some(c_for_val.clone()))
                                                            title=c_for_title
                                                        >
                                                            {c_for_view}
                                                        </button>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>

                                        // Sizes
                                        <div class="mt-6">
                                            <h3 class="text-sm font-semibold text-gray-900 mb-3">"Kích thước"</h3>
                                            <div class="flex gap-3">
                                                {
                                                    let variants_for_sizes = data.variants.clone();
                                                    sizes.into_iter().map(move |s| {
                                                    let s_for_selected = s.clone();
                                                    let s_for_val = s.clone();
                                                    let s_for_view = s.clone();
                                                    let s_for_avail = s.clone();
                                                    let is_selected = move || selected_size.get() == Some(s_for_selected.clone());

                                                    let variants = variants_for_sizes.clone();
                                                    let is_available = move || {
                                                        let c = selected_color.get();
                                                        if let Some(c) = c {
                                                            variants.iter().any(|v| {
                                                                let vc = v.attributes.get("color").and_then(|val| val.as_str()).unwrap_or_default();
                                                                let vs = v.attributes.get("size").and_then(|val| val.as_str()).unwrap_or_default();
                                                                vc == c && vs == s_for_avail
                                                            })
                                                        } else {
                                                            true
                                                        }
                                                    };

                                                    view! {
                                                        <button
                                                            class=move || format!("px-4 py-2 border rounded-md text-sm font-medium focus:outline-none transition-colors {} {}",
                                                                if is_selected() { "border-indigo-600 border-2 text-indigo-600 bg-indigo-50 ring-2 ring-indigo-500" } else { "border-gray-300 text-gray-700 bg-white hover:bg-gray-50" },
                                                                if !is_available() { "opacity-40 line-through" } else { "" }
                                                            )
                                                            on:click=move |_| selected_size.set(Some(s_for_val.clone()))
                                                        >
                                                            {s_for_view}
                                                        </button>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>

                                        // Computed Price and Add to Cart
                                        <div class="mt-8 border-t border-gray-200 pt-6">
                                            {move || match matching_variant.get() {
                                                Some(variant) => {
                                                    let price_display = format!("{}đ", variant.base_price * rust_decimal::Decimal::from(100));
                                                    let sale = variant.sale_price.map(|s| format!("{}đ", s * rust_decimal::Decimal::from(100)));
                                                    let variant_name_for_cart = variant.name.clone();
                                                    let product_name_for_cart = product_name.clone();
                                                    let vid = variant.id;
                                                    let price = variant.sale_price.unwrap_or(variant.base_price);

                                                    view! {
                                                        <div>
                                                            <div class="flex items-center justify-between mb-4">
                                                                <div>
                                                                    <p class="text-sm text-gray-500 mb-1">{format!("SKU: {}", variant.sku)}</p>
                                                                    <div class="flex items-center gap-3">
                                                                        {match &sale {
                                                                            Some(sp) => view! {
                                                                                <span class="text-2xl text-red-600 font-bold">{sp.clone()}</span>
                                                                                <span class="text-gray-400 line-through text-lg">{price_display.clone()}</span>
                                                                            }.into_any(),
                                                                            None => view! {
                                                                                <span class="text-2xl text-indigo-600 font-bold">{price_display.clone()}</span>
                                                                            }.into_any(),
                                                                        }}
                                                                    </div>
                                                                </div>
                                                                <button
                                                                    class="px-6 py-2.5 bg-black text-white rounded-md hover:bg-gray-800 font-medium transition-colors text-sm shadow-sm"
                                                                    on:click={
                                                                        let cart = cart.clone();
                                                                        move |_| {
                                                                            cart.add_item(CartItem {
                                                                                variant_id: vid,
                                                                                product_name: product_name_for_cart.clone(),
                                                                                variant_name: variant_name_for_cart.clone(),
                                                                                unit_price: (price * rust_decimal::Decimal::from(100)).to_string().parse::<i64>().unwrap_or(0),
                                                                                quantity: 1,
                                                                            });
                                                                            added_msg.set(true);
                                                                            wasm_bindgen_futures::spawn_local(async move {
                                                                                gloo_timers::future::TimeoutFuture::new(2_000).await;
                                                                                added_msg.set(false);
                                                                            });
                                                                        }
                                                                    }
                                                                >
                                                                    "Thêm vào giỏ"
                                                                </button>
                                                            </div>
                                                        </div>
                                                    }.into_any()
                                                },
                                                None => {
                                                    view! {
                                                        <div class="p-4 bg-orange-50 border border-orange-200 text-orange-700 rounded-md">
                                                            <p class="font-medium">"Phiên bản này hiện không có sẵn hoặc đã hết hàng."</p>
                                                        </div>
                                                    }.into_any()
                                                }
                                            }}
                                        </div>

                                        {move || {
                                            if added_msg.get() {
                                                view! {
                                                    <div class="mt-4 bg-green-50 border border-green-300 text-green-700 px-4 py-2 rounded flex items-center gap-2">
                                                        <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                                        "Đã thêm vào giỏ hàng!"
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }
                                        }}

                                        // Delivery info panel
                                        <div class="bg-gray-50 border border-gray-200 rounded p-4 text-sm mt-6">
                                            <ul class="space-y-2 text-gray-600">
                                                <li class="flex items-center gap-2">
                                                    <span class="text-indigo-600">"✓"</span> "Miễn phí vận chuyển cho đơn hàng từ 380k"
                                                </li>
                                                <li class="flex items-center gap-2">
                                                    <span class="text-indigo-600">"✓"</span> "Đổi trả miễn phí trong 7 ngày"
                                                </li>
                                            </ul>
                                        </div>

                                    </div>
                                </div>

                                // Detailed Description Section (Yame.vn style)
                                <div class="mt-16 border-t pt-8 space-y-12">
                                    <section>
                                        <h2 class="text-xl font-bold uppercase mb-4">"Ưu điểm của phom dáng"</h2>
                                        <p class="text-gray-700 leading-relaxed">
                                            "Vải modal mỏng nhẹ, nhanh khô được giữ nguyên phom dáng rộng rãi, đảm bảo sự thoải mái tuyệt đối, 'cân' mọi hoạt động trong ngày hè nóng bức."
                                        </p>
                                    </section>

                                    <section>
                                        <h2 class="text-xl font-bold uppercase mb-4">"Câu hỏi thường gặp (FAQ)"</h2>
                                        <div class="space-y-4 text-gray-700 leading-relaxed">
                                            <p><strong>"1. Có nên đầu tư chiếc áo này không, hay chỉ nên mua thử?"</strong><br/>
                                            "Nếu bạn cần áo mặc thường xuyên đi làm hoặc gặp khách, đây là khoản đầu tư đáng giá. Bền – dễ phối – ít nhăn – mặc được quanh năm."</p>
                                            <p><strong>"2. Mình làm việc hay di chuyển nhiều, áo có bị dính mồ hôi lưng không?"</strong><br/>
                                            "Modal 'thở' rất tốt, nên lưng không bị bí. Khi đổ mồ hôi, áo khô nhanh hơn cotton truyền thống."</p>
                                        </div>
                                    </section>

                                    <section>
                                        <h2 class="text-xl font-bold uppercase mb-4">"Thông tin sản phẩm"</h2>
                                        <ul class="list-disc list-inside space-y-2 text-gray-700 leading-relaxed">
                                            <li><strong>"Chất liệu:"</strong> " Modal Fabric, 12% Modal 88% Polyester."</li>
                                            <li><strong>"Kỹ thuật:"</strong> " Dáng rộng tạo cảm giác thoải mái, năng động. Thiết kế cổ bẻ cổ điển, thanh lịch."</li>
                                            <li><strong>"Phù hợp với ai:"</strong> " Phù hợp cho người trẻ yêu phong cách đơn giản, năng động."</li>
                                            <li><strong>"Xuất xứ:"</strong> " Việt Nam"</li>
                                        </ul>
                                    </section>
                                </div>

                                <div class="mt-12 mb-8">
                                    <a href="/products" class="inline-flex items-center text-indigo-600 hover:text-indigo-800 font-medium transition-colors">
                                        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path></svg>
                                        "Tiếp tục mua sắm"
                                    </a>
                                </div>
                            }.into_any()
                        }
                        Err(e) => {
                            view! {
                                <div class="bg-red-50 border border-red-300 text-red-700 px-4 py-3 rounded">
                                    {e.user_message().to_string()}
                                </div>
                            }.into_any()
                        }
                    }
                })}}
            </Suspense>
        </div>
    }
}

#[component]
fn ProductDetailSkeleton() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div class="bg-gray-200 rounded-lg h-96 animate-pulse"></div>
            <div class="space-y-4">
                <div class="h-8 bg-gray-200 rounded animate-pulse w-2/3"></div>
                <div class="h-4 bg-gray-200 rounded animate-pulse w-full"></div>
                <div class="h-4 bg-gray-200 rounded animate-pulse w-5/6"></div>
                <div class="mt-8 space-y-3">
                    <div class="h-20 bg-gray-200 rounded animate-pulse"></div>
                    <div class="h-20 bg-gray-200 rounded animate-pulse"></div>
                </div>
            </div>
        </div>
    }
}
