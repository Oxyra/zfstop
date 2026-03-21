use crate::app::NetworkSort;
use crate::app::state::NetworkState;
use crate::net::{list_interfaces};

impl NetworkState {
    pub fn update_network(&mut self) {
        let selected_name = self.selected_interface_name();

        let mut new_interfaces = list_interfaces();

        new_interfaces.sort_by(|a, b| {
            if a.name == "lo" { return std::cmp::Ordering::Greater; }
            if b.name == "lo" { return std::cmp::Ordering::Less; }

            match self.sort {
                NetworkSort::Name => a.name.cmp(&b.name),
                NetworkSort::State => a.operstate.cmp(&b.operstate),
                NetworkSort::Mtu => a.mtu.cmp(&b.mtu),
            }
        });

        self.interfaces = new_interfaces;

        if let Some(name) = selected_name {
            if let Some(pos) = self.interfaces.iter().position(|i| i.name == name) {
                self.interface_state.select(Some(pos));
            }
        }

        if self.interface_state.selected().is_none() && !self.interfaces.is_empty() {
            self.interface_state.select(Some(0));
        }
    }


    pub fn sort_interfaces(&mut self) {
        let selected_name = self.selected_interface_name();

        self.interfaces.sort_by(|a, b| {
            // Always push lo to bottom
            if a.name == "lo" { return std::cmp::Ordering::Greater; }
            if b.name == "lo" { return std::cmp::Ordering::Less; }

            match self.sort {
                NetworkSort::Name => a.name.cmp(&b.name),
                NetworkSort::State => a.operstate.cmp(&b.operstate),
                NetworkSort::Mtu => a.mtu.cmp(&b.mtu),
            }
        });

        if let Some(name) = selected_name {
            if let Some(pos) = self.interfaces.iter().position(|i| i.name == name) {
                self.interface_state.select(Some(pos));
            }
        }
    }

    pub fn next_interface(&mut self) {
        if self.interfaces.is_empty() { return; }

        let i = self.interface_state.selected().unwrap_or(0);
        let next = (i + 1) % self.interfaces.len();

        self.interface_state.select(Some(next));
    }

    pub fn previous_interface(&mut self) {
        if self.interfaces.is_empty() { return; }

        let i = self.interface_state.selected().unwrap_or(0);
        let prev = if i == 0 {
            self.interfaces.len() - 1
        } else {
            i - 1
        };

        self.interface_state.select(Some(prev));
    }

    fn selected_interface_name(&self) -> Option<String> {
        self.interface_state
            .selected()
            .and_then(|i| self.interfaces.get(i))
            .map(|i| i.name.clone())
    }
}
