use behaviour::Behaviour;
use event::Event;

/// Any non-character element
pub struct Entity {
    name: String,
    behaviour: Vec<Box<Behaviour>>,
}

impl Entity {
    /// Creates a new instance of `Entity`
    pub fn new(name: String) -> Entity {
        Entity {
            name: name,
            behaviour: Vec::new()
        }
    }

    /// Adds a behaviour ot the behaviour chain of the entity
    pub fn append_behaviour(&mut self, behaviour: Box<Behaviour>) {
        self.behaviour.push(behaviour);
    }
}

impl Behaviour for Entity {
    fn handle_event(&self, event: Event) -> Event {
        let mut last_event = Event::Nothing;
        let mut first_run = true;

        for ref behaviour in &self.behaviour {
            if first_run {
                first_run = false;
                last_event = behaviour.handle_event(event.clone());
            } else {
                last_event = behaviour.handle_event(last_event);
            }
        }

        last_event
    }
}