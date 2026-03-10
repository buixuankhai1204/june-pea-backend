use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto px-4 py-16">
            // Hero
            <div class="text-center">
                <h1 class="text-5xl font-extrabold text-gray-900 tracking-tight">
                    "Welcome to " <span class="text-indigo-600">"Yame Store"</span>
                </h1>
                <p class="mt-6 text-xl text-gray-500 max-w-2xl mx-auto">
                    "Discover our curated collection of premium fashion and accessories."
                </p>
                <div class="mt-10">
                    <a
                        href="/products"
                        class="inline-block px-8 py-3 bg-indigo-600 text-white rounded-lg text-lg font-medium hover:bg-indigo-700 transition-colors"
                    >
                        "Shop Now"
                    </a>
                </div>
            </div>

            // Features
            <div class="mt-24 grid grid-cols-1 md:grid-cols-3 gap-8">
                <div class="text-center p-6">
                    <div class="w-12 h-12 bg-indigo-100 rounded-full flex items-center justify-center mx-auto mb-4">
                        <span class="text-indigo-600 text-xl">"🚚"</span>
                    </div>
                    <h3 class="text-lg font-semibold text-gray-900">"Free Shipping"</h3>
                    <p class="mt-2 text-gray-500">"On orders over $50"</p>
                </div>
                <div class="text-center p-6">
                    <div class="w-12 h-12 bg-indigo-100 rounded-full flex items-center justify-center mx-auto mb-4">
                        <span class="text-indigo-600 text-xl">"🔒"</span>
                    </div>
                    <h3 class="text-lg font-semibold text-gray-900">"Secure Checkout"</h3>
                    <p class="mt-2 text-gray-500">"End-to-end encrypted"</p>
                </div>
                <div class="text-center p-6">
                    <div class="w-12 h-12 bg-indigo-100 rounded-full flex items-center justify-center mx-auto mb-4">
                        <span class="text-indigo-600 text-xl">"↩️"</span>
                    </div>
                    <h3 class="text-lg font-semibold text-gray-900">"Easy Returns"</h3>
                    <p class="mt-2 text-gray-500">"30-day return policy"</p>
                </div>
            </div>
        </div>
    }
}
