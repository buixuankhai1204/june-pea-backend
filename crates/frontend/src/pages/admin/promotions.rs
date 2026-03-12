use leptos::prelude::*;

fn icon_plus() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
    }
}

struct Promo {
    code: &'static str,
    discount: &'static str,
    kind: &'static str,
    used: u32,
    limit: u32,
    expiry: &'static str,
    status: &'static str,
}

#[component]
pub fn AdminPromotionsPage() -> impl IntoView {
    let promos = vec![
        Promo { code: "SUMMER20",   discount: "20%",   kind: "Percentage", used: 248, limit: 500, expiry: "31/03/2026", status: "Active" },
        Promo { code: "FREESHIP",   discount: "Free ship", kind: "Shipping", used: 1024, limit: 2000, expiry: "15/03/2026", status: "Active" },
        Promo { code: "NEWUSER50",  discount: "₫50k",  kind: "Fixed",      used: 392, limit: 400, expiry: "30/04/2026", status: "Active" },
        Promo { code: "FLASH15",    discount: "15%",   kind: "Percentage", used: 500, limit: 500, expiry: "10/03/2026", status: "Expired" },
        Promo { code: "VIP30",      discount: "30%",   kind: "Percentage", used: 0,   limit: 200, expiry: "30/06/2026", status: "Scheduled" },
        Promo { code: "LOYALTY10",  discount: "10%",   kind: "Percentage", used: 768, limit: 1000, expiry: "31/12/2026", status: "Active" },
    ];

    view! {
        <div class="p-6 space-y-6">
            // Header
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-xl font-black text-gray-900">"Promotions"</h1>
                    <p class="text-xs text-gray-400 mt-0.5">"Create and manage discount codes and campaigns"</p>
                </div>
                <button class="flex items-center gap-1.5 bg-[#FCE300] hover:bg-yellow-400 text-gray-900 text-xs font-bold px-4 py-2.5 rounded-xl transition-colors cursor-pointer shadow-sm">
                    {icon_plus()} "New Promotion"
                </button>
            </div>

            // KPI cards
            <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
                {[
                    ("Active Promos", "4",    "#10B981", "bg-emerald-50 border-emerald-100"),
                    ("Total Used",   "2,932", "#6366F1", "bg-indigo-50 border-indigo-100"),
                    ("Expired",      "1",     "#94A3B8", "bg-gray-50 border-gray-200"),
                    ("Scheduled",    "1",     "#F59E0B", "bg-amber-50 border-amber-100"),
                ].iter().map(|&(label, val, color, bg)| view! {
                    <div class=format!("rounded-2xl p-4 border shadow-sm {}", bg)>
                        <p class="text-xs text-gray-500 font-medium">{label}</p>
                        <p class="text-2xl font-black mt-1" style={format!("color:{}", color)}>{val}</p>
                    </div>
                }).collect_view()}
            </div>

            // Promo cards grid
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {promos.into_iter().map(|p| {
                    let fill_pct = if p.limit > 0 { p.used as f64 / p.limit as f64 * 100.0 } else { 0.0 };
                    let (badge_bg, bar_color) = match p.status {
                        "Active"    => ("bg-emerald-50 text-emerald-600 border-emerald-100", "#10B981"),
                        "Expired"   => ("bg-gray-100 text-gray-400 border-gray-200",          "#94A3B8"),
                        "Scheduled" => ("bg-amber-50 text-amber-600 border-amber-100",        "#F59E0B"),
                        _           => ("bg-gray-100 text-gray-400 border-gray-200",           "#94A3B8"),
                    };
                    view! {
                        <div class="bg-white rounded-2xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow cursor-pointer">
                            <div class="flex items-start justify-between mb-3">
                                <div>
                                    <p class="font-black text-gray-900 text-base font-mono">{p.code}</p>
                                    <p class="text-xs text-gray-400 mt-0.5">{p.kind}</p>
                                </div>
                                <span class=format!("text-[10px] font-semibold px-2.5 py-1 rounded-full border flex-shrink-0 {}", badge_bg)>
                                    {p.status}
                                </span>
                            </div>

                            // Discount highlight
                            <div class="bg-gray-50 rounded-xl p-3 text-center mb-3">
                                <span class="text-2xl font-black text-[#FCE300]" style="text-shadow: 0 0 1px #0003;">{p.discount}</span>
                                <span class="text-xs text-gray-500 block">"Discount"</span>
                            </div>

                            // Usage bar
                            <div class="space-y-1.5">
                                <div class="flex items-center justify-between text-[11px] text-gray-500">
                                    <span>"Usage: " {p.used} "/" {p.limit}</span>
                                    <span class="font-semibold text-gray-900">{format!("{:.0}%", fill_pct)}</span>
                                </div>
                                <div class="h-1.5 bg-gray-100 rounded-full overflow-hidden">
                                    <div class="h-full rounded-full transition-all duration-500"
                                        style={format!("width:{}%; background:{}", fill_pct, bar_color)}>
                                    </div>
                                </div>
                            </div>

                            <p class="text-[11px] text-gray-400 mt-3">"Expires: " <span class="text-gray-700 font-semibold">{p.expiry}</span></p>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
