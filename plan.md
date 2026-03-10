Technical Integration Prompt: Rust/Leptos Frontend & Existing E-commerce Backend
Role: You are a Senior Full-stack Engineer specializing in the Rust ecosystem, specifically Leptos (0.6+), Axum, and Tailwind CSS. Your task is to integrate an existing e-commerce backend into a brand new Leptos frontend.

Project Core:

Source: Existing Backend API (assume REST/JSON).

Target: Leptos Web Framework (SSR + Hydration preferred).

Styling: Tailwind CSS (JIT mode).

Phase 1: Type Synchronization & DTO Mapping
Strict Typing: Examine the existing backend data structures. Generate equivalent Rust structs using serde::Deserialize and serde::Serialize.

Shared Logic: If possible, suggest a way to share types between backend and frontend (if the backend is also Rust). If not, create a dto.rs module in the frontend.

Newtype Pattern: Use the Newtype pattern for IDs (e.g., ProductId(String)) to prevent logic errors during integration.

Phase 2: The API Client Layer (Reqwest/Gloo-net)
Create a centralized api module.

Implement a generic fetch_api<T> function that handles:

Base URL configuration via environment variables.

Standard headers (Content-Type, Accept).

JWT Injection: Automatically pull the token from LocalStorage or a Cookie and add it to the Authorization header.

Implement explicit error types using thiserror to map HTTP status codes (401, 403, 404, 422, 500) into UI-friendly error messages.

Phase 3: Leptos Resource & Signal Architecture
Data Fetching: Use create_resource for all data-driven views (Product list, Detail, Cart).

Server Functions: Demonstrate how to use #[server] functions to proxy requests if we need to hide API keys or bypass CORS during SSR.

Suspense & Transitions: Wrap API-dependent components in <Suspense /> and <ErrorBoundary />. Show how to implement "Skeleton Loaders" using Tailwind CSS animate-pulse.

Phase 4: State Management (The Global Cart)
Initialize a CartState using provide_context and use_context.

Persistence: Implement a synchronization loop where any change to the CartSignal is automatically debounced and saved to LocalStorage.

Optimistic UI: When a user clicks "Add to Cart", update the local signal immediately before the API call finishes, with a rollback mechanism if the backend fails.

Phase 5: Tailwind CSS Component Integration
Layouts: Create a responsive MainLayout using Tailwind Grid/Flexbox.

Dynamic Styling: Show how to use Leptos signals to toggle Tailwind classes (e.g., class:border-red-500=move_signal).

Complex UI: Implement a "Mini-cart" drawer and a "Filter Sidebar" using purely functional Leptos components (no heavy JS libraries).

Phase 6: Routing & Auth Guards
Define routes using leptos_router.

Auth Guard: Create a wrapper component or a higher-order function to protect /account, /checkout, and /order-history routes. Redirect unauthenticated users to /login.

Search Params: Integrate URL search params (e.g., ?category=electronics&sort=price_desc) directly into the create_resource dependency array.

Output Requirements:

Code Quality: Provide modular, "Dry" (Don't Repeat Yourself) code.

Build Pipeline: Provide a Trunk.toml configuration that includes the Tailwind CSS build step.

No Fluff: Do not explain "what an e-commerce app is". Focus strictly on the technical bridge between the existing API and the new Rust frontend.

Hydration Safety: Ensure all browser-only APIs (like window.local_storage) are wrapped in is_server() checks to avoid hydration mismatches.

Final Instruction:
Start by generating the Folder Structure and the API Client Module based on the logic above. Then, provide the implementation for the Product Grid fetching from the existing /api/v1/products endpoint.