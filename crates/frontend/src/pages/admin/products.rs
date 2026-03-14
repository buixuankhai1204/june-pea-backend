use leptos::prelude::*;
use crate::api::client::catalog as catalog_api;
use crate::api::types::CreateProductRequest;
use uuid::Uuid;

fn icon_plus() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
    }
}

fn icon_trash() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
        </svg>
    }
}

#[component]
pub fn AdminProductsPage() -> impl IntoView {
    let (show_modal, set_show_modal) = signal(false);
    let (name, set_name) = signal("".to_string());
    let (category_id, set_category_id) = signal("".to_string());
    
    let products_resource: LocalResource<Vec<crate::api::types::Product>> = LocalResource::new(move || {
        async move { catalog_api::list_products(1, 100).await.map(|p| p.items).unwrap_or_default() }
    });

    let categories_resource: LocalResource<Vec<crate::api::types::Category>> = LocalResource::new(move || {
        async move { catalog_api::list_categories().await.unwrap_or_default() }
    });

    let create_action = Action::new_local(|req: &CreateProductRequest| {
        let req = req.clone();
        async move { catalog_api::create_product(req).await }
    });

    let delete_action = Action::new_local(|id: &Uuid| {
        let id = *id;
        async move { catalog_api::delete_product(id).await }
    });

    let on_submit = move |ev: leptos::web_sys::SubmitEvent| {
        ev.prevent_default();
        if let Ok(c_id) = Uuid::parse_str(&category_id.get()) {
            create_action.dispatch(CreateProductRequest {
                name: name.get(),
                slug: None,
                description: None,
                category_id: c_id,
            });
            set_show_modal.set(false);
            products_resource.refetch();
        }
    };

    view! {
        <div class="p-6 space-y-6 animate-in fade-in duration-500">
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-2xl font-black text-gray-900">"Products"</h1>
                    <p class="text-sm text-gray-400 mt-1">"Manage your product catalog and variants"</p>
                </div>
                <button 
                    class="bg-[#FCE300] hover:bg-yellow-400 text-gray-900 font-bold px-4 py-2.5 rounded-xl transition-all shadow-lg shadow-yellow-200/50 flex items-center gap-2 cursor-pointer"
                    on:click=move |_| set_show_modal.set(true)
                >
                    {icon_plus()} "New Product"
                </button>
            </div>

            // Products Table
            <div class="bg-white rounded-3xl border border-gray-100 shadow-xl overflow-hidden">
                <table class="w-full text-left border-collapse">
                    <thead>
                        <tr class="border-b border-gray-50 text-[10px] font-black text-gray-400 uppercase tracking-widest">
                            <th class="px-6 py-4">"Product"</th>
                            <th class="px-6 py-4">"Slug"</th>
                            <th class="px-6 py-4">"Category"</th>
                            <th class="px-6 py-4 text-right">"Actions"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-gray-50">
                        <Suspense fallback=|| view! { <tr><td colspan="4" class="p-10 text-center text-gray-400">"Loading products..."</td></tr> }>
                            {move || products_resource.get().map(|list| {
                                list.iter().map(|p| {
                                    let p_id = p.id;
                                    view! {
                                        <tr class="hover:bg-gray-50/50 transition-colors group">
                                            <td class="px-6 py-4 font-bold text-gray-900">{p.name.clone()}</td>
                                            <td class="px-6 py-4 text-xs font-mono text-gray-400">{p.slug.clone()}</td>
                                            <td class="px-6 py-4 text-sm text-gray-500">{p.category_id.to_string()}</td>
                                            <td class="px-6 py-4 text-right">
                                                <div class="flex justify-end gap-3">
                                                    <button class="text-xs font-bold text-indigo-600 hover:text-indigo-800 transition-colors cursor-pointer">"Edit"</button>
                                                    <button 
                                                        class="text-xs font-bold text-red-600 hover:text-red-800 transition-colors cursor-pointer"
                                                        on:click=move |_| {
                                                            delete_action.dispatch(p_id);
                                                            products_resource.refetch();
                                                        }
                                                    >
                                                        {icon_trash()}
                                                    </button>
                                                </div>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()
                            })}
                        </Suspense>
                    </tbody>
                </table>
            </div>

            // Create Modal
            {move || show_modal.get().then(|| view! {
                <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/20 backdrop-blur-sm animate-in fade-in duration-200">
                    <div class="bg-white rounded-3xl shadow-2xl w-full max-w-md overflow-hidden animate-in zoom-in-95 duration-200">
                        <div class="p-6 border-b border-gray-50">
                            <h3 class="text-xl font-black text-gray-900">"Add New Product"</h3>
                        </div>
                        <form on:submit=on_submit class="p-6 space-y-4">
                            <div class="space-y-1">
                                <label class="text-[10px] font-black text-gray-400 uppercase tracking-widest px-1">"Product Name"</label>
                                <input 
                                    type="text" 
                                    placeholder="e.g. Áo Thun Modal" 
                                    required
                                    class="w-full px-4 py-3 rounded-2xl bg-gray-50 border border-gray-100 focus:bg-white focus:ring-2 focus:ring-[#FCE300] outline-none transition-all text-sm"
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                />
                            </div>
                            <div class="space-y-1">
                                <label class="text-[10px] font-black text-gray-400 uppercase tracking-widest px-1">"Category"</label>
                                <select 
                                    required
                                    class="w-full px-4 py-3 rounded-2xl bg-gray-50 border border-gray-100 focus:bg-white focus:ring-2 focus:ring-[#FCE300] outline-none transition-all text-sm"
                                    on:change=move |ev| set_category_id.set(event_target_value(&ev))
                                >
                                    <option value="">"Select Category"</option>
                                    <Suspense>
                                        {move || categories_resource.get().map(|list| {
                                            list.iter().map(|c| view! {
                                                <option value=c.id.to_string()>{c.name.clone()}</option>
                                            }).collect_view()
                                        })}
                                    </Suspense>
                                </select>
                            </div>
                            <div class="flex gap-3 pt-4">
                                <button 
                                    type="button"
                                    class="flex-1 px-6 py-3 rounded-2xl font-bold text-gray-500 hover:bg-gray-50 transition-all cursor-pointer"
                                    on:click=move |_| set_show_modal.set(false)
                                >
                                    "Cancel"
                                </button>
                                <button 
                                    type="submit"
                                    class="flex-1 px-6 py-3 rounded-2xl bg-[#FCE300] hover:bg-yellow-400 text-gray-900 font-bold transition-all shadow-lg shadow-yellow-200/50 cursor-pointer"
                                >
                                    "Create"
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            })}
        </div>
    }
}
