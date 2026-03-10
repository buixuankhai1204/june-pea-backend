use leptos::prelude::*;

use crate::api::client;
use crate::api::types::Order;
use crate::state::auth::AuthState;

fn format_cents(cents: i64) -> String {
    format!("${:.2}", cents as f64 / 100.0)
}

#[component]
pub fn OrdersPage() -> impl IntoView {
    let auth = expect_context::<AuthState>();

    let orders = LocalResource::new(move || {
        let user_id = auth.current_user_id();
        async move {
            match user_id {
                Some(id) => {
                    client::get::<Vec<Order>>(&format!("/api/v1/ordering/orders/customer/{}", id))
                        .await
                }
                None => Err(crate::api::client::ApiError::Unauthorized(
                    "Not logged in".into(),
                )),
            }
        }
    });

    let cancel_error = RwSignal::new(String::new());

    view! {
        <div class="max-w-4xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"My Orders"</h1>

            {move || {
                let err = cancel_error.get();
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

            <Suspense fallback=move || view! { <OrdersSkeleton /> }>
                {move || Suspend::new(async move {
                    match orders.await {
                        Ok(data) => {
                            if data.is_empty() {
                                view! {
                                    <div class="text-center py-16">
                                        <p class="text-gray-500 text-lg">"No orders yet"</p>
                                        <a href="/products" class="mt-4 inline-block text-indigo-600 hover:underline">
                                            "Start Shopping"
                                        </a>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-4">
                                        {data.into_iter().map(|order| {
                                            let status_class = match order.status {
                                                crate::api::types::OrderStatus::Pending => "bg-yellow-100 text-yellow-800",
                                                crate::api::types::OrderStatus::Completed => "bg-green-100 text-green-800",
                                                crate::api::types::OrderStatus::Cancelled => "bg-red-100 text-red-800",
                                            };
                                            let is_pending = order.status == crate::api::types::OrderStatus::Pending;
                                            let oid = order.id;

                                            view! {
                                                <div class="bg-white p-6 rounded-lg shadow border">
                                                    <div class="flex justify-between items-start">
                                                        <div>
                                                            <p class="text-sm text-gray-500">"Order ID"</p>
                                                            <p class="font-mono text-sm">{order.id.to_string()}</p>
                                                        </div>
                                                        <span class=format!("px-3 py-1 rounded-full text-sm font-medium {}", status_class)>
                                                            {order.status.to_string()}
                                                        </span>
                                                    </div>
                                                    <div class="mt-4 flex justify-between items-center">
                                                        <div>
                                                            <p class="text-sm text-gray-500">"Total"</p>
                                                            <p class="text-xl font-bold">{format_cents(order.total)}</p>
                                                        </div>
                                                        <div class="text-sm text-gray-500">
                                                            {order.created_at.format("%b %d, %Y %H:%M").to_string()}
                                                        </div>
                                                    </div>
                                                    {if is_pending {
                                                        view! {
                                                            <button
                                                                class="mt-4 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 text-sm"
                                                                on:click=move |_| {
                                                                    let cancel_error = cancel_error.clone();
                                                                    wasm_bindgen_futures::spawn_local(async move {
                                                                        match client::delete::<bool>(&format!("/api/v1/ordering/orders/{}", oid)).await {
                                                                            Ok(_) => {
                                                                                // Reload via navigation
                                                                                let _ = web_sys::window().and_then(|w| w.location().reload().ok());
                                                                            }
                                                                            Err(e) => cancel_error.set(e.user_message().to_string()),
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                "Cancel Order"
                                                            </button>
                                                        }.into_any()
                                                    } else {
                                                        view! { <div></div> }.into_any()
                                                    }}
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        Err(e) => {
                            view! {
                                <div class="bg-red-50 border border-red-300 text-red-700 px-4 py-3 rounded">
                                    {e.user_message().to_string()}
                                </div>
                            }.into_any()
                        }
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn OrdersSkeleton() -> impl IntoView {
    view! {
        <div class="space-y-4">
            {(0..3).map(|_| view! {
                <div class="bg-white p-6 rounded-lg shadow border">
                    <div class="h-4 bg-gray-200 rounded animate-pulse w-1/3 mb-4"></div>
                    <div class="h-6 bg-gray-200 rounded animate-pulse w-1/4"></div>
                </div>
            }).collect_view()}
        </div>
    }
}
