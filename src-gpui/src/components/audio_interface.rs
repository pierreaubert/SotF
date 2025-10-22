use crate::components::routing_matrix::RoutingMatrixComponent;
use crate::design::{colors, fonts, spacing, RADIUS};
use autoeq_backend::audio::{get_audio_devices, AudioDevice as AudioDeviceInfo};
use gpui::*;

pub struct AudioInterfaceComponent {
    devices: Vec<AudioDeviceInfo>,
    selected_output_device: Option<usize>,
    selected_input_device: Option<usize>,
    show_device_dropdown: bool,
    device_dropdown_type: DeviceType,
    routing_matrix: Entity<RoutingMatrixComponent>,
}

#[derive(Clone, Copy, PartialEq)]
enum DeviceType {
    Output,
    Input,
}

impl AudioInterfaceComponent {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let devices = match get_audio_devices() {
            Ok(device_map) => {
                let mut all_devices = Vec::new();
                if let Some(input_devices) = device_map.get("input") {
                    all_devices.extend(input_devices.iter().cloned());
                }
                if let Some(output_devices) = device_map.get("output") {
                    all_devices.extend(output_devices.iter().cloned());
                }
                all_devices
            }
            Err(e) => {
                log::error!("Failed to get audio devices: {}", e);
                Vec::new()
            }
        };

        // Find default devices
        let selected_output = devices.iter().position(|d| !d.is_input && d.is_default);
        let selected_input = devices.iter().position(|d| d.is_input && d.is_default);

        log::info!("Loaded {} audio devices", devices.len());
        if let Some(idx) = selected_output {
            log::info!("Default output: {}", devices[idx].name);
        }
        if let Some(idx) = selected_input {
            log::info!("Default input: {}", devices[idx].name);
        }

        let routing_matrix = cx.new(|_cx| RoutingMatrixComponent::new(2));

        Self {
            devices,
            selected_output_device: selected_output,
            selected_input_device: selected_input,
            show_device_dropdown: false,
            device_dropdown_type: DeviceType::Output,
            routing_matrix,
        }
    }

    pub fn demo_label() -> &'static str {
        "audio_interface"
    }

    fn refresh_devices(&mut self, cx: &mut Context<Self>) {
        match get_audio_devices() {
            Ok(device_map) => {
                let mut all_devices = Vec::new();
                if let Some(input_devices) = device_map.get("input") {
                    all_devices.extend(input_devices.iter().cloned());
                }
                if let Some(output_devices) = device_map.get("output") {
                    all_devices.extend(output_devices.iter().cloned());
                }
                self.devices = all_devices;
                // Reselect defaults if current selection is invalid
                if let Some(idx) = self.selected_output_device {
                    if idx >= self.devices.len() || self.devices[idx].is_input {
                        self.selected_output_device = self
                            .devices
                            .iter()
                            .position(|d| !d.is_input && d.is_default);
                    }
                }
                if let Some(idx) = self.selected_input_device {
                    if idx >= self.devices.len() || !self.devices[idx].is_input {
                        self.selected_input_device =
                            self.devices.iter().position(|d| d.is_input && d.is_default);
                    }
                }
                log::info!("Refreshed audio devices: {} found", self.devices.len());
                cx.notify();
            }
            Err(e) => {
                log::error!("Failed to refresh audio devices: {}", e);
            }
        }
    }

    fn get_output_devices(&self) -> Vec<(usize, &AudioDeviceInfo)> {
        self.devices
            .iter()
            .enumerate()
            .filter(|(_, d)| !d.is_input)
            .collect()
    }

    fn get_input_devices(&self) -> Vec<(usize, &AudioDeviceInfo)> {
        self.devices
            .iter()
            .enumerate()
            .filter(|(_, d)| d.is_input)
            .collect()
    }

    fn select_device(&mut self, idx: usize, is_input: bool, cx: &mut Context<Self>) {
        if is_input {
            self.selected_input_device = Some(idx);
            log::info!("Selected input device: {}", self.devices[idx].name);
        } else {
            self.selected_output_device = Some(idx);
            log::info!("Selected output device: {}", self.devices[idx].name);
        }
        cx.notify();
    }

    fn render_device_list(&self, is_input: bool, cx: &mut Context<Self>) -> Div {
        let devices: Vec<(usize, &AudioDeviceInfo)> = self
            .devices
            .iter()
            .enumerate()
            .filter(|(_, d)| d.is_input == is_input)
            .collect();

        let selected_idx = if is_input {
            self.selected_input_device
        } else {
            self.selected_output_device
        };

        if devices.is_empty() {
            return div()
                .p_4()
                .text_sm()
                .text_color(rgb(0x999999))
                .italic()
                .child(format!(
                    "No {} devices found",
                    if is_input { "input" } else { "output" }
                ));
        }

        div()
            .flex()
            .flex_col()
            .gap_1()
            .children(devices.iter().map(|(idx, device)| {
                let is_selected = selected_idx == Some(*idx);
                let idx_copy = *idx;

                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .rounded(px(4.0))
                    .bg(if is_selected {
                        rgb(0xe3f2fd)
                    } else {
                        rgb(0xffffff)
                    })
                    .border_1()
                    .border_color(if is_selected {
                        rgb(0x2196f3)
                    } else {
                        rgb(0xdddddd)
                    })
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(if is_selected {
                            rgb(0xbbdefb)
                        } else {
                            rgb(0xf5f5f5)
                        })
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.select_device(idx_copy, is_input, cx);
                        }),
                    )
                    .child(div().w(px(20.0)).text_center().child(if is_selected {
                        "●"
                    } else {
                        "○"
                    }))
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .font_weight(if device.is_default {
                                        FontWeight::BOLD
                                    } else {
                                        FontWeight::NORMAL
                                    })
                                    .text_color(rgb(0x333333))
                                    .child(format!(
                                        "{}{}",
                                        device.name,
                                        if device.is_default { " (default)" } else { "" }
                                    )),
                            )
                            .child(div().text_xs().text_color(rgb(0x666666)).child(
                                if let Some(ref config) = device.default_config {
                                    format!("{} ch @ {} Hz", config.channels, config.sample_rate)
                                } else {
                                    "No default config".to_string()
                                },
                            )),
                    )
            }))
    }

    fn render_device_details(
        &self,
        device_opt: Option<&AudioDeviceInfo>,
        is_input: bool,
        cx: &mut Context<Self>,
    ) -> Div {
        if let Some(device) = device_opt {
            if let Some(ref config) = device.default_config {
                let channels = config.channels;
                div()
                    .flex()
                    .flex_col()
                    .gap(spacing::XS)
                    .child(
                        // Device name
                        div()
                            .text_size(fonts::SIZE_BASE)
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(colors::text_primary())
                            .child(device.name.clone()),
                    )
                    .child(
                        // Device specs row
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .items_center()
                            .child(
                                // Channels
                                div()
                                    .px(spacing::SM)
                                    .py(px(2.0))
                                    .rounded(px(3.0))
                                    .bg(colors::bg_accent())
                                    .text_size(fonts::SIZE_SM)
                                    .text_color(colors::text_secondary())
                                    .child(format!("{}ch", config.channels)),
                            )
                            .child(
                                // Sample rate
                                div()
                                    .px(spacing::SM)
                                    .py(px(2.0))
                                    .rounded(px(3.0))
                                    .bg(colors::bg_accent())
                                    .text_size(fonts::SIZE_SM)
                                    .text_color(colors::text_secondary())
                                    .child(format!("{}kHz", config.sample_rate as f32 / 1000.0)),
                            )
                            .child(
                                // Sample format instead of bit depth
                                div()
                                    .px(spacing::SM)
                                    .py(px(2.0))
                                    .rounded(px(3.0))
                                    .bg(colors::bg_accent())
                                    .text_size(fonts::SIZE_SM)
                                    .text_color(colors::text_secondary())
                                    .child(config.sample_format.clone()),
                            )
                            .child(
                                // Routing matrix icon button
                                div()
                                    .px(spacing::SM)
                                    .py(px(2.0))
                                    .rounded(px(3.0))
                                    .bg(colors::bg_accent())
                                    .cursor_pointer()
                                    .hover(|s| s.bg(colors::hover_bg()))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            // Update routing matrix channel count and toggle visibility
                                            this.routing_matrix.update(cx, |matrix, cx| {
                                                matrix.set_channel_count(channels as usize, cx);
                                                matrix.toggle_visibility(cx);
                                            });
                                        }),
                                    )
                                    .child(
                                        div()
                                            .text_size(fonts::SIZE_SM)
                                            .text_color(colors::text_secondary())
                                            .child("⊞"),
                                    ),
                            ),
                    )
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap(spacing::XS)
                    .child(
                        // Device name
                        div()
                            .text_size(fonts::SIZE_BASE)
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(colors::text_primary())
                            .child(device.name.clone()),
                    )
                    .child(
                        div()
                            .text_size(fonts::SIZE_SM)
                            .text_color(colors::text_secondary())
                            .child("No default configuration available"),
                    )
            }
        } else {
            div()
                .px(spacing::MD)
                .py(spacing::SM)
                .rounded(RADIUS)
                .border_1()
                .border_color(colors::border())
                .bg(colors::bg_secondary())
                .text_size(fonts::SIZE_BASE)
                .text_color(colors::text_secondary())
                .italic()
                .child(format!(
                    "No {} device selected",
                    if is_input { "input" } else { "output" }
                ))
        }
    }
}

impl Render for AudioInterfaceComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let output_device = self
            .selected_output_device
            .and_then(|idx| self.devices.get(idx));
        let input_device = self
            .selected_input_device
            .and_then(|idx| self.devices.get(idx));

        div()
            .flex()
            .flex_col()
            .gap(spacing::MD)
            .child(
                // Header with refresh button
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .bg(colors::bg_accent())
                    .px(spacing::MD)
                    .py(spacing::SM)
                    .border_b_1()
                    .border_color(colors::border())
                    .child(
                        div()
                            .text_size(fonts::SIZE_MD)
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors::text_primary())
                            .child("Audio Interface Configuration"),
                    )
                    .child(
                        div()
                            .px(spacing::MD)
                            .py(spacing::SM)
                            .rounded(RADIUS)
                            .bg(colors::button_primary())
                            .text_color(rgb(0xffffff))
                            .text_size(fonts::SIZE_BASE)
                            .font_weight(FontWeight::SEMIBOLD)
                            .cursor_pointer()
                            .hover(|s| s.bg(colors::button_primary_hover()))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.refresh_devices(cx);
                                }),
                            )
                            .child("🔄 Refresh"),
                    ),
            )
            .child(
                // Output device
                div()
                    .flex()
                    .flex_col()
                    .gap(spacing::SM)
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors::text_primary())
                            .text_size(fonts::SIZE_SM)
                            .child("Output Device:"),
                    )
                    .child(self.render_device_details(output_device, false, cx)),
            )
            .child(
                // Input device
                div()
                    .flex()
                    .flex_col()
                    .gap(spacing::SM)
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors::text_primary())
                            .text_size(fonts::SIZE_SM)
                            .child("Input Device:"),
                    )
                    .child(self.render_device_details(input_device, true, cx)),
            )
            .child(
                // Device summary
                div()
                    .p_4()
                    .rounded(px(4.0))
                    .bg(rgb(0xf9f9f9))
                    .border_1()
                    .border_color(rgb(0xeeeeee))
                    .text_sm()
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Total devices: {} ({} output, {} input)",
                        self.devices.len(),
                        self.get_output_devices().len(),
                        self.get_input_devices().len()
                    )),
            )
            .child(
                // Output device list
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .mt_4()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x333333))
                            .child("Available Output Devices:"),
                    )
                    .child(self.render_device_list(false, cx)),
            )
            .child(
                // Input device list
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .mt_4()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x333333))
                            .child("Available Input Devices:"),
                    )
                    .child(self.render_device_list(true, cx)),
            )
            .child(
                // Routing matrix overlay (only visible when toggled)
                self.routing_matrix.clone(),
            )
    }
}
