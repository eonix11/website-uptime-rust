use poem::Error;
use poem::{Route, Server, get, handler, listener::TcpListener, post, web::Path};

use poem::web::Json;

//importing request input and output types
pub mod req_inputs;
pub mod req_outputs;
use crate::req_inputs::CreateUserInput;
use crate::req_outputs::{CreateUserOutput, GetWebsiteOutput, SignInOutput};
use req_inputs::CreateWebsiteInput;
use req_outputs::CreateWebsiteOutput;

use store::models::website::Website;
use store::store::Store;

#[handler]
fn get_website(Path(id): Path<String>) -> Json<GetWebsiteOutput> {
    let mut s = Store::default().unwrap();
    let website = s.get_website(id).unwrap();

    Json(GetWebsiteOutput { url: website.url })
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>) -> Json<SignInOutput> {
    let mut s = Store::default().unwrap();

    let exists = s.sign_up(data.username, data.password).unwrap();

    let response: SignInOutput = SignInOutput {
        jwt: String::from("Junaid"),
    };

    Json(response)
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<SignInOutput> {
    let mut s = Store::default().unwrap();

    let exists = s.sign_up(data.username, data.password).unwrap();

    let response: SignInOutput = SignInOutput {
        jwt: String::from("Junaid"),
    };

    Json(response)
}

//getting json inputs and outputs can be done like this:
#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url: String = data.url;
    //logic to persiste in database would go here
    //sqlx -> postgres
    //diesel -> orm

    let mut s = Store::default().unwrap();
    let wesite: Website = s.create_website(String::from("hdjshjdhsjdh"), url).unwrap();

    let response: CreateWebsiteOutput = CreateWebsiteOutput { id: wesite.id };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //sets up the route with tracing middleware
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", get(sign_up))
        .at("/user/signin", post(sign_in));
    //creates and runs the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
