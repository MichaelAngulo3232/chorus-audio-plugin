use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use std::sync::Arc;
use std::time::Duration;
use nih_plug_iced::Font;
use crate::ChorusParams;

// This .ttf file is needed to use NotoSansMono font to display nice ASCII art lettering
pub const NOTO_SANS_MONO: Font = Font::External {
    name: "NotoSansMono",
    bytes: include_bytes!("../assets/NotoSansMono-Regular.ttf"),
};

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(700, 450)
}

pub(crate) fn create(
    params: Arc<ChorusParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<ChorusEditor>(editor_state, params)
}

struct ChorusEditor {
    params: Arc<ChorusParams>,
    context: Arc<dyn GuiContext>,

    rate_slider_state: nih_widgets::param_slider::State,
    depth_slider_state: nih_widgets::param_slider::State,
    mix_slider_state: nih_widgets::param_slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for ChorusEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<ChorusParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = ChorusEditor {
            params,
            context,

            rate_slider_state: Default::default(),
            depth_slider_state: Default::default(),
            mix_slider_state: Default::default(),
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                Text::new("
 ██████╗██████╗ ██╗███╗   ███╗███████╗ ██████╗ ███╗   ██╗
██╔════╝██╔══██╗██║████╗ ████║██╔════╝██╔═══██╗████╗  ██║
██║     ██████╔╝██║██╔████╔██║███████╗██║   ██║██╔██╗ ██║
██║     ██╔══██╗██║██║╚██╔╝██║╚════██║██║   ██║██║╚██╗██║
╚██████╗██║  ██║██║██║ ╚═╝ ██║███████║╚██████╔╝██║ ╚████║
 ╚═════╝╚═╝  ╚═╝╚═╝╚═╝     ╚═╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝")
        .font(NOTO_SANS_MONO)
        .size(12)
        .color(Color::from_rgb(0.9, 0.2, 0.2)) // Bright red
        .horizontal_alignment(alignment::Horizontal::Center)
            )
            .push(
                Text::new("Rate")
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
                    .color(Color::from_rgb(1.0, 0.0, 0.0)), // Red
            )
            .push(
                nih_widgets::ParamSlider::new(&mut self.rate_slider_state, &self.params.rate)
                    .map(Message::ParamUpdate),
            )
            .push(Space::with_height(10.into()))
            
            /*
            .push(
                nih_widgets::PeakMeter::new(
                    &mut self.peak_meter_state,
                    util::gain_to_db(self.peak_meter.load(std::sync::atomic::Ordering::Relaxed)),
                )
                .hold_time(Duration::from_millis(600)),
            )
            */

            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.07,
            g: 0.07,
            b: 0.07,
            a: 1.0,
        }
    }
}