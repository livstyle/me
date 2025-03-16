use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    // provide_meta_context();

    view! {
        <head>
            <title>"Livstyle AI"</title>
            <link rel="stylesheet" href="/style/output.css"/>
            <link rel="stylesheet" href="/style/style.css"/>
            <link rel="shortcut icon" href="/favicon.ico"/>
        </head>
        <div class="min-h-screen bg-black relative overflow-hidden">
            // Starry background
            <div class="star-field absolute inset-0"></div>
            
            // Main content
            <div class="relative z-10 container mx-auto px-4 py-16">
                <div class="text-center text-white">
                    // Profile section
                    <div class="mb-12">
                        <img
                            src="https://avatars.githubusercontent.com/livstyle"
                            alt="Profile"
                            class="w-32 h-32 rounded-full mx-auto mb-6 border-4 border-blue-500"
                        />
                        <h1 class="text-4xl font-bold mb-2">"姜坤 (livstyle)"</h1>
                        <p class="text-xl text-gray-300 mb-4">"🏠 Working from home | Ruster"</p>
                        <p class="text-gray-400">"广州"</p>
                    </div>

                    // Stats section
                    <div class="grid grid-cols-3 gap-6 max-w-2xl mx-auto mb-12">
                        <div class="bg-gray-800 bg-opacity-50 rounded-lg p-4">
                            <h3 class="text-2xl font-bold">"127"</h3>
                            <p class="text-gray-400">"Repositories"</p>
                        </div>
                        <div class="bg-gray-800 bg-opacity-50 rounded-lg p-4">
                            <h3 class="text-2xl font-bold">"13"</h3>
                            <p class="text-gray-400">"Followers"</p>
                        </div>
                        <div class="bg-gray-800 bg-opacity-50 rounded-lg p-4">
                            <h3 class="text-2xl font-bold">"78"</h3>
                            <p class="text-gray-400">"Following"</p>
                        </div>
                    </div>

                    // Popular repositories section
                    <div class="max-w-4xl mx-auto">
                        <h2 class="text-2xl font-bold mb-6">"Popular Repositories"</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            <div class="bg-gray-800 bg-opacity-50 rounded-lg p-6 text-left">
                                <h3 class="text-xl font-bold mb-2">"kata-os"</h3>
                                <p class="text-gray-400 mb-3">"KADAOS"</p>
                                <div class="flex items-center">
                                    <span class="w-3 h-3 bg-[#DEA584] rounded-full mr-2"></span>
                                    <span class="text-gray-400">"Rust"</span>
                                </div>
                            </div>
                            <div class="bg-gray-800 bg-opacity-50 rounded-lg p-6 text-left">
                                <h3 class="text-xl font-bold mb-2">"tensorbase"</h3>
                                <p class="text-gray-400 mb-3">"TensorBase is building a modern big data warehouse"</p>
                                <div class="flex items-center">
                                    <span class="w-3 h-3 bg-[#DEA584] rounded-full mr-2"></span>
                                    <span class="text-gray-400">"Rust"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

