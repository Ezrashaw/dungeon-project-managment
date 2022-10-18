use lazy_static::lazy_static;
use macroquad::prelude::*;

lazy_static! {
    pub static ref ARMOUR_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/armored_ferris-card.png"),
        Some(ImageFormat::Png),
    );
    pub static ref CRAB_FOOD: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/crab-food.png"),
        Some(ImageFormat::Png),
    );
    pub static ref SUPPLEMENTS: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/supplements.png"),
        Some(ImageFormat::Png),
    );
    pub static ref CARDS: [Card; 3] = [
        Card {
            title: "Stronger Armor".to_string(),
            card_type: CardType::TempDamageReduction,
            card_width: 300.,
            card_height: 300. * 1.618_034,
            image: *ARMOUR_TEXTURE,
            description: "Take 1 less damage from each enemy attack for the next brawl."
                .to_string(),
        },
        Card {
            title: "Crab Food".to_string(),
            card_type: CardType::TempHeal,
            card_width: 300.,
            card_height: 300. * 1.618_034,
            image: *CRAB_FOOD,
            description: "Restore 40 health".to_string(),
        },
        Card {
            title: "Supplements".to_string(),
            card_type: CardType::TempDamageReduction,
            card_width: 300.,
            card_height: 300. * 1.618_034,
            image: *SUPPLEMENTS,
            description: "You need to type 10 fewer characters in the next brawl".to_string(),
        },
    ];
    pub static ref PERMANENT_CARDS: [Card; 3] = [
        Card {
            title: "Strengthened Exoskeleton".to_string(),
            card_type: CardType::TempDamageReduction,
            card_width: 300.,
            card_height: 300. * 1.618_034,
            image: *SUPPLEMENTS,
            description: "You take less damage from enemy attacks".to_string(),
        },
        Card {
            title: "Bulked Up".to_string(),
            card_type: CardType::TempHeal,
            card_width: 300.,
            card_height: 300. * 1.618_034,
            image: *SUPPLEMENTS,
            description: "Increases your max health.".to_string(),
        },
        Card {
            title: "Sharpened Pincers".to_string(),
            card_type: CardType::TempDamageReduction,
            card_width: 300.,
            card_height: 300. * 1.618_034,
            image: *SUPPLEMENTS,
            description: "You need fewer characters to defeat any enemy".to_string(),
        },
    ];
}
pub fn load_treasure_images() {
    let _ = *ARMOUR_TEXTURE;
    println!("Armour card texture loaded");
    let _ = *CRAB_FOOD;
    println!("Crab food card texture loaded");
    let _ = *SUPPLEMENTS;
    println!("Supplement card texture loaded");
}

#[derive(Clone)]
pub struct Card {
    pub title: String,
    pub card_type: CardType,
    pub image: Texture2D,
    pub description: String,
    pub card_width: f32,
    pub card_height: f32,
}

#[derive(Clone)]
pub enum CardType {
    TempHeal,
    TempWordsReduce,
    TempDamageReduction,
}

impl Card {
    pub fn draw_card(&self, x: f32, y: f32) {
        draw_rectangle(x, y, self.card_width, 0.15 * self.card_height, WHITE);
        draw_rectangle_lines(x, y, self.card_width, 0.15 * self.card_height, 5., BLACK);
        let td = measure_text(&self.title[..], None, 40, 1.0);
        draw_text_ex(
            &self.title[..],
            x - (td.width - self.card_width) / 2.,
            y + (0.15 * self.card_height) / 2. + td.height / 4.,
            TextParams {
                font_size: 40,
                font_scale: 1.0,
                color: BLACK,
                font_scale_aspect: 1.0,
                ..Default::default()
            },
        );
        draw_rectangle(
            x,
            y + 0.15 * self.card_height,
            self.card_width,
            self.card_height / 2.,
            WHITE,
        );

        draw_texture_ex(
            self.image,
            x,
            y + 0.15 * self.card_height,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::from([self.card_width, self.card_height / 2.])),
                ..Default::default()
            },
        );
        let text_box_y = y + 0.15 * self.card_height + self.card_height / 2.;
        let text_box_height = self.card_height - 0.15 * self.card_height - self.card_height / 2.;

        draw_rectangle(x, text_box_y, self.card_width, text_box_height, WHITE);

        draw_rectangle_lines(x, text_box_y, self.card_width, text_box_height, 5., BLACK);
        self.add_description(x + 20., text_box_y + 20., self.card_width - 20.);
        draw_rectangle_lines(x, y, self.card_width, self.card_height, 5., BLACK);
    }

    pub fn add_description(&self, x: f32, y: f32, width: f32) {
        let font_size = 30.;
        let mut num_lines: usize = 1;
        let words: Vec<&str> = self.description.split(' ').collect();
        let mut line: Vec<&str> = Vec::new();
        let mut temp_line = line.clone();
        for word in words {
            temp_line.push(word);
            if measure_text(&temp_line.join(" ")[..], None, 1, font_size).width > width {
                draw_text_ex(
                    &line.join(" "),
                    x,
                    y + font_size * num_lines as f32,
                    TextParams {
                        font_size: 30,
                        font_scale: 1.,
                        font_scale_aspect: 1.,
                        color: BLACK,
                        ..Default::default()
                    },
                );
                line = vec![word];
                temp_line = line.clone();
                num_lines += 1;
            } else {
                line.push(word);
            }
        }
        if line.concat() != "" {
            draw_text(
                &line.join(" ")[..],
                x,
                y + font_size * num_lines as f32,
                font_size,
                BLACK,
            );
        }
    }
}

#[must_use]
pub fn card_select(cards_and_coords: &[(Card, (f32, f32))]) -> Option<&Card> {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (x_pos, y_pos) = mouse_position();
        for (card, (x, y)) in cards_and_coords.iter() {
            if (x_pos >= *x && x_pos <= *x + card.card_width)
                && (y_pos >= *y && y_pos <= *y + card.card_height)
            {
                println!("{}", card.title);
                return Some(card);
            }
        }
        println!("No card selected");
    }

    None
}
