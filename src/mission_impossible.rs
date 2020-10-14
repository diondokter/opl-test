use crate::sequencer::{ActionPoint, Action, Sequence};
use crate::{QUARTER, HALF, EIGHTH, Opl, SIXTEENTH, FULL};
use opl_driver::hl::{Note, Opl2Error, Melody};
use opl_driver::instrument::{MelodyInstrument, OperatorSettings};
use opl_driver::ll::registers::{operator_settings0, operator_settings1, operator_settings2, operator_settings3, operator_settings4, channel_settings2};
use opl_driver::ll::{ModulatorFrequencyMultiple, Bit, WaveformType, SynthesisType, ScalingLevel};

pub fn bass_instrument() -> MelodyInstrument {
    MelodyInstrument::new(
        OperatorSettings::new(
            operator_settings0::W::zero()
                .modulator_frequency_multiple(ModulatorFrequencyMultiple::AtSpecified)
                .sustain(Bit::Cleared)
                .tremolo(Bit::Cleared)
                .keyboard_scaling_rate(Bit::Cleared)
                .vibrato(Bit::Cleared),
            operator_settings1::W::zero()
                .output_level(16)
                .level_key_scaling(ScalingLevel::NoChange),
            operator_settings2::W::zero()
                .attack_rate(15)
                .decay_rate(3),
            operator_settings3::W::zero()
                .sustain_level(4)
                .release_rate(6),
            operator_settings4::W::zero()
                .waveform(WaveformType::Sine),
        ),
        channel_settings2::W::zero()
            .feedback(5)
            .synthesis_type(SynthesisType::FrequencyModulation),
        OperatorSettings::new(
            operator_settings0::W::zero()
                .modulator_frequency_multiple(ModulatorFrequencyMultiple::AtSpecified)
                .sustain(Bit::Cleared)
                .tremolo(Bit::Cleared)
                .keyboard_scaling_rate(Bit::Cleared)
                .vibrato(Bit::Cleared),
            operator_settings1::W::zero()
                .output_level(0)
                .level_key_scaling(ScalingLevel::NoChange),
            operator_settings2::W::zero()
                .attack_rate(9)
                .decay_rate(4),
            operator_settings3::W::zero()
                .sustain_level(3)
                .release_rate(13),
            operator_settings4::W::zero()
                .waveform(WaveformType::PulseSine),
        ),
    )
}

pub fn motiv_instrument() -> MelodyInstrument {
    MelodyInstrument::new(
        OperatorSettings::new(
            operator_settings0::W::zero()
                .modulator_frequency_multiple(ModulatorFrequencyMultiple::OneOctaveBelow)
                .sustain(Bit::Set)
                .tremolo(Bit::Set)
                .keyboard_scaling_rate(Bit::Cleared)
                .vibrato(Bit::Set),
            operator_settings1::W::zero()
                .output_level(21)
                .level_key_scaling(ScalingLevel::NoChange),
            operator_settings2::W::zero()
                .attack_rate(6)
                .decay_rate(1),
            operator_settings3::W::zero()
                .sustain_level(6)
                .release_rate(6),
            operator_settings4::W::zero()
                .waveform(WaveformType::Sine),
        ),
        channel_settings2::W::zero()
            .feedback(0)
            .synthesis_type(SynthesisType::FrequencyModulation),
        OperatorSettings::new(
            operator_settings0::W::zero()
                .modulator_frequency_multiple(ModulatorFrequencyMultiple::AtSpecified)
                .sustain(Bit::Set)
                .tremolo(Bit::Set)
                .keyboard_scaling_rate(Bit::Cleared)
                .vibrato(Bit::Set),
            operator_settings1::W::zero()
                .output_level(0)
                .level_key_scaling(ScalingLevel::NoChange),
            operator_settings2::W::zero()
                .attack_rate(15)
                .decay_rate(5),
            operator_settings3::W::zero()
                .sustain_level(2)
                .release_rate(5),
            operator_settings4::W::zero()
                .waveform(WaveformType::Sine),
        ),
    )
}

pub fn chord_fill_instrument() -> MelodyInstrument {
    MelodyInstrument::new(
        OperatorSettings::new(
            operator_settings0::W::zero()
                .modulator_frequency_multiple(ModulatorFrequencyMultiple::AtSpecified)
                .sustain(Bit::Set)
                .tremolo(Bit::Cleared)
                .keyboard_scaling_rate(Bit::Cleared)
                .vibrato(Bit::Cleared),
            operator_settings1::W::zero()
                .output_level(8)
                .level_key_scaling(ScalingLevel::NoChange),
            operator_settings2::W::zero()
                .attack_rate(6)
                .decay_rate(2),
            operator_settings3::W::zero()
                .sustain_level(3)
                .release_rate(4),
            operator_settings4::W::zero()
                .waveform(WaveformType::HalfSine),
        ),
        channel_settings2::W::zero()
            .feedback(1)
            .synthesis_type(SynthesisType::FrequencyModulation),
        OperatorSettings::new(
            operator_settings0::W::zero()
                .modulator_frequency_multiple(ModulatorFrequencyMultiple::AtSpecified)
                .sustain(Bit::Set)
                .tremolo(Bit::Cleared)
                .keyboard_scaling_rate(Bit::Cleared)
                .vibrato(Bit::Set),
            operator_settings1::W::zero()
                .output_level(0)
                .level_key_scaling(ScalingLevel::NoChange),
            operator_settings2::W::zero()
                .attack_rate(15)
                .decay_rate(4),
            operator_settings3::W::zero()
                .sustain_level(1)
                .release_rate(6),
            operator_settings4::W::zero()
                .waveform(WaveformType::HalfSine),
        ),
    )
}


pub fn bass_loop(times: u32, channel: usize, octave: u8) -> Action<Opl<Melody>, Opl2Error> {
    #[rustfmt::skip]
        let bass_sequence = Sequence::new(&[
        ActionPoint::new(0, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::Bb(octave), duration: QUARTER - 1 }),
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::C(octave+1), duration: QUARTER - 1 }),

        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::F(octave), duration: QUARTER - 1 }),
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::Fs(octave), duration: QUARTER - 1 }),
    ]);

    Action::Repetition {
        sequence: bass_sequence,
        repetition_duration: QUARTER * 10,
        repetition_times: times,
    }
}

pub fn bass_loop_to_alt_transition(channel: usize, octave: u8) -> Action<Opl<Melody>, Opl2Error> {
    #[rustfmt::skip]
        let bass_sequence = Sequence::new(&[
        ActionPoint::new(0, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::Bb(octave), duration: QUARTER - 1 }),
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::C(octave+1), duration: QUARTER - 1 }),

        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::Bb(octave), duration: QUARTER - 1 }),
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::B(octave), duration: QUARTER - 1 }),
    ]);

    Action::Repetition {
        sequence: bass_sequence,
        repetition_duration: QUARTER * 10,
        repetition_times: 1,
    }
}

pub fn bass_loop_alt(channel: usize, octave: u8) -> Action<Opl<Melody>, Opl2Error> {
    #[rustfmt::skip]
        let bass_sequence = Sequence::new(&[
        ActionPoint::new(0, Action::PlayNote { channel, value: Note::C(octave), duration: QUARTER }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::Fs(octave), duration: QUARTER - 1 }),
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER - 1 }),

        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::C(octave), duration: QUARTER }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::F(octave), duration: QUARTER - 1 }),
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER - 1 }),
    
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::C(octave), duration: QUARTER }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::Eb(octave), duration: QUARTER - 1 }),
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::G(octave), duration: QUARTER - 1 }),
    
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::C(octave), duration: QUARTER }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::C(octave), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel, value: Note::Eb(octave), duration: QUARTER - 1 }),
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::F(octave), duration: QUARTER - 1 }),
    ]);

    Action::Repetition {
        sequence: bass_sequence,
        repetition_duration: QUARTER * 20,
        repetition_times: 1,
    }
}

pub fn bass_finisher(channel_low: usize, channel_high: usize, octave_low: u8, octave_high: u8) -> Action<Opl<Melody>, Opl2Error> {
    #[rustfmt::skip]
        let bass_sequence = Sequence::new(&[
        ActionPoint::new(0, Action::PlayNote { channel: channel_low, value: Note::G(octave_low), duration: QUARTER }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel: channel_low, value: Note::G(octave_low), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel: channel_low, value: Note::Bb(octave_low), duration: QUARTER - 1 }),
        ActionPoint::new(QUARTER, Action::PlayNote { channel: channel_low, value: Note::C(octave_low+1), duration: QUARTER - 1 }),

        ActionPoint::new(QUARTER, Action::PlayNote { channel: channel_low, value: Note::D(octave_low), duration: QUARTER }),
        ActionPoint::new(0      , Action::PlayNote { channel: channel_high, value: Note::A(octave_high), duration: QUARTER }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel: channel_low, value: Note::D(octave_low), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(0               , Action::PlayNote { channel: channel_high, value: Note::A(octave_high), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel: channel_low, value: Note::Eb(octave_low), duration: QUARTER - 1 }),
        ActionPoint::new(0               , Action::PlayNote { channel: channel_high, value: Note::Bb(octave_high), duration: QUARTER - 1 }),
        ActionPoint::new(QUARTER, Action::PlayNote { channel: channel_low, value: Note::F(octave_low), duration: QUARTER - 1 }),
        ActionPoint::new(0      , Action::PlayNote { channel: channel_high, value: Note::C(octave_high+1), duration: QUARTER - 1 }),
    
        ActionPoint::new(QUARTER, Action::PlayNote { channel: channel_low, value: Note::D(octave_low), duration: QUARTER }),
        ActionPoint::new(0      , Action::PlayNote { channel: channel_high, value: Note::A(octave_high), duration: QUARTER }),

        ActionPoint::new(QUARTER * 3, Action::PlayNote { channel: channel_high, value: Note::Ab(octave_high), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel: channel_high, value: Note::Cs(octave_high), duration: QUARTER * 5 + EIGHTH }),
    ]);

    Action::Repetition {
        sequence: bass_sequence,
        repetition_duration: QUARTER * 20,
        repetition_times: 1,
    }
}

pub fn main_motiv(channel: usize) -> Action<Opl<Melody>, Opl2Error> {
    const OCTAVE: u8 = 5;

    #[rustfmt::skip]
        let bass_sequence = Sequence::new(&[
        ActionPoint::new(0, Action::PlayNote { channel, value: Note::Bb(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::G(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::D(OCTAVE), duration: FULL }),

        ActionPoint::new(FULL, Action::PlayNote { channel, value: Note::Bb(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::G(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::Cs(OCTAVE), duration: FULL }),

        ActionPoint::new(FULL, Action::PlayNote { channel, value: Note::Bb(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::G(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::C(OCTAVE), duration: FULL }),

        ActionPoint::new(FULL, Action::PlayNote { channel, value: Note::Bb(OCTAVE - 1), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::C(OCTAVE), duration: QUARTER }),
    ]);

    Action::Repetition {
        sequence: bass_sequence,
        repetition_duration: QUARTER * 20,
        repetition_times: 1,
    }
}

pub fn alt_motiv(channel: usize) -> Action<Opl<Melody>, Opl2Error> {
    const OCTAVE: u8 = 4;

    #[rustfmt::skip]
        let bass_sequence = Sequence::new(&[
        ActionPoint::new(QUARTER, Action::PlayNote { channel, value: Note::Bb(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::G(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::Fs(OCTAVE+1), duration: QUARTER*3 }),

        ActionPoint::new(FULL, Action::PlayNote { channel, value: Note::Bb(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::G(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::F(OCTAVE+1), duration: QUARTER*3 }),

        ActionPoint::new(FULL, Action::PlayNote { channel, value: Note::Bb(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::G(OCTAVE), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::E(OCTAVE+1), duration: QUARTER*3 }),

        ActionPoint::new(QUARTER*3, Action::PlayNote { channel, value: Note::Eb(OCTAVE+1), duration: EIGHTH }),
        ActionPoint::new(EIGHTH, Action::PlayNote { channel, value: Note::D(OCTAVE+1), duration: QUARTER }),
    ]);

    Action::Repetition {
        sequence: bass_sequence,
        repetition_duration: QUARTER * 20,
        repetition_times: 1,
    }
}

pub fn chord_fill(channels: [usize; 3]) -> Action<Opl<Melody>, Opl2Error> {
    const OCTAVE: u8 = 4;

    #[rustfmt::skip]
    let fill = Sequence::new(&[
        ActionPoint::new(0, Action::PlayNote { channel: channels[0], value: Note::G(OCTAVE), duration: QUARTER }),
        ActionPoint::new(0, Action::PlayNote { channel: channels[1], value: Note::D(OCTAVE), duration: QUARTER }),
        ActionPoint::new(0, Action::PlayNote { channel: channels[2], value: Note::Bb(OCTAVE-1), duration: QUARTER }),
        // ---
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel: channels[0], value: Note::G(OCTAVE), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(0               , Action::PlayNote { channel: channels[1], value: Note::D(OCTAVE), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(0               , Action::PlayNote { channel: channels[2], value: Note::Bb(OCTAVE-1), duration: QUARTER + SIXTEENTH }),
        // ---
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel: channels[0], value: Note::Bb(OCTAVE), duration: QUARTER - 1 }),
        ActionPoint::new(0               , Action::PlayNote { channel: channels[1], value: Note::F(OCTAVE), duration: QUARTER - 1 }),
        ActionPoint::new(0               , Action::PlayNote { channel: channels[2], value: Note::D(OCTAVE), duration: QUARTER - 1 }),
        // ---
        ActionPoint::new(QUARTER, Action::PlayNote { channel: channels[0], value: Note::Eb(OCTAVE), duration: QUARTER - 1 }),
        ActionPoint::new(0      , Action::PlayNote { channel: channels[1], value: Note::G(OCTAVE), duration: QUARTER - 1 }),
        ActionPoint::new(0      , Action::PlayNote { channel: channels[2], value: Note::C(OCTAVE+1), duration: QUARTER - 1 }),

        ActionPoint::new(QUARTER, Action::PlayNote { channel: channels[0], value: Note::G(OCTAVE), duration: QUARTER }),
        ActionPoint::new(0      , Action::PlayNote { channel: channels[1], value: Note::D(OCTAVE), duration: QUARTER }),
        ActionPoint::new(0      , Action::PlayNote { channel: channels[2], value: Note::Bb(OCTAVE-1), duration: QUARTER }),
        // ---
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel: channels[0], value: Note::G(OCTAVE), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(0               , Action::PlayNote { channel: channels[1], value: Note::D(OCTAVE), duration: QUARTER + SIXTEENTH }),
        ActionPoint::new(0               , Action::PlayNote { channel: channels[2], value: Note::Bb(OCTAVE-1), duration: QUARTER + SIXTEENTH }),
        // ---
        ActionPoint::new(QUARTER + EIGHTH, Action::PlayNote { channel: channels[0], value: Note::Ab(OCTAVE-1), duration: QUARTER - 1 }),
        ActionPoint::new(0               , Action::PlayNote { channel: channels[1], value: Note::C(OCTAVE), duration: QUARTER - 1 }),
        ActionPoint::new(0               , Action::PlayNote { channel: channels[2], value: Note::F(OCTAVE), duration: QUARTER - 1 }),
        // ---
        ActionPoint::new(QUARTER, Action::PlayNote { channel: channels[0], value: Note::A(OCTAVE-1), duration: QUARTER - 1 }),
        ActionPoint::new(0      , Action::PlayNote { channel: channels[1], value: Note::C(OCTAVE), duration: QUARTER - 1 }),
        ActionPoint::new(0      , Action::PlayNote { channel: channels[2], value: Note::Fs(OCTAVE), duration: QUARTER - 1 }),
        // ---
        ActionPoint::new(QUARTER, Action::PlayNote { channel: channels[0], value: Note::Bb(OCTAVE-1), duration: QUARTER - 1 }),
        ActionPoint::new(0      , Action::PlayNote { channel: channels[1], value: Note::D(OCTAVE), duration: QUARTER - 1 }),
        ActionPoint::new(0      , Action::PlayNote { channel: channels[2], value: Note::G(OCTAVE), duration: QUARTER - 1 }),
    ]);

    Action::Repetition {
        sequence: fill,
        repetition_duration: QUARTER * 20,
        repetition_times: 1,
    }
}
