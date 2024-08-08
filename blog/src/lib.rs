mod routes;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use routes::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (is_routing, set_is_routing) = create_signal(false);

    view! {
        <Router set_is_routing>
            <div class="routing-progress absolute top-0">
                <RoutingProgress
                    class="progress h-2"
                    is_routing
                    max_time=std::time::Duration::from_millis(250)
                />
            </div>
            <main>
                <Routes>
                    <Route path="" view=home::Page />
                    <Route path="posts" view=posts::Page />
                    <Route path="posts/:id" view=post::Page />

                    <Route path="*" view=_not_found::Page />
                </Routes>
            </main>
        </Router>
    }
}
