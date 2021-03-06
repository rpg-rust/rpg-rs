use event::Event;
use super::Behaviour;

/// A default reponse when a character is talking to an entity
#[derive(Clone)]
pub struct DefaultResponse {
    response: String,
}

impl DefaultResponse {
    /// Creates a new instance of `DefaultResponse`
    pub fn new(response: &str) -> DefaultResponse {
        DefaultResponse { response: response.to_string() }
    }
}

impl Behaviour for DefaultResponse {
    fn handle_event(&self, _: Event) -> Event {
        Event::Tell(self.response.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use event::Event;
    use entity::Entity;

    #[test]
    fn new_default_response() {
        let default_response = DefaultResponse::new("Responsy!");

        let mut entity = Entity::new("TestEntity");

        entity.append_behaviour(default_response.clone());

        assert_eq!(entity.send_event(Event::Nothing),
                   Event::Tell(String::from("Responsy!")));
    }
}
