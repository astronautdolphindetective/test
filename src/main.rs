use actix_web::{get, web::Path, HttpServer, HttpResponse, App, Responderuse serde::Serialize;



#[get("/home")]
async fn home() -> impl Responder{
    let response = "Welcom to Actix WEb Server";
    response
}


#[get("/hello/{firstname}/{lastname}")] 
async fn hello_user(params: Path<(String, String)>) -> HttpResponse{
    let response = User::new(params.0.clone(), params.1.clone());
    HttpResponse::Ok().json(response)
}



#[derive(Serialize)]
struct User{
    first_name: String, 
    last_name: String
}



impl User{

    fn new(first_name: String, last_name: String) -> Self{
        User{
            first_name, last_name
        }
    }
}

async fn main()  -> std::io::Result<()>{

    let server = HttpServer::new(||{
        App::new()
        .service(home)
        .service(hello_user)
    }).bind(("127.0.0.1", 8000))?
    .run();
    println!("Server Running at 127.0.0.1 at port 8000");
    server.await
}
