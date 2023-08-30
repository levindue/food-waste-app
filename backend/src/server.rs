use crate::logic::{Food, Manager, Person};
use std::io;
use tiny_http::{Method, Request, Response, Server, StatusCode};

fn serve_404(request: Request) -> io::Result<()> {
    request.respond(Response::from_string("404").with_status_code(StatusCode(404)))
}

fn serve_list_people(request: Request, manager: &Manager) -> io::Result<()> {
    let people = manager.list_people();
    let response = serde_json::to_string(&people)?;
    request.respond(Response::from_string(response).with_status_code(StatusCode(200)))
}

fn serve_add_person(mut request: Request, manager: &mut Manager) -> io::Result<()> {
    let mut buffer = Vec::new();
    request.as_reader().read_to_end(&mut buffer)?;

    let person: Person = serde_json::from_slice(&buffer)?;
    manager.add_person(person);

    manager.save_to_file("data.json").unwrap();
    request.respond(Response::from_string("Succesfully added person").with_status_code(200))
}

fn serve_remove_person(mut request: Request, manager: &mut Manager) -> io::Result<()> {
    let mut buffer = Vec::new();
    request.as_reader().read_to_end(&mut buffer)?;

    let person_id: u32 = serde_json::from_slice(&buffer)?;
    manager.remove_person(person_id);

    manager.save_to_file("data.json").unwrap();
    request.respond(Response::from_string("Succesfully removed person").with_status_code(200))
}

fn serve_list_food(request: Request, manager: &Manager, person_id: u32) -> io::Result<()> {
    if let Some(person) = manager.people.get(&person_id) {
        let food_list = manager.list_food(&person);
        let response = serde_json::to_string(&food_list)?;
        request.respond(Response::from_string(response).with_status_code(StatusCode(200)))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Person with id {} not found", person_id),
        ))
    }
}

fn serve_list_all_food(request: Request, manager: &Manager) -> io::Result<()> {
    let all_food = manager.list_all_food();
    let response = serde_json::to_string(&all_food)?;
    request.respond(Response::from_string(response).with_status_code(StatusCode(200)))
}

fn serve_add_food(mut request: Request, manager: &mut Manager) -> io::Result<()> {
    let mut buffer = Vec::new();
    request.as_reader().read_to_end(&mut buffer)?;

    let food: Food = serde_json::from_slice(&buffer)?;
    manager.add_food(food);

    manager.save_to_file("data.json").unwrap();
    request
        .respond(Response::from_string("Food added successfully").with_status_code(StatusCode(200)))
}

fn serve_remove_food(mut request: Request, manager: &mut Manager) -> io::Result<()> {
    let mut buffer = Vec::new();
    request.as_reader().read_to_end(&mut buffer)?;

    let food_id: u32 = serde_json::from_slice(&buffer)?;
    manager.remove_food(food_id);

    manager.save_to_file("data.json").unwrap();
    request.respond(
        Response::from_string("Food removed successfully").with_status_code(StatusCode(200)),
    )
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
