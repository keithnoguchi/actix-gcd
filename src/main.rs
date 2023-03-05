use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || {
        App::new()
            .route("/", web::get().to(index))
            .route("/gcd", web::post().to(get_gcd))
    };

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
               <input type="text" name="m" />
               <input type="text" name="n" />
               <button type="submit">Compute GCD</button>
            </form>
        "#,
    )
}

#[derive(serde::Deserialize)]
struct GcdForm {
    m: u64,
    n: u64,
}

async fn get_gcd(form: web::Form<GcdForm>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body(
            r#"
                    <!doctype html>
                    <p>Computing the GCD with zero is boring.</p>
                "#,
        );
    }

    let resp = format!(
        r#"
            <!doctype html>
            <p>
                The greatest common divisor of the numbers {} and {}
                is <b>{}</b>.
            <p>
        "#,
        form.m,
        form.n,
        gcd(form.m, form.n),
    );
    HttpResponse::Ok().content_type("text/html").body(resp)
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
