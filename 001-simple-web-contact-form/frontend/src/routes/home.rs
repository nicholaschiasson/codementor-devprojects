use yew::prelude::*;

use crate::components::Display;
use crate::components::{Form, FormMsg};

enum State {
	Display(String, String, String, String),
	Form,
}

pub struct Home {
	link: ComponentLink<Self>,
	state: State,
}

impl Component for Home {
	type Message = FormMsg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			state: State::Form,
		}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			FormMsg::Post(username, email, issue, comment) => {
				self.state = State::Display(username, email, issue, comment)
			}
		}
		true
	}

	fn view(&self) -> Html {
		match &self.state {
			State::Display(username, email, issue, comment) => {
				html! { <Display username=username, email=email, issue=issue, comment=comment/> }
			}
			State::Form => html! { <Form callback=self.link.callback(|msg| msg)/> },
		}
	}
}
