use leptos::prelude::*;

use crate::api::client;
use crate::api::types::PaginatedProducts;

#[component]
pub fn ProductsPage() -> impl IntoView {
    let page = RwSignal::new(1i64);
    let page_size = 12i64;

    let products = LocalResource::new(move || {
        let p = page.get();
        async move {
            client::get::<PaginatedProducts>(&format!(
                "/api/v1/catalog/products?page={}&page_size={}",
                p, 12
            ))
            .await
        }
    });

    view! {
        <div class="max-w-7xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Products"</h1>

            <Suspense fallback=move || view! { <ProductGridSkeleton /> }>
                {move || Suspend::new(async move {
                    match products.await {
                        Ok(data) => {
                            let total_pages = (data.total + page_size - 1) / page_size;
                            view! {
                                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                                    {data.items.into_iter().map(|product| {
                                        let slug = product.slug.clone();
                                        view! {
                                            <a
                                                href=format!("/products/{}", slug)
                                                class="group bg-white rounded-lg shadow-md overflow-hidden hover:shadow-xl transition-shadow duration-300"
                                            >
                                                <div class="h-48 bg-gray-200 flex items-center justify-center">
                                                    <span class="text-gray-400 text-sm">"Product Image"</span>
                                                </div>
                                                <div class="p-4">
                                                    <h3 class="text-lg font-semibold text-gray-900 group-hover:text-indigo-600 truncate">
                                                        {product.name.clone()}
                                                    </h3>
                                                    <p class="text-sm text-gray-500 mt-1 line-clamp-2">
                                                        {product.description.clone().unwrap_or_default()}
                                                    </p>
                                                </div>
                                            </a>
                                        }
                                    }).collect_view()}
                                </div>

                                // Pagination
                                <div class="mt-8 flex justify-center gap-2">
                                    <button
                                        class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 disabled:opacity-50"
                                        disabled=move || page.get() <= 1
                                        on:click=move |_| page.update(|p| *p -= 1)
                                    >
                                        "Previous"
                                    </button>
                                    <span class="px-4 py-2 text-gray-700">
                                        {move || format!("Page {} of {}", page.get(), total_pages.max(1))}
                                    </span>
                                    <button
                                        class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 disabled:opacity-50"
                                        disabled=move || page.get() >= total_pages
                                        onclick=move |_| page.update(|p| *p += 1)
                                    >
                                        "Next"
                                    </button>
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
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn ProductGridSkeleton() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
            {(0..8).map(|_| {
                view! {
                    <div class="bg-white rounded-lg shadow-md overflow-hidden">
                        <div class="h-48 bg-gray-200 animate-pulse"></div>
                        <div class="p-4 space-y-3">
                            <div class="h-5 bg-gray-200 rounded animate-pulse w-3/4"></div>
                            <div class="h-4 bg-gray-200 rounded animate-pulse w-full"></div>
                        </div>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
