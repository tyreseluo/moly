use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::resource_imports::*;
    use crate::shared::widgets::MolyButton;

    SUCCESS_ICON = dep("crate://self/resources/images/success_icon.png")
    FAILURE_ICON = dep("crate://self/resources/images/failure_icon.png")

    PRIMARY_LINK_FONT_COLOR = #x0E7090
    SECONDARY_LINK_FONT_COLOR = #667085

    PopupActionLink = <LinkLabel> {
        width: Fit,
        margin: 2,
        draw_text: {
            text_style: <BOLD_FONT>{font_size: 9},
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        PRIMARY_LINK_FONT_COLOR,
                        PRIMARY_LINK_FONT_COLOR,
                        self.hover
                    ),
                    PRIMARY_LINK_FONT_COLOR,
                    self.down
                )
            }
        }
    }

    PopupSecondaryActionLink = <LinkLabel> {
        width: Fit,
        margin: 2,
        draw_text: {
            text_style: <BOLD_FONT>{font_size: 9},
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        SECONDARY_LINK_FONT_COLOR,
                        SECONDARY_LINK_FONT_COLOR,
                        self.hover
                    ),
                    SECONDARY_LINK_FONT_COLOR,
                    self.down
                )
            }
        }
    }

    UpdaterPopupDialog = <RoundedView> {
        width: 350
        height: Fit
        margin: {top: 20, right: 20}
        padding: {top: 20, right: 20 bottom: 20 left: 20}
        spacing: 15

        show_bg: true
        draw_bg: {
            color: #fff
            instance border_radius: 4.0
            fn pixel(self) -> vec4 {
                let border_color = #d4;
                let border_width = 1;
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let body = #fff

                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    self.border_radius
                )
                sdf.fill_keep(body)

                sdf.stroke(
                    border_color,
                    border_width
                )
                return sdf.result
            }
        }
    }

    UpdaterCloseButton = <MolyButton> {
        width: Fit,
        height: Fit,

        margin: {top: -8}

        draw_icon: {
            svg_file: (ICON_CLOSE),
            fn get_color(self) -> vec4 {
                return #000;
            }
        }
        icon_walk: {width: 10, height: 10}
    }

    UpdaterIcon = <View> {
        width: Fit,
        height: Fit,
        margin: {top: -10, left: -10}
        success_icon = <View> {
            width: Fit,
            height: Fit,
            <Image> {
                source: (SUCCESS_ICON),
                width: 35,
                height: 35,
            }
        }
        failure_icon = <View> {
            visible: false,
            width: Fit,
            height: Fit,
            <Image> {
                source: (FAILURE_ICON),
                width: 35,
                height: 35,
            }
        }
    }

    UpdaterContent = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 10

        title = <Label> {
            draw_text:{
                text_style: <BOLD_FONT>{font_size: 9},
                word: Wrap,
                color: #000
            }
            text: "Checking for updates"
        }

        summary = <Label> {
            width: Fill,
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 9},
                word: Wrap,
                color: #000
            }
            text: "Please wait..."
        }

        actions = <View> {
            width: Fit,
            height: Fit,
            spacing: 10,

            install_link = <PopupActionLink> {
                visible: false,
                text: "Install and Restart"
            }

            cancel_link = <PopupSecondaryActionLink> {
                visible: false,
                text: "Cancel"
            }
        }
    }

    pub UpdaterNotificationPopup = {{UpdaterNotificationPopup}} {
        width: Fit
        height: Fit

        <UpdaterPopupDialog> {
            <UpdaterIcon> {}
            <UpdaterContent> {}
            close_button = <UpdaterCloseButton> {}
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum UpdaterNotificationPopupAction {
    None,
    CloseButtonClicked,
    InstallAndRestartClicked,
    CancelClicked,
}

#[derive(Live, LiveHook, Widget)]
pub struct UpdaterNotificationPopup {
    #[deref]
    view: View,

    #[layout]
    layout: Layout,
}

impl Widget for UpdaterNotificationPopup {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self
            .view
            .draw_walk(cx, scope, walk.with_abs_pos(DVec2 { x: 0., y: 0. }));

        DrawStep::done()
    }
}

impl WidgetMatchEvent for UpdaterNotificationPopup {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(ids!(close_button)).clicked(actions) {
            cx.action(UpdaterNotificationPopupAction::CloseButtonClicked);
        }

        if self.link_label(ids!(install_link)).clicked(actions) {
            cx.action(UpdaterNotificationPopupAction::InstallAndRestartClicked);
        }

        if self.link_label(ids!(cancel_link)).clicked(actions) {
            cx.action(UpdaterNotificationPopupAction::CancelClicked);
        }
    }
}

impl UpdaterNotificationPopup {
    fn show_success_icon(&mut self, cx: &mut Cx) {
        self.view(ids!(success_icon)).set_visible(cx, true);
        self.view(ids!(failure_icon)).set_visible(cx, false);
    }

    fn set_update_actions_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.view(ids!(actions)).set_visible(cx, visible);
        self.link_label(ids!(install_link)).set_visible(cx, visible);
        self.link_label(ids!(cancel_link)).set_visible(cx, visible);
    }

    fn set_content(&mut self, cx: &mut Cx, title: &str, summary: &str) {
        self.label(ids!(title)).set_text(cx, title);
        self.label(ids!(summary)).set_text(cx, summary);
    }
}

impl UpdaterNotificationPopupRef {
    pub fn show_update_available(
        &mut self,
        cx: &mut Cx,
        current_version: &str,
        latest_version: &str,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_success_icon(cx);
            inner.set_update_actions_visible(cx, true);
            let summary = format!(
                "Current version: {current_version}\nLatest version: {latest_version}"
            );
            inner.set_content(cx, "Update available", &summary);
        }
    }
}
