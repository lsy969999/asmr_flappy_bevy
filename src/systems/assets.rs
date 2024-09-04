use bevy::prelude::*;

use crate::{resources::assets::FlappyAssets, states::my_states::MyStates};

pub fn asset_gen(
    mut assets: ResMut<FlappyAssets>,
    mut next_state: ResMut<NextState<MyStates>>,
    mut textures: ResMut<Assets<Image>>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("assets gen!");
    let bird_0 = assets.bird_orange_0.id();
    let bird_1 = assets.bird_orange_1.id();
    let bird_2 = assets.bird_orange_2.id();
    let bird_0_texture = textures.get(bird_0).unwrap();
    let bird_1_texture = textures.get(bird_1).unwrap();
    let bird_2_texture = textures.get(bird_2).unwrap();
    let (bird_at, bird_tex) = TextureAtlasBuilder::default()
        .add_texture(Some(bird_0), bird_0_texture)
        .add_texture(Some(bird_1), bird_1_texture)
        .add_texture(Some(bird_2), bird_2_texture)
        .build()
        .unwrap();
    assets.gen_bird_atlas_layout = atlases.add(bird_at);
    assets.gen_bird_atlas_texture = textures.add(bird_tex);

    let sparkle_0 = assets.spark_0.id();
    let sparkle_1 = assets.spark_1.id();
    let sparkle_2 = assets.spark_2.id();
    let sparkle_0_texture = textures.get(sparkle_0).unwrap();
    let sparkle_1_texture = textures.get(sparkle_1).unwrap();
    let sparkle_2_texture = textures.get(sparkle_2).unwrap();
    let (sparkle_at, sparkle_tex) = TextureAtlasBuilder::default()
        .add_texture(Some(sparkle_0), sparkle_0_texture)
        .add_texture(Some(sparkle_1), sparkle_1_texture)
        .add_texture(Some(sparkle_2), sparkle_2_texture)
        .build()
        .unwrap();
    assets.gen_sparkle_atlas_layout = atlases.add(sparkle_at);
    assets.gen_sparkle_atlas_texture = textures.add(sparkle_tex);
    next_state.set(MyStates::MainMenu);
}
