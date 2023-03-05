use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || App::new().route("/", web::get().to(index));

    HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .bind("[::1]:3000")?
        .run()
        .await
}

async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body(
        r#"
            <!doctype html>
            <form action="/gcd" method="post">
               <input type="text" name="n" />
               <input type="text" name="m" />
               <button type="submit">Compute GCD</button>
            </form>
        "#,
    )
}
