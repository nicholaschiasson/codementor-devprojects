use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum FormMsg {
	Post(String, String, String, String),
}

#[derive(Clone, Properties)]
pub struct FormProps {
	pub callback: Callback<FormMsg>,
}

pub struct Form {
	link: ComponentLink<Self>,
	props: FormProps,
	refs: Vec<NodeRef>,
}

impl Component for Form {
	type Message = ();
	type Properties = FormProps;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			props,
			refs: vec![
				NodeRef::default(),
				NodeRef::default(),
				NodeRef::default(),
				NodeRef::default(),
			],
		}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		self.props.callback.emit(FormMsg::Post(
			self.refs[0].cast::<HtmlInputElement>().unwrap().value(),
			self.refs[1].cast::<HtmlInputElement>().unwrap().value(),
			self.refs[2].cast::<HtmlInputElement>().unwrap().value(),
			self.refs[3].cast::<HtmlInputElement>().unwrap().value(),
		));
		true
	}

	fn view(&self) -> Html {
		html! {
			<div>
				<form id="userform" action="/display" method="POST">
					<div>
						<label for="username">{ "Username" }</label>
						<input type="text" ref=self.refs[0].clone() id="username" name="username"/>
					</div>
					<div>
						<label for="email">{ "Email" }</label>
						<input type="email" ref=self.refs[1].clone() id="email" name="email"/>
					</div>
					<div>
						<label for="issue">{ "Issue" }</label>
						<select ref=self.refs[2].clone() id="issue" name="issue">
							<option value="Query">{ "Query" }</option>
							<option value="Feedback">{ "Feedback" }</option>
							<option value="Complaint">{ "Complaint" }</option>
							<option value="Other">{ "Other" }</option>
						</select>
					</div>
					<div>
						<label for="comment">{ "Comment" }</label>
						<textarea ref=self.refs[3].clone() name="comment"></textarea>
					</div>
					<div>
						<input type="button" onclick=self.link.callback(|_| ()) value="Submit"/>
					</div>
				</form>
			</div>
		}
	}
}
