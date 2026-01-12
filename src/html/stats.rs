use maud::{Markup, html};

use crate::html::head;

pub async fn stats() -> Markup {
    html! {
        (head("Zbiorywalizacja WPiK"))
        body.bg-neutral-900.text-neutral-300.lg:min-h-screen.w-full.flex.flex-col {
            p.text-center.text-2xl.font-serif.py-4 { "Zbiorywalizacja WPiK" }
            .pb-4.lg:pb-8.px-4.lg:px-8.w-full.grid.grid-cols-3.grid-rows-3.gap-3.flex-1 {
                div class="bg-neutral-700 flex justify-center items-center border border-neutral-500 rounded row-span-2" {
                    "Logo or something"
                }
                div class="bg-neutral-700 flex justify-center items-center text-center border border-neutral-500 rounded row-span-2 col-span-2" {
                    "Main graph (total amts in each container - a graph)"
                }
                div class="bg-neutral-700 flex justify-center items-center border border-neutral-500 rounded" {
                    "THIS CONTAINER is in the lead by THIS AMOUNT"
                }
                div class="bg-neutral-700 flex justify-center items-center border border-neutral-500 rounded" {
                    "TOTAL AMOUNT OF DONATIONS and SUM OF DONATION AMOUNTS"
                }
                div class="bg-neutral-700 flex justify-center items-center border border-neutral-500 rounded" {
                    "graph of lead over time"
                }
            }
        }
    }
}
