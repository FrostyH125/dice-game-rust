use raylib::{color::Color, math::Rectangle, prelude::{RaylibDraw, RaylibDrawHandle}};

use crate::system::input_handler::{InputState, MouseState};

// responsible for moving through current dialogue and managing whether its finished or not
// responsible for displaying the dialogue
pub struct DialogueSystem {
    current_text_index: usize,
    current_dialogue: Option<Dialogue>,
}

impl DialogueSystem {
    pub fn new() -> Self {
        Self {
            current_text_index: 0,
            current_dialogue: None,
        }
    }
    
    pub fn update(&mut self, input: &InputState) {
        if let Some(dialogue) = &self.current_dialogue {
            if let MouseState::Clicked = input.mouse_state {
                self.current_text_index += 1;
                
                let number_of_texts = dialogue.text_blocks.len();
                
                if self.current_text_index >= number_of_texts {
                    self.current_dialogue = None;
                }
            }
        }
    }
    
    // 480 x 270
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        
        if let None = self.current_dialogue {
            return;
        }
        
        d.draw_rectangle(30, 170, 420, 90, Color::BLACK);
        d.draw_rectangle_lines_ex(Rectangle::new(28.0, 168.0, 424.0, 92.0), 2.0, Color::WHITE);
        
        // next goal, draw the text. should be easy but will need to use the text wrapping function, perhaps you can store it in a variable
        // in order to not need to recalculate it every frame, even if its cheap and wouldnt take very long, theres no reason to
        
    }
    
    pub fn add_dialogue(&mut self, dialogue: Dialogue) {
        self.current_dialogue = Some(dialogue);
    }
    
    pub fn is_active(&mut self) -> bool {
        return self.current_dialogue.is_some();
    }
}

// a simple struct that can be amended depending on what the game should need. possible amendments are things like special per dialogue effects, for example
// i could make Dialogue a collection of like DialogueNodes or something but i think for right now im just going to keep things simple and do a Vec<(String, etc, etc)>, just
// <String> to start though. the etc would be like, Vec<(String, TextEffect, SoundEffect)>, for example
pub struct Dialogue {
    pub text_blocks: Vec<String>,
}
