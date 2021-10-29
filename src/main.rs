use dbus::{
    blocking::Connection,
    Path,
};
use dbus_crossroads::{Context, Crossroads, IfaceBuilder};
use std::{error::Error, vec};

mod metadata;
use crate::metadata::Metadata;

struct Player {
    position: i64,
    shuffle: bool,
    rate: f64,
    volume: f64,
    loop_status: String,
    playing: bool,
    metadata: Metadata,
}

impl Player {
    fn playback_status(&self) -> String {
        if self.playing {
            return "Playing".to_string();
        }
        return "Paused".to_string();
    }
}

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
                .get(|_, player| Ok(player.metadata.to_map()));

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

            b.property("Position").get(|_, player| Ok(player.position));

            b.property("LoopStatus")
                .get(|_, player| Ok(player.loop_status.clone()))
                .set(|_, player, value| {
                    player.loop_status = value.clone();
                    Ok(Some(value.clone()))
                });

            b.property("PlaybackStatus")
                .get(|_, player| Ok(player.playback_status()));

            let seeked = b.signal::<(i64,), _>("Seeked", ("sender",)).msg_fn();

            b.method(
                "Seek",
                ("Offset",),
                (),
                move |ctx: &mut Context, player: &mut Player, (offset,): (i64,)| {
                    // Seeek by offset
                    player.position += offset;
                    println!("Seeking by {} (new position: {})", offset, player.position);

                    // send signal to let peers know we seeked
                    let signal_msg = seeked(ctx.path(), &(player.position,));
                    ctx.push_msg(signal_msg);

                    Ok(())
                },
            );

            b.method("Next", (), (), move |_, _, (): ()| Ok(()));

            b.method("OpenUri", ("Uri",), (), move |_, _, (uri,): (String,)| {
                println!("open_uri: {}", uri);
                Ok(())
            });

            b.method("Pause", (), (), move |_, player, (): ()| {
                player.playing = false;
                Ok(())
            });

            b.method("Play", (), (), move |_, player, (): ()| {
                player.playing = true;
                Ok(())
            });

            b.method("PlayPause", (), (), move |_, player, (): ()| {
                player.playing = !player.playing;
                Ok(())
            });

            b.method("Previous", (), (), move |_, _, (): ()| Ok(()));

            b.method(
                "SetPosition",
                ("TrackId", "Position"),
                (),
                move |_, player: &mut Player, (_track_id, position): (Path, i64)| {
                    // Seeek by offset
                    player.position = position;
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

    let metadata = Metadata {
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
    };

    cr.insert(
        "/org/mpris/MediaPlayer2",
        &[media_player_token, player_token],
        Player {
            position: 0,
            shuffle: false,
            rate: 1.0,
            volume: 1.0,
            playing: true,
            loop_status: "None".to_string(),
            metadata: metadata,
        },
    );

    cr.serve(&c)?;
    unreachable!()
}
