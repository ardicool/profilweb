use axum::Router;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1. Definisikan lokasi folder static
    // ServeDir akan mencari file di folder "static"
    let serve_dir = ServeDir::new("static");

    // 2. Buat Router
    // PERBAIKAN: Menggunakan .fallback_service() menggantikan .nest_service("/", ...)
    // Ini akan menangani semua request yang masuk ke root dan mencocokkannya dengan file di folder static
    let app = Router::new()
        .fallback_service(serve_dir);

    // 3. Tentukan alamat dan port (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server berjalan di http://{}", addr);

    // 4. Jalankan server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
