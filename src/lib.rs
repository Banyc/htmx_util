use maud::{html, Markup, DOCTYPE};

pub fn base_html(body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            script src="https://unpkg.com/htmx.org@1.9.10"
                integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC"
                crossorigin="anonymous" {}
        }
        body {
            (body)
        }
    }
}
