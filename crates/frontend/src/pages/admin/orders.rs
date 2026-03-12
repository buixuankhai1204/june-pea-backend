use leptos::prelude::*;

fn icon_search() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
    }
}

fn icon_plus() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
    }
}

fn icon_filter() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"></polygon>
        </svg>
    }
}

struct AdminOrder {
    id: &'static str,
    customer: &'static str,
    product: &'static str,
    date: &'static str,
    amount: &'static str,
    status: &'static str,
}

#[component]
pub fn AdminOrdersPage() -> impl IntoView {
    let active_tab = RwSignal::new("All");

    let orders = vec![
        AdminOrder { id: "#ORD-9041", customer: "Nguyễn Văn An",   product: "Áo Thun Modal × 2",    date: "12/03/2026", amount: "₫378k", status: "Delivered" },
        AdminOrder { id: "#ORD-9040", customer: "Trần Thị Bình",   product: "Áo Sơ Mi Non-Iron × 1", date: "12/03/2026", amount: "₫249k", status: "Shipping" },
        AdminOrder { id: "#ORD-9039", customer: "Lê Hoàng Cường",  product: "Áo Khoác Worker × 1",   date: "11/03/2026", amount: "₫589k", status: "Processing" },
        AdminOrder { id: "#ORD-9038", customer: "Phạm Thu Dung",   product: "Áo Polo Raglan × 3",    date: "11/03/2026", amount: "₫657k", status: "Delivered" },
        AdminOrder { id: "#ORD-9037", customer: "Hoàng Minh Đức",  product: "Áo Thun Boxy × 2",      date: "10/03/2026", amount: "₫438k", status: "Cancelled" },
        AdminOrder { id: "#ORD-9036", customer: "Vũ Thị Hoa",      product: "Áo Sơ Mi Modal × 1",    date: "10/03/2026", amount: "₫219k", status: "Delivered" },
        AdminOrder { id: "#ORD-9035", customer: "Đặng Quốc Khánh", product: "Áo Thun Modal × 4",    date: "09/03/2026", amount: "₫756k", status: "Pending" },
        AdminOrder { id: "#ORD-9034", customer: "Bùi Thị Lan",     product: "Áo Polo Flex × 1",     date: "09/03/2026", amount: "₫219k", status: "Shipping" },
    ];

    let tabs = ["All", "Pending", "Processing", "Shipping", "Delivered", "Cancelled"];

    let stat_cards = [
        ("Total Orders", "3,247", "#6366F1", "bg-indigo-50"),
        ("Pending",      "128",   "#F59E0B", "bg-amber-50"),
        ("Shipping",     "456",   "#3B82F6", "bg-blue-50"),
        ("Delivered",    "2,481", "#10B981", "bg-emerald-50"),
        ("Cancelled",    "182",   "#F43F5E", "bg-rose-50"),
    ];

    view! {
        <div class="p-6 space-y-6">
            // Header
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-xl font-black text-gray-900">"Orders"</h1>
                    <p class="text-xs text-gray-400 mt-0.5">"Manage and track all customer orders"</p>
                </div>
                <button class="flex items-center gap-1.5 bg-[#FCE300] hover:bg-yellow-400 text-gray-900 text-xs font-bold px-4 py-2.5 rounded-xl transition-colors cursor-pointer shadow-sm">
                    {icon_plus()} "New Order"
                </button>
            </div>

            // Stat cards
            <div class="grid grid-cols-2 sm:grid-cols-5 gap-3">
                {stat_cards.iter().map(|&(label, val, color, bg)| view! {
                    <div class=format!("rounded-2xl p-4 border border-gray-100 shadow-sm {}", bg)>
                        <p class="text-xs text-gray-500 font-medium">{label}</p>
                        <p class="text-2xl font-black mt-1" style={format!("color:{}", color)}>{val}</p>
                    </div>
                }).collect_view()}
            </div>

            // Table card
            <div class="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
                // Toolbar
                <div class="flex items-center justify-between px-5 py-4 border-b border-gray-100 flex-wrap gap-3">
                    // Tabs
                    <div class="flex items-center gap-1">
                        {tabs.iter().map(|&t| {
                            let t_str = t;
                            view! {
                                <button
                                    class=move || format!("px-3 py-1.5 rounded-lg text-xs font-semibold transition-all duration-150 cursor-pointer {}",
                                        if active_tab.get() == t_str { "bg-[#FCE300] text-gray-900" } else { "text-gray-500 hover:text-gray-800 hover:bg-gray-50" })
                                    on:click=move |_| active_tab.set(t_str)
                                >{t}</button>
                            }
                        }).collect_view()}
                    </div>
                    // Search + filter
                    <div class="flex items-center gap-2">
                        <div class="flex items-center gap-2 bg-gray-50 border border-gray-200 rounded-xl px-3 py-2 w-44">
                            <span class="text-gray-400">{icon_search()}</span>
                            <input type="text" placeholder="Search orders..." class="bg-transparent text-xs text-gray-700 outline-none w-full"/>
                        </div>
                        <button class="flex items-center gap-1.5 border border-gray-200 text-gray-500 rounded-xl px-3 py-2 text-xs font-medium hover:bg-gray-50 transition-colors cursor-pointer">
                            {icon_filter()} "Filter"
                        </button>
                    </div>
                </div>

                // Table
                <div class="overflow-x-auto">
                    <table class="w-full text-xs">
                        <thead>
                            <tr class="border-b border-gray-100 text-gray-400 uppercase tracking-wider text-[10px]">
                                <th class="px-5 py-3 text-left font-medium">"Order ID"</th>
                                <th class="px-4 py-3 text-left font-medium">"Customer"</th>
                                <th class="px-4 py-3 text-left font-medium hidden md:table-cell">"Product"</th>
                                <th class="px-4 py-3 text-left font-medium hidden sm:table-cell">"Date"</th>
                                <th class="px-4 py-3 text-right font-medium">"Amount"</th>
                                <th class="px-4 py-3 text-center font-medium">"Status"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-gray-50">
                            {orders.into_iter().map(|o| {
                                let (bg, text) = match o.status {
                                    "Delivered"  => ("bg-emerald-50 text-emerald-600 border-emerald-100", "●  Delivered"),
                                    "Shipping"   => ("bg-blue-50 text-blue-600 border-blue-100",           "●  Shipping"),
                                    "Processing" => ("bg-violet-50 text-violet-600 border-violet-100",     "●  Processing"),
                                    "Cancelled"  => ("bg-rose-50 text-rose-600 border-rose-100",           "●  Cancelled"),
                                    _            => ("bg-amber-50 text-amber-600 border-amber-100",        "●  Pending"),
                                };
                                view! {
                                    <tr class="hover:bg-gray-50 transition-colors duration-100">
                                        <td class="px-5 py-3.5 font-mono text-indigo-600 font-semibold whitespace-nowrap">{o.id}</td>
                                        <td class="px-4 py-3.5 text-gray-800 font-medium whitespace-nowrap">{o.customer}</td>
                                        <td class="px-4 py-3.5 text-gray-500 hidden md:table-cell truncate max-w-[180px]">{o.product}</td>
                                        <td class="px-4 py-3.5 text-gray-400 hidden sm:table-cell whitespace-nowrap">{o.date}</td>
                                        <td class="px-4 py-3.5 text-right text-gray-900 font-bold whitespace-nowrap">{o.amount}</td>
                                        <td class="px-4 py-3.5 text-center">
                                            <span class=format!("text-[10px] font-semibold px-2.5 py-1 rounded-full border {}", bg)>
                                                {bg; text}
                                            </span>
                                        </td>
                                    </tr>
                                }
                            }).collect_view()}
                        </tbody>
                    </table>
                </div>

                // Pagination footer
                <div class="flex items-center justify-between px-5 py-3.5 border-t border-gray-100 text-xs text-gray-500">
                    <span>"Showing 1–8 of 3,247 orders"</span>
                    <div class="flex items-center gap-1">
                        {[1, 2, 3, 4, 5].iter().map(|&n| view! {
                            <button class=format!("w-7 h-7 rounded-lg text-xs font-semibold cursor-pointer transition-colors {}",
                                if n == 1 { "bg-[#FCE300] text-gray-900" } else { "text-gray-500 hover:bg-gray-100" })>
                                {n}
                            </button>
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </div>
    }
}
