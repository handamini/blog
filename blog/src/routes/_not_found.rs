use leptos::*;
use leptos_router::*;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <div class="hero bg-base-200 min-h-screen">
            <div class="hero-content text-center">
                <div class="max-w-lg">
                    <h1 class="text-9xl font-bold text-primary">404</h1>
                    <h2 class="text-4xl font-bold mt-4">Page Not Found</h2>
                    <p class="py-6 text-bold">
                        "I'm sorry, but the page you're looking for isn't here."
                        <br />
                        "It might have been moved or deleted."
                        <br />
                        <br />
                        "Let's get you back to the main page"
                        <br />
                        "where you can find more of my content."
                    </p>
                    <A class="btn btn-primary" href="/">Return Home</A>
                </div>
            </div>
        </div>
    }
}
