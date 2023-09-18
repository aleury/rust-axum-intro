//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTicketParams {
    pub title: String,
}

type Store<T> = Arc<Mutex<Vec<Option<T>>>>;

#[derive(Clone)]
pub struct TicketService {
    ticket_store: Store<Ticket>,
}

impl TicketService {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ticket_store: Arc::default(),
        })
    }
}

impl TicketService {
    pub async fn create_ticket(&self, params: CreateTicketParams) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();

        let ticket = Ticket {
            id: store.len() as u64,
            title: params.title,
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();

        let tickets: Vec<Ticket> = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketNotFound { id })
    }
}
