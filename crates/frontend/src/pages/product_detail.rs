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
                            view! {
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                                    // Image placeholder
                                    <div class="bg-gray-200 rounded-lg h-96 flex items-center justify-center">
                                        <span class="text-gray-400">"Product Image"</span>
                                    </div>

                                    // Details
                                    <div>
                                        <h1 class="text-3xl font-bold text-gray-900">{data.product.name.clone()}</h1>
                                        <p class="mt-4 text-gray-600">
                                            {data.product.description.clone().unwrap_or_else(|| "No description available.".into())}
                                        </p>

                                        // Variants
                                        <div class="mt-8 space-y-4">
                                            <h3 class="text-lg font-semibold text-gray-800">"Available Variants"</h3>
                                            {data.variants.into_iter().map(|variant| {
                                                let price_display = format!("${:.2}", variant.base_price);
                                                let sale = variant.sale_price.map(|s| format!("${:.2}", s));
                                                let variant_name_for_cart = variant.name.clone();
                                                let product_name_for_cart = product_name.clone();
                                                let vid = variant.id;

                                                view! {
                                                    <div class="flex items-center justify-between p-4 bg-gray-50 rounded-lg border">
                                                        <div>
                                                            <p class="font-medium text-gray-900">{variant.name.clone()}</p>
                                                            <p class="text-sm text-gray-500">{format!("SKU: {}", variant.sku)}</p>
                                                            <div class="flex items-center gap-2 mt-1">
                                                                {match &sale {
                                                                    Some(sp) => view! {
                                                                        <span class="text-red-600 font-bold">{sp.clone()}</span>
                                                                        <span class="text-gray-400 line-through text-sm">{price_display.clone()}</span>
                                                                    }.into_any(),
                                                                    None => view! {
                                                                        <span class="text-indigo-600 font-bold">{price_display.clone()}</span>
                                                                    }.into_any(),
                                                                }}
                                                            </div>
                                                        </div>
                                                        <button
                                                            class="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 text-sm"
                                                            on:click={
                                                                let cart = cart.clone();
                                                                let vn = variant_name_for_cart.clone();
                                                                let pn = product_name_for_cart.clone();
                                                                let price = variant.sale_price.unwrap_or(variant.base_price);
                                                                move |_| {
                                                                    cart.add_item(CartItem {
                                                                        variant_id: vid,
                                                                        product_name: pn.clone(),
                                                                        variant_name: vn.clone(),
                                                                        unit_price: (price * rust_decimal::Decimal::from(100)).to_string().parse::<i64>().unwrap_or(0),
                                                                        quantity: 1,
                                                                    });
                                                                    added_msg.set(true);
                                                                    // Reset message after 2s
                                                                    wasm_bindgen_futures::spawn_local(async move {
                                                                        gloo_timers::future::TimeoutFuture::new(2_000).await;
                                                                        added_msg.set(false);
                                                                    });
                                                                }
                                                            }
                                                        >
                                                            "Add to Cart"
                                                        </button>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>

                                        {move || {
                                            if added_msg.get() {
                                                view! {
                                                    <div class="mt-4 bg-green-50 border border-green-300 text-green-700 px-4 py-2 rounded">
                                                        "Added to cart!"
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }
                                        }}
                                    </div>
                                </div>

                                <a href="/products" class="inline-block mt-8 text-indigo-600 hover:underline">
                                    "← Back to Products"
                                </a>
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
