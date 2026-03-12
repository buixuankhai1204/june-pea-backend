use leptos::prelude::*;

fn icon_plus() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
    }
}

fn gauge_small(pct: f64, color: &str) -> impl IntoView {
    let r = 28.0_f64;
    let circ = std::f64::consts::PI * r;
    let dash = pct / 100.0 * circ;
    let gap = circ - dash;
    let color = color.to_string();
    view! {
        <div class="relative" style="width:72px;height:40px;overflow:hidden;">
            <svg viewBox="0 0 72 40" style="width:72px;height:40px;">
                <path d="M 8 36 A 28 28 0 0 1 64 36" fill="none" stroke="#F3F4F6" stroke-width="8" stroke-linecap="round"/>
                <path d="M 8 36 A 28 28 0 0 1 64 36" fill="none" stroke={color} stroke-width="8" stroke-linecap="round"
                    stroke-dasharray={format!("{:.2} {:.2}", dash, gap)}/>
            </svg>
            <div class="absolute inset-x-0 bottom-0 text-center">
                <span class="text-xs font-black text-gray-900">{format!("{:.0}%", pct)}</span>
            </div>
        </div>
    }
}

struct StaffMember {
    name: &'static str,
    role: &'static str,
    dept: &'static str,
    checkin: &'static str,
    target: u32,
    achieved: u32,
    status: &'static str,
}

#[component]
pub fn AdminStaffPage() -> impl IntoView {
    let staff = vec![
        StaffMember { name: "Nguyễn Văn An",   role: "Cashier",              dept: "Operations", checkin: "08:32", target: 1920, achieved: 1758, status: "Active" },
        StaffMember { name: "Trần Thị Bình",   role: "Inventory Manager",    dept: "Warehouse",  checkin: "08:47", target: 1710, achieved: 1599, status: "Active" },
        StaffMember { name: "Lê Hoàng Cường",  role: "Marketing Strategist", dept: "Marketing",  checkin: "09:12", target: 1540, achieved: 1212, status: "Late" },
        StaffMember { name: "Phạm Thu Dung",   role: "Inventory Manager",    dept: "Warehouse",  checkin: "08:55", target: 1540, achieved: 1452, status: "Active" },
        StaffMember { name: "Hoàng Minh Đức",  role: "Sales Associate",      dept: "Sales",      checkin: "--",    target: 1320, achieved: 0,    status: "Absent" },
        StaffMember { name: "Vũ Thị Hoa",      role: "Customer Support",     dept: "CX",         checkin: "08:28", target: 1200, achieved: 1088, status: "Active" },
    ];

    view! {
        <div class="p-6 space-y-6">
            // Header
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-xl font-black text-gray-900">"Staff Management"</h1>
                    <p class="text-xs text-gray-400 mt-0.5">"Track attendance, performance and team targets"</p>
                </div>
                <button class="flex items-center gap-1.5 bg-[#FCE300] hover:bg-yellow-400 text-gray-900 text-xs font-bold px-4 py-2.5 rounded-xl transition-colors cursor-pointer shadow-sm">
                    {icon_plus()} "Add Staff"
                </button>
            </div>

            // Top row: KPI + Gauge
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
                // KPI cards 2/3
                <div class="lg:col-span-2 grid grid-cols-2 sm:grid-cols-4 gap-3">
                    {[
                        ("Total Staff", "24", "#6366F1", "bg-indigo-50 border-indigo-100"),
                        ("Present",     "20", "#10B981", "bg-emerald-50 border-emerald-100"),
                        ("On Leave",    "2",  "#F59E0B", "bg-amber-50 border-amber-100"),
                        ("Absent",      "2",  "#F43F5E", "bg-rose-50 border-rose-100"),
                    ].iter().map(|&(label, val, color, bg)| view! {
                        <div class=format!("rounded-2xl p-5 border shadow-sm {}", bg)>
                            <p class="text-xs text-gray-500 font-medium">{label}</p>
                            <p class="text-3xl font-black mt-1.5" style={format!("color:{}", color)}>{val}</p>
                        </div>
                    }).collect_view()}
                </div>

                // Monthly target gauge 1/3
                <div class="bg-white rounded-2xl p-5 shadow-sm border border-gray-100 flex flex-col items-center justify-center">
                    <p class="text-xs font-bold text-gray-700 mb-3">"Monthly Target"</p>
                    <div class="relative flex items-center justify-center" style="width:128px;height:72px;overflow:hidden;">
                        <svg viewBox="0 0 128 72" style="width:128px;height:72px;">
                            <path d="M 10 64 A 54 54 0 0 1 118 64" fill="none" stroke="#F3F4F6" stroke-width="12" stroke-linecap="round"/>
                            <path d="M 10 64 A 54 54 0 0 1 118 64" fill="none" stroke="#FCE300" stroke-width="12" stroke-linecap="round"
                                stroke-dasharray="151.38 169.65"/>
                        </svg>
                        <div class="absolute bottom-0 inset-x-0 text-center pb-1">
                            <span class="text-xl font-black text-gray-900">"88%"</span>
                        </div>
                    </div>
                    <p class="text-xs text-emerald-500 font-semibold mt-2">"+4.5% vs last month"</p>
                </div>
            </div>

            // Staff table
            <div class="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
                <div class="flex items-center justify-between px-5 py-4 border-b border-gray-100">
                    <h2 class="text-sm font-bold text-gray-900">"Team Overview"</h2>
                </div>
                <div class="overflow-x-auto">
                    <table class="w-full text-xs">
                        <thead>
                            <tr class="border-b border-gray-100 text-gray-400 uppercase tracking-wider text-[10px]">
                                <th class="px-5 py-3 text-left font-medium">"Staff Member"</th>
                                <th class="px-4 py-3 text-left font-medium hidden md:table-cell">"Department"</th>
                                <th class="px-4 py-3 text-center font-medium hidden sm:table-cell">"Check-in"</th>
                                <th class="px-4 py-3 text-center font-medium">"Performance"</th>
                                <th class="px-4 py-3 text-center font-medium">"Status"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-gray-50">
                            {staff.into_iter().map(|s| {
                                let pct = if s.target > 0 { s.achieved as f64 / s.target as f64 * 100.0 } else { 0.0 };
                                let (bg, text) = match s.status {
                                    "Active" => ("bg-emerald-50 text-emerald-600 border-emerald-100", "Active"),
                                    "Late"   => ("bg-amber-50 text-amber-600 border-amber-100",       "Late"),
                                    _        => ("bg-rose-50 text-rose-600 border-rose-100",           "Absent"),
                                };
                                let gauge_color = if pct >= 90.0 { "#10B981" } else if pct >= 70.0 { "#FCE300" } else { "#F43F5E" };
                                view! {
                                    <tr class="hover:bg-gray-50 transition-colors duration-100">
                                        <td class="px-5 py-3.5">
                                            <div class="flex items-center gap-3">
                                                <div class="w-8 h-8 rounded-full bg-gradient-to-br from-violet-400 to-indigo-500 flex items-center justify-center text-white text-xs font-bold flex-shrink-0">
                                                    {s.name.chars().next().unwrap_or('?').to_string()}
                                                </div>
                                                <div>
                                                    <p class="font-semibold text-gray-900">{s.name}</p>
                                                    <p class="text-gray-400 text-[11px]">{s.role}</p>
                                                </div>
                                            </div>
                                        </td>
                                        <td class="px-4 py-3.5 text-gray-600 hidden md:table-cell">{s.dept}</td>
                                        <td class="px-4 py-3.5 text-center text-gray-500 font-mono hidden sm:table-cell">{s.checkin}</td>
                                        <td class="px-4 py-3.5 text-center">
                                            {gauge_small(pct, gauge_color)}
                                        </td>
                                        <td class="px-4 py-3.5 text-center">
                                            <span class=format!("text-[10px] font-semibold px-2.5 py-1 rounded-full border {}", bg)>
                                                {text}
                                            </span>
                                        </td>
                                    </tr>
                                }
                            }).collect_view()}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
