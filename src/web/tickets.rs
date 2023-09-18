use axum::extract::{Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};

use crate::model::{CreateTicketParams, Ticket, TicketService};
use crate::Result;

pub fn routes(ticket_service: TicketService) -> Router {
    Router::new()
        .route("/tickets", get(list_tickets).post(create_ticket))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(ticket_service)
}

pub async fn create_ticket(
    State(ticket_service): State<TicketService>,
    Json(params): Json<CreateTicketParams>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = ticket_service.create_ticket(params).await?;

    Ok(Json(ticket))
}

pub async fn list_tickets(
    State(ticket_service): State<TicketService>,
) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets = ticket_service.list_tickets().await?;

    Ok(Json(tickets))
}

pub async fn delete_ticket(
    State(ticket_service): State<TicketService>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let ticket = ticket_service.delete_ticket(id).await?;

    Ok(Json(ticket))
}
