//! Snake abstraction.
//!
//! Provides an abstraction over a snake. Will be in charge of eating
//! food and roaming around the entire playground.
//!

use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

use ggez::graphics::{ Image, Canvas, DrawParam, Rect };
use ggez::audio::{ Source, SoundSource };
use ggez::Context;
use std::collections::{ LinkedList };

use crate::food::*;
use crate::tile::*;

/// Represents the direction the snake could potentially move to
#[derive(Copy, Clone, smart_default::SmartDefault )]
pub enum Direction {
  /// Up
  U,
  /// Down
  D,
  /// Left
  L,
  /// Right
  /// This will be the default direction when starting the game
  #[default]
  R
}

// Indicates if the snake has eaten.
// It is necessary to track down this action from the game crate as it
// will allow to handle scorekeeping.
pub enum Ate {
  Food
}

#[derive(Serialize, Deserialize, Debug)]
struct Sprite <T> {
    frames: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Frame <T> {
  #[serde(rename(deserialize = "sourceSize"))]
  source_size: T
}

#[derive(Deserialize, Debug)]
struct Size <T>{ w: T, h: T }

/// Represents the game's snake.
///
/// The head will be a tile and will help to move around the playground,
/// identify collisions, food consumption, etc.
///
/// The previous field holds the previous direction the snake was heading
/// towards.
///
/// The current direction is held in an <Option> because we would want
/// to stop the snake when colliding with itself by assigning a None.
///
/// The body is a linked list holding tiles. It is helpful when it comes
/// to adding new tiles on top of the snake.
///
/// The velocity is the directional speed the snake is currently at.
///
/// The ate field incates if the snake has eaten food or not that's why
/// an option is being used.
#[derive(smart_default::SmartDefault)]
pub struct Snake {
  pub head: Tile,
  pub previous: Direction,
  pub current_direction: Option<Direction>,
  pub body: LinkedList<Tile>,
  pub velocity: f32,
  pub ate: Option<Ate>
}


/// TODO
/// For better error reporting, create an enum to wrap all possible errors
/// Use these resources as guidelines:
/// https://stackoverflow.com/questions/73042458/how-do-i-iterate-over-a-json-list-rust
/// https://fettblog.eu/rust-enums-wrapping-errors/

impl Snake {
  /// Construct a snake representing the main character of the game.
  pub fn new() -> Snake {

    let head = Tile::new(64.0, 64.0);
    let tile = Tile::new(32.0, 64.0);

    let mut body =  LinkedList::new();
    body.push_back(tile);

    Snake {
      head,
      previous: Direction::R,
      current_direction: Some(Direction::R),
      body,
      velocity: 32.0,
      ate: None
    }
  }

  /// Draws the snake onto the canvas out of a sprite image.
  ///
  /// In order to calculate the offset for every tile reading the sprite.json
  /// file is required. The formula for the offset is the following :
  /// offset = sprite image length / tile width
  ///
  ///
  pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> () {
    let sprite: Sprite<Value> = {
      let data = fs::read_to_string("./resources/sprite.json")
        .expect("Couldn't read the .json file");

      serde_json::from_str::<Sprite<Value>>(&data).unwrap()
    };

    let frame: Frame<Value> = serde_json::from_value(
      sprite.frames.first()
      .unwrap()
      .to_owned()
    ).unwrap();

    let size: Size<Value> = serde_json::from_value(frame.source_size.to_owned())
      .unwrap();

    let offset = size.h.as_f64().unwrap() as f32 / size.w.as_f64().unwrap() as f32;

    let image = Image::from_path(ctx, "/sprite.png").unwrap();
    for square in self.body.iter() {
      let tile = square.clone();

      canvas.draw(
          &image,
          DrawParam::new()
            .src(Rect::new(
              offset * 5.0,
              0.0,
              1.0 / 6.0,
              1.0)
            )
            .dest([tile.x, tile.y])
        );
    }

    let src = match self.current_direction.unwrap() {
      Direction::U => Rect::new( offset * 2.0, 0.0, 1.0 / 6.0, 1.0 ),
      Direction::L => Rect::new( offset * 3.0, 0.0, 1.0 / 6.0, 1.0 ),
      Direction::R => Rect::new( offset * 4.0, 0.0, 1.0 / 6.0, 1.0 ),
      Direction::D => Rect::new( offset * 1.0, 0.0, 1.0 / 6.0, 1.0 ),
    };

    canvas.draw(
        &image,
        DrawParam::new()
          .src(src)
          .dest([self.head.x, self.head.y])
    );
  }

  /// Updates the snake's position based on its current direction.
  ///
  /// Handles the movement logic around the playground and ensures
  /// that whenever the snake surpasses the edge border it will
  /// continue its course
  pub fn update(&mut self, food: &mut Food, ctx: &mut Context) {
    if self.current_direction.is_some() {
      self.body.push_front(self.head);
      match self.current_direction.unwrap() {
        Direction::R => {
          self.head.move_x(self.velocity);
          if self.head.x >= 960.0 { self.head.x = 0.0; }
        }
        Direction::L => {
          self.head.move_x(self.velocity * -1.0);
          if self.head.x <= -32.0 { self.head.x = 928.0; }
        }
        Direction::U => {
          self.head.move_y(self.velocity * -1.0);
          if self.head.y <= 64.0 {
            self.head.y = 928.0;
          }
        }
        Direction::D => {
          self.head.move_y(self.velocity);
          if self.head.y >= 960.0 {
            self.head.y = 96.0;
          }
        }
      }
      if self.eats(&food){
        self.ate = Some(Ate::Food);
        let mut sound = Source::new(ctx, "/sound.wav").unwrap();
        sound.play_detached(ctx).unwrap();
        food.serve();

      } else {
        self.body.pop_back();
      }
    }
  }

  /// Indicates if the snake ate or not by comparing its head's position
  /// with that of the food.
  ///
  /// Since both the head and the food are the same type and the
  /// partialeq trait was implemented comparing one against the other is
  /// utterly possible.
  fn eats(&mut self, food: &Food) -> bool {
    self.head == food.piece
  }

  /// Changes the snake's direction.
  ///
  /// It's simply setting the direction-related attributes fo the snake.
  /// For the time being an option is being used on the current_direction
  /// attribute. That is because we'd want to set it to none once the
  /// snake collides with itself.
  pub fn change_direction(&mut self, direction: Direction) {
    self.current_direction = Some(direction);
    self.previous = direction;
  }

  /// Returns a boolean indicating whether the snake collided or not.
  ///
  /// Since the snake's body is a linked list of tiles, the partialeq
  /// trait implementation is being taken advantage of in order to
  /// compare the body position with that of the head.
  pub fn collides(&mut self) -> bool {
    self.body.iter().any(|&x| x == self.head )
  }
}