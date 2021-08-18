use crate::gui::style;
use crate::core::uad_lists::{ UadLists, PackageState };

use iced::{
    scrollable, Align, Column, Command, Container, Element, Space,
    Length, Row, Scrollable, Text, text_input, TextInput,
    PickList, pick_list,
};

use crate::core::sync::list_phone_packages;

#[derive(Default, Debug, Clone)]
pub struct List {
    p_row: Vec<PackageRow>,
    packages: String,
    input: text_input::State,
    package_scrollable_state: scrollable::State,
    package_state_picklist: pick_list::State<PackageState>,
    list_picklist: pick_list::State<UadLists>,
    selected_package_state: Option<PackageState>,
    selected_list: Option<UadLists>,
    pub input_value: String,
}

/*impl Default for List {
    fn default() -> Self {
        List { ..List::default() }
    }
}*/


#[derive(Debug, Clone)]
pub enum Message {
    ListInputChanged(String),
    LoadPackages,
    ListSelected(UadLists),
    PackageStateSelected(PackageState)
}


impl List {
    pub fn update(&mut self, message: Message) -> Command<Message> {
       match message {
            Message::ListInputChanged(_letter) => {
                Command::none()
            }

            Message::LoadPackages => {
                self.packages = test();
                self.p_row = Vec::new();
                for p_name in self.packages.lines() {
                    let package_row = PackageRow::new(
                        &p_name,
                        "Installed",
                        "REMOVE",
                    );
                    self.p_row.push(package_row)
                }
                self.p_row.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

                Command::none()
            }

            Message::ListSelected(list) => {
                self.selected_list = Some(list);
                Command::none()
            }

            Message::PackageStateSelected(package_state) => {
                self.selected_package_state = Some(package_state);
                Command::none()
            }
        }
    }
    pub fn view(&mut self) -> Element<Message> {

                let search_packages = TextInput::new(
                    &mut self.input,
                    "Search packages...",
                    &mut self.input_value,
                    Message::ListInputChanged,
                )
                .padding(5);

                // let package_amount = Text::new(format!("{} packages found", packages.len()));

                let divider = Space::new(Length::Fill, Length::Shrink);

                let list_picklist = PickList::new(
                            &mut self.list_picklist,
                            &UadLists::ALL[..],
                            self.selected_list,
                            Message::ListSelected,
                        );

                let package_state_picklist = PickList::new(
                            &mut self.package_state_picklist,
                            &PackageState::ALL[..],
                            self.selected_package_state,
                            Message::PackageStateSelected,
                        );

                let control_panel = Row::new()
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .spacing(10)
                    .push(search_packages)
                    .push(divider)
                    .push(package_state_picklist)
                    .push(list_picklist);

                let package_name = Text::new("Package").width(Length::FillPortion(6));
                let package_state = Text::new("State").width(Length::FillPortion(3));
                let advice = Text::new("Advice").width(Length::FillPortion(3));

                let package_panel = Row::new()
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .push(package_name)
                    .push(package_state)
                    .push(advice);
                    
                // let mut packages_v: Vec<&str> = self.packages.lines().collect();

                let test = self.p_row
                    .iter_mut()
                    .enumerate()
                    .fold(Column::new().spacing(5), |col, (_, p)| {
                        col.push(p.view())
                    });

                let packages_scrollable = Scrollable::new(&mut self.package_scrollable_state)
                    .push(test)
                    .spacing(5)
                    .align_items(Align::Center)
                    .style(style::Scrollable);

                let content = Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(control_panel)
                    .push(package_panel)
                    .push(packages_scrollable);

                Container::new(content)
                    .height(Length::Fill)
                    .padding(10)
                    .style(style::Content)
                    .into()
    }
}

#[derive(Clone, Debug)]
pub struct PackageRow {
    pub name: String,
    pub state: String,
    pub advice: String,
}

#[derive(Clone, Debug)]
pub enum RowMessage {
    NoEvent,
}

impl PackageRow {
    pub fn new(
        name: &str,
        state: &str,
        advice: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            state: "Installed".to_string(),
            advice: advice.to_string(),
        }
    }

/*    pub fn update(&mut self, message: RowMessage) -> Command<RowMessage> {
        match message {
            RowMessage::NoEvent => Command::none(),
        }
    }*/

    pub fn view(&mut self) -> Element<Message> {

        let content = Row::new()
            .align_items(Align::Center)
            .push(Text::new(&self.name).width(Length::FillPortion(6)))
            .push(Text::new(&self.state).width(Length::FillPortion(3)))
            .push(Text::new(&self.advice).width(Length::FillPortion(3)));

        let p_row = Container::new(content)
            .padding(10)
            .style(style::PackageRow);

        Column::new().push(p_row).into()


    }
}


pub fn test() -> String {
        list_phone_packages()
}