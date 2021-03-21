use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct DisplayProps {
	pub username: String,
	pub email: String,
	pub issue: String,
	pub comment: String,
}

pub struct Display {
	props: DisplayProps,
}

impl Component for Display {
	type Message = ();
	type Properties = DisplayProps;

	fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self { props }
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		true
	}

	fn view(&self) -> Html {
		html! {
			<div>
				<p>
					{ format!("{} from ", self.props.issue) }
					<a href=format!("mailto:{}", self.props.email)>{ self.props.username.as_str() }</a>
				</p>
				<p>{ self.props.comment.as_str() }</p>
			</div>
		}
	}
}
