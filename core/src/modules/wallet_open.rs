use crate::imports::*;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Select,
    Unlock(Option<Arc<Error>>),
    Unlocking,
}

pub struct WalletOpen {
    #[allow(dead_code)]
    interop: Interop,
    secret: String,
    pub state: State,
    pub message: Option<String>,
    selected_wallet: Option<String>,
}

impl WalletOpen {
    pub fn new(interop: Interop) -> Self {
        Self {
            interop,
            secret: String::new(),
            state: State::Select,
            message: None,
            selected_wallet: None,
        }
    }

    pub fn lock(&mut self) {
        // Go to unlock page
        self.state = State::Unlock(None);
    }
}

impl ModuleT for WalletOpen {
    fn render(
        &mut self,
        wallet: &mut Wallet,
        _ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            let size = egui::Vec2::new(200_f32, 40_f32);
            let unlock_result = Payload::<Result<()>>::new("test");

            match self.state.clone() {
                State::Select => {
                    Panel::new(self)
                        .with_caption("Select Wallet")
                        .with_close_enabled(false, |_| {})
                        .with_header(|_ctx, ui| {
                            ui.label("Select a wallet to unlock");
                        })
                        .with_body(|this, ui| {
                            for wallet in wallet.wallet_list.iter() {
                                // let text = render_wallet_descriptor(wallet, ui);
                                let text = wallet.filename.clone();

                                // if ui.add_sized(size, egui::Button::new(wallet.filename.clone())).clicked() {
                                if ui.add_sized(size, egui::Button::new(text)).clicked() {
                                    this.selected_wallet = Some(wallet.filename.clone());
                                    this.state = State::Unlock(None);
                                }
                            }
                            ui.label(" ");
                            ui.separator();
                            ui.label(" ");
                            if ui
                                .add_sized(size, egui::Button::new("Create new wallet"))
                                .clicked()
                            {
                                // wallet.get::<section::CreateWallet>().
                                // wallet.select::<section::CreateWallet>(TypeId::of::<section::OpenWallet>());
                                wallet.select::<modules::WalletCreate>();
                            }

                            ui.label(" ");
                        })
                        .render(ui);
                }

                State::Unlock(error) => {
                    // let width = ui.available_width();
                    // let theme = theme();
                    Panel::new(self)
                        .with_caption("Unlock Wallet")
                        .with_back(|ctx| {
                            ctx.state = State::Select;
                        })
                        .with_close(|_ctx| {})
                        .with_body(|ctx, ui| {
                            // ui.label(" ");
                            ui.label(format!(
                                "Opening wallet: \"{}\"",
                                ctx.selected_wallet.as_ref().unwrap()
                            ));
                            ui.label(" ");
                            // ui.add_space(24.);

                            if let Some(err) = error {
                                // ui.horizontal(|ui| {
                                //     ui.vertical(|ui| {
                                //         ui.horizontal(|ui| {
                                //             ui.set_width(theme.error_icon_size.outer_width());
                                //             icons().error.render(ui,&theme.error_icon_size,theme.error_color);
                                //         });
                                //     });
                                //     ui.vertical(|ui| {
                                //         // ui.set_width(width-theme.error_icon_size.outer_width());
                                //         // ui.label(egui::RichText::new("Error unlocking wallet").color(egui::Color32::from_rgb(255, 120, 120)));
                                //     });
                                // });
                                ui.label(
                                    egui::RichText::new(err.to_string())
                                        .color(egui::Color32::from_rgb(255, 120, 120)),
                                );
                                ui.label(" ");
                            }

                            ui.label("Enter your password to unlock your wallet");
                            ui.label(" ");

                            let mut unlock = false;

                            if ui
                                .add_sized(
                                    size,
                                    TextEdit::singleline(&mut ctx.secret)
                                        .hint_text("Enter Password...")
                                        .password(true)
                                        .vertical_align(Align::Center),
                                )
                                .text_edit_submit(ui)
                            {
                                unlock = true;
                            }

                            if ui.add_sized(size, egui::Button::new("Unlock")).clicked() {
                                unlock = true;
                            }

                            if unlock {
                                let secret = kaspa_wallet_core::secret::Secret::new(
                                    ctx.secret.as_bytes().to_vec(),
                                );
                                ctx.secret.zeroize();
                                let wallet = ctx.interop.wallet().clone();
                                let wallet_name = ctx.selected_wallet.clone(); //.expect("Wallet name not set");

                                spawn_with_result(&unlock_result, async move {
                                    wallet.load(secret, wallet_name).await?;
                                    Ok(())
                                });

                                ctx.state = State::Unlocking;
                            }

                            ui.label(" ");
                        })
                        // .with_footer(|ui|{
                        //     if ui
                        //         .add_sized(size, egui::Button::new("Select a different wallet"))
                        //         .clicked() {
                        //             self.state = State::Select;
                        //         }
                        // })
                        .render(ui);

                    // egui::ScrollArea::vertical()
                    //     .id_source("unlock-wallet")
                    //     .show(ui, |ui| {

                    //     if ui
                    //         .add_sized(size, egui::Button::new("Select a different wallet"))
                    //         .clicked() {
                    //             self.state = State::Select;
                    //         }
                    // });
                }
                State::Unlocking => {
                    ui.heading("Unlocking");
                    // ui.separator();
                    ui.label(" ");
                    ui.label("Unlocking wallet, please wait...");
                    ui.label(" ");
                    ui.add_space(64.);
                    ui.add(egui::Spinner::new().size(92.));

                    if let Some(result) = unlock_result.take() {
                        match result {
                            Ok(_) => {
                                println!("Unlock success");
                                // self.state = State::Unlock;
                                wallet.select::<modules::AccountManager>();
                                self.state = Default::default();
                            }
                            Err(err) => {
                                println!("Unlock error: {}", err);
                                self.state = State::Unlock(Some(Arc::new(err)));
                            }
                        }
                        // ui.label(format!("Result: {:?}", result));
                        // _ctx.value = result.unwrap();
                        // Stage::Next
                    } else {
                        // Stage::Current
                    }
                }
            }
        });
    }
}

fn _render_wallet_descriptor(wallet: &WalletDescriptor, ui: &mut Ui) -> LayoutJob {
    let mut job = LayoutJob {
        halign: Align::Center,
        ..Default::default()
    };

    job.append(
        wallet
            .title
            .clone()
            .unwrap_or_else(|| "NO NAME".to_string())
            .as_str(),
        0.0,
        TextFormat {
            font_id: FontId::new(18.0, FontFamily::Proportional),
            color: ui.ctx().style().visuals.strong_text_color(), //text_color(),
            ..Default::default()
        },
    );
    //  job.append(text, leading_space, format)
    job.append(
        "\n",
        0.0,
        TextFormat {
            font_id: FontId::new(12.0, FontFamily::Proportional),
            color: ui.ctx().style().visuals.text_color(),
            ..Default::default()
        },
    );
    job.append(
        wallet.filename.clone().as_str(),
        0.0,
        TextFormat {
            font_id: FontId::new(12.0, FontFamily::Proportional),
            color: ui.ctx().style().visuals.text_color(),
            ..Default::default()
        },
    );

    job
}