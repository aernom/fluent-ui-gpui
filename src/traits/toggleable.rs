pub trait Toggleable {
    fn toggle_state(self, selected: bool) -> Self;
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ToggleState {
    #[default]
    Unselected,
    Indeterminate,
    Selected,
}

impl ToggleState {
    pub fn inverse(&self) -> Self {
        match self {
            Self::Unselected | Self::Indeterminate => Self::Selected,
            Self::Selected => Self::Unselected,
        }
    }
}

impl From<bool> for ToggleState {
    fn from(selected: bool) -> Self {
        if selected {
            Self::Selected
        } else {
            Self::Unselected
        }
    }
}

impl From<Option<bool>> for ToggleState {
    fn from(selected: Option<bool>) -> Self {
        match selected {
            Some(true) => Self::Selected,
            Some(false) => Self::Unselected,
            None => Self::Unselected,
        }
    }
}
