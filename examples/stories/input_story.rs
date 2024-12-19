use gpui::{
    actions, AppContext, InteractiveElement, IntoElement, KeyBinding, ParentElement as _, Render,
    Styled, View, ViewContext, VisualContext, WindowContext,
};

use luna::{v_flex, InputEvent, TextInput};

actions!(input_story, [Tab, TabPrev]);

const CONTEXT: &str = "InputStory";

pub fn init(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("shift-tab", TabPrev, Some(CONTEXT)),
        KeyBinding::new("tab", Tab, Some(CONTEXT)),
    ])
}

pub struct InputStory {
    input1: View<TextInput>,
    input2: View<TextInput>,
}

impl InputStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(Self::new)
    }

    fn new(cx: &mut ViewContext<Self>) -> Self {
        let input1 = cx.new_view(|cx| {
            let mut input = TextInput::new(cx);
            input.set_text("Ehi Ciao! Un saluto da LUNA!", cx);
            input
        });
        cx.subscribe(&input1, Self::on_input_event).detach();

        let input2 = cx.new_view(|cx| TextInput::new(cx).placeholder("Inserisci del testo qui..."));
        cx.subscribe(&input2, Self::on_input_event).detach();

        Self { input1, input2 }
    }

    fn tab(&mut self, _: &Tab, cx: &mut ViewContext<Self>) {
        self.cycle_focus(true, cx);
    }

    fn tab_prev(&mut self, _: &TabPrev, cx: &mut ViewContext<Self>) {
        self.cycle_focus(false, cx);
    }

    fn on_input_event(
        &mut self,
        _: View<TextInput>,
        event: &InputEvent,
        _cx: &mut ViewContext<Self>,
    ) {
        match event {
            InputEvent::Change(text) => println!("Change: {}", text),
            InputEvent::PressEnter => println!("PressEnter"),
            InputEvent::Focus => println!("Focus"),
            InputEvent::Blur => println!("Blur"),
        };
    }

    fn cycle_focus(&self, is_next: bool, cx: &mut ViewContext<Self>) {
        let handles = vec![self.input1.focus_handle(cx), self.input2.focus_handle(cx)];
        let focused_handle = cx.focused();
        let handles = if is_next {
            handles
        } else {
            handles.into_iter().rev().collect()
        };

        let fallback_handle = handles[0].clone();
        let target_focus_handle = handles
            .into_iter()
            .skip_while(|handle| Some(handle) != focused_handle.as_ref())
            .skip(1)
            .next()
            .unwrap_or(fallback_handle);

        target_focus_handle.focus(cx);
        cx.stop_propagation();
    }
}

impl gpui::FocusableView for InputStory {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.input1.focus_handle(cx)
    }
}

impl Render for InputStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context(CONTEXT)
            .id("input-story")
            .on_action(cx.listener(Self::tab))
            .on_action(cx.listener(Self::tab_prev))
            .size_full()
            .p_4()
            .justify_start()
            .gap_3()
            .child(self.input1.clone())
            .child(self.input2.clone())
    }
}
