pub mod cimahi;

use amethyst::prelude::*;

use reqwest::header;

use serde_derive::{Deserialize};

use crate::config::Request;
use crate::model::token::Token;


use crate::model::character::CharacterPosition;

#[derive(Deserialize)]
pub struct Town {
    id: usize,
    name: String,
    characters: Vec<CharacterPosition>
}

trait IsTown : super::has_characters::HasCharacters + super::has_chat::HasChat {
    fn get_town_id(&self) -> usize;

    fn init_town(&mut self, world: &mut World) {
        let resp = self.request_town(world);
        match resp {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    let town: Town = resp.json().unwrap();
                    self.set_characters_position(town.characters);
                } else if resp.status().is_server_error() {
                    panic!()
                } else {
                    panic!()
                }
            },
            Err(_) => panic!()
        };

        self.init_chat_ui(world);
        self.init_characters_ui(world);
    }

    fn request_town(&self, world: &mut World) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let config = world.read_resource::<Request>();
        let uri = format!("{}{}{}", config.api_url, "/towns/", self.get_town_id());

        let mut headers = header::HeaderMap::new();
        let token = format!("Bearer {}", world.read_resource::<Token>().get_token());
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&token).unwrap());

        reqwest::Client::builder()
            .default_headers(headers)
            .build()?
            .get(&uri)
            .send()
    }
}
