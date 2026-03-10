use leptos::prelude::*;

use crate::state::cart::CartState;

fn format_cents(cents: i64) -> String {
    format!("${:.2}", cents as f64 / 100.0)
}

#[component]
pub fn CartPage() -> impl IntoView {
    let cart = expect_context::<CartState>();

    view! {
        <div class="max-w-3xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Shopping Cart"</h1>

            {move || {
                let items = cart.items.get();
                if items.is_empty() {
                    view! {
                        <div class="text-center py-16">
                            <p class="text-gray-500 text-lg">"Your cart is empty"</p>
                            <a href="/products" class="mt-4 inline-block text-indigo-600 hover:underline">
                                "Browse Products"
                            </a>
                        </div>
                    }.into_any()
                } else {
                    let total: i64 = items.iter().map(|i| i.line_total()).sum();
                    view! {
                        <div class="space-y-4">
                            {items.into_iter().map(|item| {
                                let vid = item.variant_id;
                                let qty = item.quantity;
                                let cart_inc = cart.clone();
                                let cart_dec = cart.clone();
                                let cart_rm = cart.clone();

                                view! {
                                    <div class="flex items-center justify-between p-4 bg-white rounded-lg shadow border">
                                        <div class="flex-1">
                                            <p class="font-semibold text-gray-900">{item.product_name.clone()}</p>
                                            <p class="text-sm text-gray-500">{item.variant_name.clone()}</p>
                                            <p class="text-sm text-gray-600">{format_cents(item.unit_price)} " each"</p>
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <button
                                                class="w-8 h-8 flex items-center justify-center bg-gray-200 rounded hover:bg-gray-300"
                                                on:click=move |_| cart_dec.update_quantity(vid, qty - 1)
                                            >"-"</button>
                                            <span class="w-8 text-center font-medium">{qty}</span>
                                            <button
                                                class="w-8 h-8 flex items-center justify-center bg-gray-200 rounded hover:bg-gray-300"
                                                on:click=move |_| cart_inc.update_quantity(vid, qty + 1)
                                            >"+"</button>
                                        </div>
                                        <div class="ml-4 text-right">
                                            <p class="font-bold text-gray-900">{format_cents(item.line_total())}</p>
                                            <button
                                                class="text-red-500 text-sm hover:underline"
                                                on:click=move |_| cart_rm.remove_item(vid)
                                            >"Remove"</button>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>

                        <div class="mt-8 bg-white p-6 rounded-lg shadow border">
                            <div class="flex justify-between text-xl font-bold">
                                <span>"Total"</span>
                                <span>{format_cents(total)}</span>
                            </div>
                            <a
                                href="/checkout"
                                class="mt-4 block w-full text-center py-3 px-4 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 font-medium"
                            >
                                "Proceed to Checkout"
                            </a>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
