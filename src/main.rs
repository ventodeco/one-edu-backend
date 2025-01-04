use one_edu_backend::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting apps!");
    run().await
}
