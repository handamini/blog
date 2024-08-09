use leptos::*;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <section class="flex flex-col items-center">
            <div class="avatar">
                <div class="w-28 rounded-full">
                    <img src="https://avatars.githubusercontent.com/u/176331224?v=4" />
                </div>
            </div>

            <div class="my-4" />

            <h1 class="text-4xl font-extrabold">HanDamin</h1>

            <div class="my-4" />
        </section>

        <div class="divider" />


        <nav class="nav">
            <div class="nav-item">
                <button class="nav-button">Posts</button>
                <div class="nav-description">
                    <p>dev tools and misc reviews</p>
                </div>
            </div>
            <div class="nav-item">
                <button class="nav-button">Reviews</button>
                <div class="nav-description">
                    <p>development log</p>
                </div>
            </div>
            <div class="nav-item">
                <button class="nav-button">Diary</button>
                <div class="nav-description">
                    <p>daily life</p>
                </div>
            </div>
            <div class="nav-item">
                <button class="nav-button">About Me</button>
                <div class="nav-description">
                    <p>career info</p>
                </div>
            </div>
        </nav>
    }
}
