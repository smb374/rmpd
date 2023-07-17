use std::fmt::Display;

use serde::{Deserialize, Serialize};
use specta::Type;

use super::Reflect;

#[derive(Serialize, Deserialize, Type, Hash, PartialEq, Eq, Clone, Debug)]
pub enum Tag {
    Album,
    AlbumArtist,
    AlbumArtistSort,
    AlbumSort,
    Artist,
    ArtistSort,
    Comment,
    Composer,
    ComposerSort,
    Conductor,
    Date,
    Disc,
    Ensemble,
    Genre,
    Grouping,
    Label,
    Location,
    Movement,
    MovementNumber,
    MusicBrainzArtistId,
    MusicBrainzRecordingId,
    MusicBrainzReleaseArtistId,
    MusicBrainzReleaseId,
    MusicBrainzTrackId,
    MusicBrainzWorkId,
    Name,
    OriginalDate,
    Performer,
    Title,
    Track,
    Work,
    Other(String),
}

impl Tag {
    pub fn render(self) -> Result<mpd_client::tag::Tag, mpd_client::tag::TagError> {
        mpd_client::tag::Tag::try_from(self.to_string().as_str())
    }
}

impl Reflect for mpd_client::tag::Tag {
    type Output = Tag;
    fn reflect(self) -> Self::Output {
        match self {
            mpd_client::tag::Tag::Album => Tag::Album,
            mpd_client::tag::Tag::AlbumArtist => Tag::AlbumArtist,
            mpd_client::tag::Tag::AlbumArtistSort => Tag::AlbumArtistSort,
            mpd_client::tag::Tag::AlbumSort => Tag::AlbumSort,
            mpd_client::tag::Tag::Artist => Tag::Artist,
            mpd_client::tag::Tag::ArtistSort => Tag::ArtistSort,
            mpd_client::tag::Tag::Comment => Tag::Comment,
            mpd_client::tag::Tag::Composer => Tag::Composer,
            mpd_client::tag::Tag::ComposerSort => Tag::ComposerSort,
            mpd_client::tag::Tag::Conductor => Tag::Conductor,
            mpd_client::tag::Tag::Date => Tag::Date,
            mpd_client::tag::Tag::Disc => Tag::Disc,
            mpd_client::tag::Tag::Ensemble => Tag::Ensemble,
            mpd_client::tag::Tag::Genre => Tag::Genre,
            mpd_client::tag::Tag::Grouping => Tag::Grouping,
            mpd_client::tag::Tag::Label => Tag::Label,
            mpd_client::tag::Tag::Location => Tag::Location,
            mpd_client::tag::Tag::Movement => Tag::Movement,
            mpd_client::tag::Tag::MovementNumber => Tag::MovementNumber,
            mpd_client::tag::Tag::MusicBrainzArtistId => Tag::MusicBrainzArtistId,
            mpd_client::tag::Tag::MusicBrainzRecordingId => Tag::MusicBrainzRecordingId,
            mpd_client::tag::Tag::MusicBrainzReleaseArtistId => Tag::MusicBrainzReleaseArtistId,
            mpd_client::tag::Tag::MusicBrainzReleaseId => Tag::MusicBrainzReleaseId,
            mpd_client::tag::Tag::MusicBrainzTrackId => Tag::MusicBrainzTrackId,
            mpd_client::tag::Tag::MusicBrainzWorkId => Tag::MusicBrainzWorkId,
            mpd_client::tag::Tag::Name => Tag::Name,
            mpd_client::tag::Tag::OriginalDate => Tag::OriginalDate,
            mpd_client::tag::Tag::Performer => Tag::Performer,
            mpd_client::tag::Tag::Title => Tag::Title,
            mpd_client::tag::Tag::Track => Tag::Track,
            mpd_client::tag::Tag::Work => Tag::Work,
            mpd_client::tag::Tag::Other(x) => Tag::Other(x.to_string()),
            _ => unreachable!(),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Tag::Album => String::from("Album"),
            Tag::AlbumArtist => String::from("AlbumArtist"),
            Tag::AlbumArtistSort => String::from("AlbumArtistSort"),
            Tag::AlbumSort => String::from("AlbumSort"),
            Tag::Artist => String::from("Artist"),
            Tag::ArtistSort => String::from("ArtistSort"),
            Tag::Comment => String::from("Comment"),
            Tag::Composer => String::from("Composer"),
            Tag::ComposerSort => String::from("ComposerSort"),
            Tag::Conductor => String::from("Conductor"),
            Tag::Date => String::from("Date"),
            Tag::Disc => String::from("Disc"),
            Tag::Ensemble => String::from("Ensemble"),
            Tag::Genre => String::from("Genre"),
            Tag::Grouping => String::from("Grouping"),
            Tag::Label => String::from("Label"),
            Tag::Location => String::from("Location"),
            Tag::Movement => String::from("Movement"),
            Tag::MovementNumber => String::from("MovementNumber"),
            Tag::MusicBrainzArtistId => String::from("MusicBrainzArtistId"),
            Tag::MusicBrainzRecordingId => String::from("MusicBrainzRecordingId"),
            Tag::MusicBrainzReleaseArtistId => String::from("MusicBrainzReleaseArtistId"),
            Tag::MusicBrainzReleaseId => String::from("MusicBrainzReleaseId"),
            Tag::MusicBrainzTrackId => String::from("MusicBrainzTrackId"),
            Tag::MusicBrainzWorkId => String::from("MusicBrainzWorkId"),
            Tag::Name => String::from("Name"),
            Tag::OriginalDate => String::from("OriginalDate"),
            Tag::Performer => String::from("Performer"),
            Tag::Title => String::from("Title"),
            Tag::Track => String::from("Track"),
            Tag::Work => String::from("Work"),
            Tag::Other(x) => x.to_string(),
        };
        write!(f, "{}", val)
    }
}
