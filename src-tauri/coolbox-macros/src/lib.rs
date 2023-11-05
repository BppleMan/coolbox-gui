mod state;
mod task;

use proc_macro::TokenStream;

#[proc_macro_derive(State)]
pub fn derive_state(item: TokenStream) -> TokenStream {
    state::derive_state(item)
}

#[proc_macro_derive(TaskRef)]
pub fn derive_task(item: TokenStream) -> TokenStream {
    task::derive_task(item)
}
