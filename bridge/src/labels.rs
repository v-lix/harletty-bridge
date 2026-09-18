use bridge_api::RChannelLabel;
use truehd::structs::channel::ChannelLabel;

/// Convert a TrueHD `ChannelLabel` to its ABI-stable counterpart.
pub(crate) fn channel_label_to_r(label: &ChannelLabel) -> RChannelLabel {
    match label {
        ChannelLabel::L => RChannelLabel::L,
        ChannelLabel::R => RChannelLabel::R,
        ChannelLabel::C => RChannelLabel::C,
        ChannelLabel::LFE => RChannelLabel::LFE,
        ChannelLabel::Ls => RChannelLabel::Ls,
        ChannelLabel::Rs => RChannelLabel::Rs,
        ChannelLabel::Tfl => RChannelLabel::Tfl,
        ChannelLabel::Tfr => RChannelLabel::Tfr,
        ChannelLabel::Tsl => RChannelLabel::Tsl,
        ChannelLabel::Tsr => RChannelLabel::Tsr,
        ChannelLabel::Tbl => RChannelLabel::Tbl,
        ChannelLabel::Tbr => RChannelLabel::Tbr,
        ChannelLabel::Lsc => RChannelLabel::Lsc,
        ChannelLabel::Rsc => RChannelLabel::Rsc,
        ChannelLabel::Lb => RChannelLabel::Lb,
        ChannelLabel::Rb => RChannelLabel::Rb,
        ChannelLabel::Cb => RChannelLabel::Cb,
        ChannelLabel::Tc => RChannelLabel::Tc,
        ChannelLabel::Lsd => RChannelLabel::Lsd,
        ChannelLabel::Rsd => RChannelLabel::Rsd,
        ChannelLabel::Lw => RChannelLabel::Lw,
        ChannelLabel::Rw => RChannelLabel::Rw,
        ChannelLabel::Tfc => RChannelLabel::Tfc,
        ChannelLabel::LFE2 => RChannelLabel::LFE2,
    }
}

/// Convert an E-AC3 `BedChannel` to its ABI-stable counterpart.
/// Map a DCA (DTS) bed channel to the renderer's channel label. DTS core beds
/// cover the 5.1/7.1 layout; the renderer places each at its canonical speaker.
pub(crate) fn dca_bed_channel_to_r(ch: dca::BedChannel) -> RChannelLabel {
    use dca::BedChannel;
    match ch {
        BedChannel::FrontLeft => RChannelLabel::L,
        BedChannel::FrontRight => RChannelLabel::R,
        BedChannel::Center => RChannelLabel::C,
        BedChannel::LowFrequencyEffects => RChannelLabel::LFE,
        BedChannel::SurroundLeft => RChannelLabel::Ls,
        BedChannel::SurroundRight => RChannelLabel::Rs,
        BedChannel::RearCenter => RChannelLabel::Cb,
        BedChannel::RearLeft => RChannelLabel::Lb,
        BedChannel::RearRight => RChannelLabel::Rb,
        BedChannel::WideLeft => RChannelLabel::Lw,
        BedChannel::WideRight => RChannelLabel::Rw,
    }
}

/// Map a DTS:X spatial-extension channel to the renderer's channel label.
///
/// Which extension waveform sits at which position is codec knowledge and lives
/// in `dca::spatial`; this is only the ABI projection of it, so the realtime
/// path and the offline ADM exporter cannot drift apart on the mapping.
pub(crate) fn dca_spatial_channel_to_r(ch: dca::SpatialChannel) -> RChannelLabel {
    use dca::SpatialChannel;
    match ch {
        SpatialChannel::TopFrontLeft => RChannelLabel::Tfl,
        SpatialChannel::TopFrontRight => RChannelLabel::Tfr,
        SpatialChannel::TopFrontCenter => RChannelLabel::Tfc,
        SpatialChannel::TopSideLeft => RChannelLabel::Tsl,
        SpatialChannel::TopSideRight => RChannelLabel::Tsr,
        SpatialChannel::TopBackLeft => RChannelLabel::Tbl,
        SpatialChannel::TopBackRight => RChannelLabel::Tbr,
        SpatialChannel::WideLeft => RChannelLabel::Lw,
        SpatialChannel::WideRight => RChannelLabel::Rw,
    }
}

pub(crate) fn bed_channel_to_r(ch: eac3::BedChannel) -> RChannelLabel {
    use eac3::BedChannel;
    match ch {
        BedChannel::FrontLeft => RChannelLabel::L,
        BedChannel::FrontRight => RChannelLabel::R,
        BedChannel::Center => RChannelLabel::C,
        BedChannel::LowFrequencyEffects => RChannelLabel::LFE,
        BedChannel::SurroundLeft => RChannelLabel::Ls,
        BedChannel::SurroundRight => RChannelLabel::Rs,
        BedChannel::RearCenter => RChannelLabel::Cb,
        BedChannel::RearLeft => RChannelLabel::Lb,
        BedChannel::RearRight => RChannelLabel::Rb,
        BedChannel::TopFrontLeft => RChannelLabel::Tfl,
        BedChannel::TopFrontRight => RChannelLabel::Tfr,
        BedChannel::TopSurroundLeft => RChannelLabel::Tsl,
        BedChannel::TopSurroundRight => RChannelLabel::Tsr,
        BedChannel::TopRearLeft => RChannelLabel::Tbl,
        BedChannel::TopRearRight => RChannelLabel::Tbr,
        BedChannel::WideLeft => RChannelLabel::Lw,
        BedChannel::WideRight => RChannelLabel::Rw,
        BedChannel::LowFrequencyEffects2 => RChannelLabel::LFE2,
    }
}

/// Channel label for an OAMD bed-assignment speaker index
/// (`truehd::structs::oamd::SpeakerLabels` order).
pub(crate) fn oamd_speaker_to_label(speaker_index: usize) -> RChannelLabel {
    match speaker_index {
        0 => RChannelLabel::L,
        1 => RChannelLabel::R,
        2 => RChannelLabel::C,
        3 => RChannelLabel::LFE,
        4 => RChannelLabel::Ls,   // Lss
        5 => RChannelLabel::Rs,   // Rss
        6 => RChannelLabel::Lb,   // Lrs
        7 => RChannelLabel::Rb,   // Rrs
        8 => RChannelLabel::Tfl,  // Lfh (front height)
        9 => RChannelLabel::Tfr,  // Rfh
        10 => RChannelLabel::Tsl, // Lts (top side)
        11 => RChannelLabel::Tsr, // Rts
        12 => RChannelLabel::Tbl, // Lrh (rear height)
        13 => RChannelLabel::Tbr, // Rrh
        14 => RChannelLabel::Lw,
        15 => RChannelLabel::Rw,
        16 => RChannelLabel::LFE2,
        _ => RChannelLabel::Unknown,
    }
}

/// Map an Auro stream to the renderer's channel label.
///
/// Auro's own labels throughout, because Auro is a layout before it is a codec
/// and it places every one of these itself: the floor at ±30 and ±110, the
/// height layer over each of them at 30 degrees, the top overhead, all
/// equidistant from the listener. The shared labels mean a room's corners, so
/// they were wrong here by 20 degrees at the surrounds before they were wrong
/// overhead - and no top label means "over the surrounds at 30º" at all. The
/// renderer knows where Auro's are.
///
/// Two keep the shared labels, both because no Auro document gives them an
/// angle: the LFE, which the guidelines send wherever it measures best rather
/// than to a position, and the centre surround of the old 12.1 layouts. The
/// 2011 white paper does define that layout - a 6.1 lower layer under a 6.0
/// height layer - but never says where the channel it adds goes, and no layout
/// id here pairs six floor channels with a height layer anyway, so it cannot
/// reach an Auro presentation.
///
/// Stream 15, the second top of the `_2T` layouts, is left unlabelled: nothing
/// public says which of the pair it is, and a guess would be placed as
/// confidently as a fact.
pub(crate) fn auro_stream_to_r(stream: auro::StreamId) -> RChannelLabel {
    match stream.0 {
        0 => RChannelLabel::AuroL,
        1 => RChannelLabel::AuroR,
        2 => RChannelLabel::AuroC,
        3 => RChannelLabel::LFE,
        4 => RChannelLabel::AuroLs,
        5 => RChannelLabel::AuroRs,
        6 => RChannelLabel::Cb,
        7 => RChannelLabel::AuroLb,
        8 => RChannelLabel::AuroRb,
        9 => RChannelLabel::AuroHl,
        10 => RChannelLabel::AuroHr,
        11 => RChannelLabel::AuroHc,
        12 => RChannelLabel::AuroT,
        13 => RChannelLabel::AuroHls,
        14 => RChannelLabel::AuroHrs,
        _ => RChannelLabel::Unknown,
    }
}
