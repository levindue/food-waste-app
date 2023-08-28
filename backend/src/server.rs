use crate::logic::{PeopleList, Person};
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use tiny_http::{Method, Request, Response, Server, StatusCode};

fn serve_404(request: Request) -> io::Result<()> {
    request.respond(Response::from_string("404").with_status_code(StatusCode(404)))
}

fn serve_hello_world(request: Request) -> io::Result<()> {
    request.respond(Response::from_string("Hello World"))
}

fn serve_get_people(request: Request) -> io::Result<()> {
    let people_list = PeopleList::load_from_file("data.json").unwrap_or_else(|_| PeopleList::new());
    let response_json = serde_json::to_string(&people_list.people)?;

    request.respond(Response::from_string(response_json))
}

fn serve_add_person(mut request: Request) -> io::Result<()> {
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content)?;

    let person: Person = serde_json::from_str(&content)?;

    let mut people_list =
        PeopleList::load_from_file("data.json").unwrap_or_else(|_| PeopleList::new());
    people_list.add_person(person);
    people_list.save_to_file("data.json").unwrap();

    request.respond(Response::empty(200))
}

fn serve_add_food(mut request: Request) -> io::Result<()> {
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content)?;

    let data: HashMap<String, Value> = serde_json::from_str(&content)?;

    let person_id = data
        .get("person_id")
        .and_then(|id| id.as_u64())
        .expect("Invalid person_id");

    let food = data
        .get("food")
        .and_then(|food| food.as_str())
        .expect("Invalid food");

    let mut people_list =
        PeopleList::load_from_file("data.json").unwrap_or_else(|_| PeopleList::new());

    if let Some(person) = people_list
        .people
        .iter_mut()
        .find(|p| p.id == person_id as u32)
    {
        person.add_food(food);
        people_list.save_to_file("data.json").unwrap();
        request.respond(
            Response::from_string("Food added to person").with_status_code(StatusCode(200)),
        )
    } else {
        request.respond(Response::from_string("Person not found").with_status_code(StatusCode(404)))
    }
}

fn serve_request(request: Request) -> io::Result<()> {
    println!(
        "INFO: received request! method: {:?}, url: {:?}",
        request.method(),
        request.url()
    );

    match (request.method(), request.url()) {
        (Method::Get, "/") => serve_hello_world(request),
        (Method::Get, "/api/people") => serve_get_people(request),
        (Method::Post, "/api/people") => serve_add_person(request),
        (Method::Post, "/api/people/add_food") => serve_add_food(request),
        _ => serve_404(request),
    }
}

pub fn start(addr: &str) -> Result<(), ()> {
    let server = Server::http(&addr).map_err(|err| {
        eprintln!("ERROR: could not start HTTP server at {addr}: {err}");
    })?;

    println!("INFO: listening at http://{addr}/");

    for request in server.incoming_requests() {
        serve_request(request)
            .map_err(|err| {
                eprintln!("ERROR: could not serve the response: {err}");
            })
            .ok(); // <- don't stop on errors, keep serving
    }

    eprintln!("ERROR: the server socket has shutdown");
    Err(())
}
