// This enum causes issues for VS Code so all are commented except when used.

// Generated from https://github.com/microsoft/WinUI-Gallery/blob/main/WinUIGallery/DataModel/IconsData.json

pub enum Text {
    /* MenuBar */
    File,

    /* Sidebar */
    Home,
    // ParkingLocation,
    // MapCompassTop,
    // MapCompassBottom,
    // IncidentTriangle,
    // Touch,
    // MapDirections,
    // StartPoint,
    // StopPoint,
    // EndPoint,
    // History,
    // Location,
    // MapLayers,
    // Accident,
    // Work,
    // Construction,
}

impl Text {
    pub const fn name(&self) -> char {
        match self {
            /* MenuBar Name */
            Text::File => 'File',

            /* Sidebar items */
            Text::Home => 'Home',


            // Icon::PhoneBook => '\u{E780}',
            // Icon::LEDLight => '\u{E781}',
            // Icon::Error => '\u{E783}',
            // Icon::GripperBarVertical => '\u{E784}',
            // Icon::Unlock => '\u{E785}',
            // Icon::GripperResize => '\u{E788}',
            // Icon::Megaphone => '\u{E789}',
            // Icon::Trim => '\u{E78A}',
            // Icon::Speakers => '\u{E7F5}',
            // Icon::Headphone => '\u{E7F6}',
            // Icon::DeviceLaptopPic => '\u{E7F7}',
            // Icon::DeviceLaptopNoPic => '\u{E7F8}',
        }
    }
}
