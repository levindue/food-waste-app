use crate::logic::{Food, Manager, Person};
use std::io;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

fn serve_404(request: Request) -> io::Result<()> {
    request.respond(Response::from_string("404").with_status_code(StatusCode(404)))
}

fn add_cors_header<R>(response: &mut Response<R>)
where
    R: std::io::Read,
{
    response.add_header(
        Header::from_bytes(
            &b"Access-Control-Allow-Origin"[..],
            &b"http://localhost:3000"[..],
        )
        .unwrap(),
    );
}

fn serve_list_people(request: Request, manager: &Manager) -> io::Result<()> {
    let people = manager.list_people();
    let mut response = Response::from_string(serde_json::to_string(&people)?)
        .with_status_code(StatusCode(200));
    add_cors_header(&mut response);
    request.respond(response)
}

fn serve_add_person(mut request: Request, manager: &mut Manager) -> io::Result<()> {
    let mut buffer = Vec::new();
    request.as_reader().read_to_end(&mut buffer)?;

    let person: Person = serde_json::from_slice(&buffer)?;
    manager.add_person(person);

    manager.save_to_file("data.json").unwrap();
    let mut response = Response::from_string("Successfully added person")
        .with_status_code(StatusCode(200));
    add_cors_header(&mut response);
    request.respond(response)
}

fn serve_remove_person(mut request: Request, manager: &mut Manager) -> io::Result<()> {
    let mut buffer = Vec::new();
    request.as_reader().read_to_end(&mut buffer)?;

    let person_id: u32 = serde_json::from_slice(&buffer)?;
    manager.remove_person(person_id);

    manager.save_to_file("data.json").unwrap();
    let mut response = Response::from_string("Successfully removed person")
        .with_status_code(StatusCode(200));
    add_cors_header(&mut response);
    request.respond(response)
}

fn serve_list_food(request: Request, manager: &Manager, person_id: u32) -> io::Result<()> {
    if let Ok(food_list) = manager.list_food(person_id) {
        let response = serde_json::to_string(food_list)?;
        let mut response = Response::from_string(response).with_status_code(StatusCode(200));
        add_cors_header(&mut response);
        request.respond(response)
    } else {
        serve_404(request)
    }
}

fn serve_list_all_food(request: Request, manager: &Manager) -> io::Result<()> {
    let all_food = manager.list_all_food();
    let response = serde_json::to_string(&all_food)?;
    let mut response = Response::from_string(response).with_status_code(StatusCode(200));
    add_cors_header(&mut response);
    request.respond(response)
}

fn serve_add_food(mut request: Request, manager: &mut Manager) -> io::Result<()> {
    let mut buffer = Vec::new();
    request.as_reader().read_to_end(&mut buffer)?;

    let (food, person_id): (Food, u32) = serde_json::from_slice(&buffer)?;

    if let Err(err) = manager.add_food(food, person_id) {
        let mut response = Response::from_string(format!("Failed to add food: {}", err))
            .with_status_code(400);
        add_cors_header(&mut response);
        return request.respond(response);
    }

    manager.save_to_file("data.json").unwrap();
    let mut response = Response::from_string("Successfully added food").with_status_code(200);
    add_cors_header(&mut response);
    request.respond(response)
}


fn serve_remove_food(mut request: Request, manager: &mut Manager) -> io::Result<()> {
    let mut buffer = Vec::new();
    request.as_reader().read_to_end(&mut buffer)?;

    let (food_id, person_id): (u32, u32) = serde_json::from_slice(&buffer)?;
    if let Err(err) = manager.remove_food(food_id, person_id) {
        let mut response =
            Response::from_string(format!("Failed to remove food: {}", err)).with_status_code(400);
        add_cors_header(&mut response);
        return request.respond(response);
    }

    manager.save_to_file("data.json").unwrap();
    let mut response = Response::from_string("Successfully removed food").with_status_code(200);
    add_cors_header(&mut response);
    request.respond(response)
}

fn serve_request(manager: &mut Manager, request: Request) -> io::Result<()> {
    println!(
        "INFO: received request! method: {:?}, url: {:?}",
        request.method(),
        request.url()
    );

    match (request.method(), request.url()) {
        (Method::Get, "/api/list_people") => serve_list_people(request, manager),
        (Method::Post, "/api/add_person") => serve_add_person(request, manager),
        (Method::Post, "/api/remove_person") => serve_remove_person(request, manager),
        (Method::Get, path) if path.starts_with("/api/list_food/") => {
            let person_id = path.trim_start_matches("/api/list_food/").parse::<u32>().unwrap();
            serve_list_food(request, manager, person_id)
        }
        (Method::Get, "/api/list_food") => serve_list_all_food(request, manager),
        (Method::Post, "/api/add_food") => serve_add_food(request, manager),
        (Method::Post, "/api/remove_food") => serve_remove_food(request, manager),
        _ => serve_404(request),
    }
}

pub fn start(addr: &str, manager: &mut Manager) -> Result<(), ()> {
    let server = Server::http(&addr).map_err(|err| {
        eprintln!("ERROR: could not start HTTP server at {addr}: {err}");
    })?;

    println!("INFO: listening at http://{addr}/");

    for request in server.incoming_requests() {
        if let Err(err) = serve_request(manager, request) {
            eprintln!("ERROR: could not serve the response: {err}");
        }
    }

    eprintln!("ERROR: the server socket has shutdown");
    Err(())
}
