use crate::cheese::features;
use crate::cheese::gui::entry::TheCheese;
use crate::cheese::gui::menu_definition::{MenuDefinition, MenuEntry};

pub const MAIN_MENU_ID: u32 = 0;
const SETTINGS_MENU_ID: u32 = 100;
const VEHICLE_MENU_ID: u32 = 200;
const VEHICLE_HANDLING_MENU_ID: u32 = 201;
const PLAYER_MENU_ID: u32 = 300;

impl TheCheese {
    pub(crate) fn setup_menus(&mut self) {
        self.setup_vehicle_menu();
        self.setup_main_menu();
        self.setup_vehicle_handling();
        self.setup_player_menu();
        self.current_menu_id = 0;
    }
    
    fn setup_main_menu(&mut self) {
        let mut def = MenuDefinition::new("Main Menu");
        def.entries.clear();
        
        def.entries.push(MenuEntry::Redirect { name: "Vehicle Mods".to_string(), to_menu_id: VEHICLE_MENU_ID });
        def.entries.push(MenuEntry::Redirect { name: "Player Mods".to_string(), to_menu_id: PLAYER_MENU_ID});
        
        self.menu_definitions.insert(MAIN_MENU_ID, def);
    }

    fn setup_vehicle_menu(&mut self) {
        let mut def = MenuDefinition::new("Vehicle Mods");
       
        def.entries.push(MenuEntry::State { name: "Seatbelt".to_string(), state: self.seatbelt.clone() });
        def.entries.push(MenuEntry::Redirect { name: "Handling".to_string(), to_menu_id: VEHICLE_HANDLING_MENU_ID});
        
        self.menu_definitions.insert(VEHICLE_MENU_ID, def);
    }

    fn setup_vehicle_handling(&mut self) {
        let mut def = MenuDefinition::new("Handling");
        
        def.entries.clear();
        
        def.entries.push(MenuEntry::Redirect { name: "<< Back".to_string(), to_menu_id: VEHICLE_MENU_ID});
        def.entries.push(MenuEntry::State { name: "Placeholder1".to_string(), state: self.seatbelt.clone() });
        def.entries.push(MenuEntry::State { name: "Placeholder2".to_string(), state: self.seatbelt.clone() });
        def.entries.push(MenuEntry::State { name: "Placeholder3".to_string(), state: self.seatbelt.clone() });
        def.entries.push(MenuEntry::State { name: "Placeholder4".to_string(), state: self.seatbelt.clone() });
        def.entries.push(MenuEntry::State { name: "Placeholder5".to_string(), state: self.seatbelt.clone() });
        def.entries.push(MenuEntry::State { name: "Placeholder6".to_string(), state: self.seatbelt.clone() });
        def.entries.push(MenuEntry::State { name: "Placeholder7".to_string(), state: self.seatbelt.clone() });

        self.menu_definitions.insert(VEHICLE_HANDLING_MENU_ID, def);
    }
    
    fn setup_player_menu(&mut self) {
        let mut def = MenuDefinition::new("Player Mods");

        def.entries.push(MenuEntry::State { name: "Godmode".to_string(), state: self.godmode.clone() });
        def.entries.push(MenuEntry::Execute { name: "Kill".to_string(), function: features::player::kill});

        self.menu_definitions.insert(PLAYER_MENU_ID, def);
    }
}