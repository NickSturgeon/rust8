extern crate sdl2;

pub fn initialize(sdl_context: &sdl2::Sdl) -> sdl2::EventPump {
    sdl_context.event_pump().unwrap()
}

pub fn poll_for_event(event_pump: &mut sdl2::EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit {..} => panic!(),
            _ => {}
        }
    }
}