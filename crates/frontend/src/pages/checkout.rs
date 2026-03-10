use leptos::prelude::*;

use crate::api::client;
use crate::api::types::{NewOrderItem, PlaceOrderRequest, PlaceOrderResponse};
use crate::state::auth::AuthState;
use crate::state::cart::CartState;

#[component]
pub fn CheckoutPage() -> impl IntoView {
    let auth = expect_context::<AuthState>();
    let cart = expect_context::<CartState>();
    let error = RwSignal::new(String::new());
    let loading = RwSignal::new(false);
    let success_order_id = RwSignal::new(Option::<String>::None);

    let on_place_order = move |_: web_sys::MouseEvent| {
        let user_id = match auth.current_user_id() {
            Some(id) => id,
            None => {
                error.set("You must be logged in to place an order.".into());
                return;
            }
        };

        let items = cart.items.get_untracked();
        if items.is_empty() {
            error.set("Cart is empty.".into());
            return;
        }

        let order_items: Vec<NewOrderItem> = items
            .iter()
            .map(|i| NewOrderItem {
                variant_id: i.variant_id,
                quantity: i.quantity,
                unit_price: i.unit_price,
            })
            .collect();

        let req = PlaceOrderRequest {
            customer_id: user_id,
            items: order_items,
        };

        loading.set(true);
        error.set(String::new());
        let cart = cart.clone();

        wasm_bindgen_futures::spawn_local(async move {
            match client::post::<PlaceOrderResponse, _>("/api/v1/ordering/orders", &req).await {
                Ok(resp) => {
                    cart.clear();
                    success_order_id.set(Some(resp.order_id.to_string()));
                }
                Err(e) => {
                    error.set(e.user_message().to_string());
                }
            }
            loading.set(false);
        });
    };

    view! {
        <div class="max-w-2xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Checkout"</h1>

            {move || {
                if let Some(oid) = success_order_id.get() {
                    view! {
                        <div class="bg-green-50 border border-green-300 text-green-700 px-6 py-8 rounded-lg text-center">
                            <h2 class="text-2xl font-bold mb-2">"Order Placed!"</h2>
                            <p>"Order ID: " <span class="font-mono">{oid}</span></p>
                            <a href="/orders" class="mt-4 inline-block text-indigo-600 hover:underline">
                                "View your orders"
                            </a>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            {move || {
                let err = error.get();
                if !err.is_empty() {
                    view! {
                        <div class="mb-4 bg-red-50 border border-red-300 text-red-700 px-4 py-3 rounded">
                            {err}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            {move || {
                let items = cart.items.get();
                let total: i64 = items.iter().map(|i| i.line_total()).sum();

                view! {
                    <div class="bg-white p-6 rounded-lg shadow border space-y-4">
                        <h3 class="text-lg font-semibold">"Order Summary"</h3>
                        {items.into_iter().map(|item| {
                            view! {
                                <div class="flex justify-between text-sm">
                                    <span>{format!("{} × {}", item.product_name, item.quantity)}</span>
                                    <span>{format!("${:.2}", item.line_total() as f64 / 100.0)}</span>
                                </div>
                            }
                        }).collect_view()}
                        <hr class="my-2" />
                        <div class="flex justify-between font-bold text-lg">
                            <span>"Total"</span>
                            <span>{format!("${:.2}", total as f64 / 100.0)}</span>
                        </div>
                    </div>
                }
            }}

            <button
                class="mt-6 w-full py-3 px-4 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 font-medium disabled:opacity-50"
                disabled=move || loading.get()
                on:click=on_place_order
            >
                {move || if loading.get() { "Placing Order..." } else { "Place Order" }}
            </button>
        </div>
    }
}
