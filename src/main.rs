#[macro_use]
extern crate chan;

use dbus::{blocking::Connection, Path};
use dbus_crossroads::{Context, Crossroads, IfaceBuilder};
use std::{error::Error, vec};

mod metadata;
mod player;
use crate::metadata::Metadata;
use crate::player::Player;

fn main() -> Result<(), Box<dyn Error>> {
    let c = Connection::new_session()?;
    c.request_name("org.mpris.MediaPlayer2.spotify", false, true, false)?;

    let mut cr = Crossroads::new();

    let media_player_token =
        cr.register("org.mpris.MediaPlayer2", |b: &mut IfaceBuilder<Player>| {
            b.property("SupportedMimeTypes").get(|_, _| {
                let v: Vec<String> = vec![];
                Ok(v)
            });

            b.property("SupportedUriSchemes")
                .get(|_, _| Ok(vec!["spotify".to_string()]));

            b.property("CanQuit").get(|_, _| Ok(false));

            b.property("CanRaise").get(|_, _| Ok(false));

            b.property("HasTrackList").get(|_, _| Ok(false));

            b.property("DesktopEntry")
                .get(|_, _| Ok("spotify".to_string()));

            b.property("Identity").get(|_, _| Ok("Spotify".to_string()));

            b.method("Quit", (), (), move |_, _, (): ()| Ok(()));

            b.method("Raise", (), (), move |_, _, (): ()| Ok(()));
        });

    let player_token = cr.register(
        "org.mpris.MediaPlayer2.Player",
        |b: &mut IfaceBuilder<Player>| {
            b.property("CanControl").get(|_, _| Ok(true));

            b.property("CanGoNext").get(|_, _| Ok(true));

            b.property("CanGoPrevious").get(|_, _| Ok(true));

            b.property("CanPause").get(|_, _| Ok(true));

            b.property("CanPlay").get(|_, _| Ok(true));

            b.property("CanSeek").get(|_, _| Ok(true));

            b.property("Shuffle")
                .get(|_, player| Ok(player.shuffle))
                .set(|_, player, value| {
                    player.shuffle = value;
                    Ok(Some(value))
                });

            b.property("Metadata")
                .get(|_, player| Ok(player.current_metadata().to_map()));

            b.property("MaximumRate").get(|_, _| Ok(1.0));

            b.property("MinimumRate").get(|_, _| Ok(1.0));

            b.property("Rate")
                .get(|_, player| Ok(player.rate))
                .set(|_, player, value| {
                    player.rate = value;
                    Ok(Some(value))
                });

            b.property("Volume")
                .get(|_, player| Ok(player.volume))
                .set(|_, player, value| {
                    player.volume = value;
                    Ok(Some(value))
                });

            b.property("Position")
                .get(|_, player| Ok(player.get_position()));

            b.property("LoopStatus")
                .get(|_, player| Ok(player.loop_status.clone()))
                .set(|_, player, value| {
                    player.loop_status = value.clone();
                    Ok(Some(value.clone()))
                });

            b.property("PlaybackStatus")
                .get(|_, player| Ok(player.playback_status()));

            let seeked = b.signal::<(u64,), _>("Seeked", ("sender",)).msg_fn();

            b.method(
                "Seek",
                ("Offset",),
                (),
                move |ctx: &mut Context, player: &mut Player, (offset,): (i64,)| {
                    // Seeek by offset
                    player.seek(offset);
                    println!(
                        "Seeking by {} (new position: {})",
                        offset,
                        player.get_position()
                    );

                    // send signal to let peers know we seeked
                    let signal_msg = seeked(ctx.path(), &(player.get_position(),));
                    ctx.push_msg(signal_msg);

                    Ok(())
                },
            );

            b.method("Next", (), (), move |_, player, (): ()| {
                player.next();
                Ok(())
            });

            b.method("OpenUri", ("Uri",), (), move |_, _, (uri,): (String,)| {
                println!("open_uri: {}", uri);
                Ok(())
            });

            b.method("Pause", (), (), move |_, player, (): ()| {
                player.set_playing(false);
                Ok(())
            });

            b.method("Play", (), (), move |_, player, (): ()| {
                player.set_playing(true);
                Ok(())
            });

            b.method("PlayPause", (), (), move |_, player, (): ()| {
                player.play_pause();
                Ok(())
            });

            b.method("Previous", (), (), move |_, player, (): ()| {
                player.previous();
                Ok(())
            });

            b.method(
                "SetPosition",
                ("TrackId", "Position"),
                (),
                move |_, player: &mut Player, (_track_id, position): (Path, u64)| {
                    // TODO: switch track based on ID
                    // Seeek by offset
                    player.set_position(position);
                    println!("Setting poistion to {}", position);

                    // send signal to let peers know we seeked
                    //let signal_msg = seeked(ctx.path(), &(player.position,));
                    //ctx.push_msg(signal_msg);

                    Ok(())
                },
            );

            b.method("Stop", (), (), move |_, _, (): ()| Ok(()));
        },
    );

    let queue = &[
        Metadata {
            trackid: "spotify:track:dabdabdabdabdabdab".to_string(),
            length: 2000,
            art_url: "https://i.scdn.co/image/ab67616d0000b2739e1cc9cb60157c36a4f1d341".to_string(),
            album: r#"<script>alert("album")</script>"#.to_string(),
            album_artist: vec![r#"<script>alert("album artist")</script>"#.to_string()],
            artist: vec![r#"<script>alert("artist")</script>"#.to_string()],
            auto_rating: 0.40,
            disc_number: 2,
            title: r#"<script>alert("title")</script>"#.to_string(),
            track_number: 3,
            url: "https://open.spotify.com/track/dabdabdabdabdabdab".to_string(),
        },
        Metadata {
            trackid: "spotify:track:0Szxm4RHk2fztgpW4jHh02".to_string(),
            length: 144040000,
            art_url: "https://i.scdn.co/image/ab67616d0000b2739e1cc9cb60157c36a4f1d341".to_string(),
            album: "Fractured Life".to_string(),
            album_artist: vec!["Air Traffic".to_string()],
            artist: vec!["Air Traffic".to_string()],
            auto_rating: 0.36,
            disc_number: 1,
            title: "Charlotte".to_string(),
            track_number: 2,
            url: "https://open.spotify.com/track/0Szxm4RHk2fztgpW4jHh02".to_string(),
        },
    ];

    cr.insert(
        "/org/mpris/MediaPlayer2",
        &[media_player_token, player_token],
        Player::new(queue),
    );

    cr.serve(&c)?;
    unreachable!()
}
