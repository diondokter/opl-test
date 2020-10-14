use core::fmt::Display;

use alloc::collections::LinkedList;
use opl_driver::{hl::Initialized, hl::Note, hl::Opl2, hl::Opl2Error, ll::HardwareInterface};
use rtt_target::rprintln;

pub struct Sequence<O, E> {
    points: LinkedList<AbsoluteActionPoint<O, E>>,
}

impl<I: HardwareInterface, S: Initialized> Sequence<Opl2<I, S>, Opl2Error> {
    pub fn new(relative_points: &[ActionPoint<Opl2<I, S>, Opl2Error>]) -> Self {
        let mut running_timestamp = 0;

        let mut points = LinkedList::new();

        for point in relative_points {
            running_timestamp += point.delay;
            points.push_back(AbsoluteActionPoint::new(running_timestamp, point.value.clone()));
        }

        Self {
            points,
        }
    }

    pub fn merge(&mut self, other: Self) {
        for p in other.points {
            self.insert(p);
        }
    }

    pub fn run(&mut self, opl: &mut Opl2<I, S>, timestamp: u32) -> Result<bool, Opl2Error> {
        while let Some(point) = self.points.front() {
            if point.timestamp < timestamp {
                panic!("We've got a point from the past?");
            } else if point.timestamp == timestamp {
                // Execute the action
                let point = self.points.pop_front().unwrap();
                self.run_point(opl, point)?;
            } else {
                break;
            }
        }

        Ok(!self.points.is_empty())
    }

    fn run_point(
        &mut self,
        opl: &mut Opl2<I, S>,
        point: AbsoluteActionPoint<Opl2<I, S>, Opl2Error>,
    ) -> Result<(), Opl2Error> {
        rprintln!("Running {} at {}", point.value, point.timestamp);
        match point.value {
            Action::Custom { function } => function(opl)?,
            Action::NoteOn { channel, value } => opl.start_channel(channel, value)?,
            Action::NoteOff { channel } => opl.stop_channel(channel)?,
            Action::PlayNote {
                channel,
                value,
                duration,
            } => {
                self.insert(AbsoluteActionPoint {
                    timestamp: point.timestamp,
                    value: Action::NoteOn { channel, value },
                });
                self.insert(AbsoluteActionPoint {
                    timestamp: point.timestamp + duration,
                    value: Action::NoteOff { channel },
                });
            }
            Action::Repetition {
                sequence,
                repetition_duration,
                repetition_times: repetition_count,
            } => {
                let mut points = sequence.points.clone();

                while let Some(mut repetition_point) = points.pop_front() {
                    repetition_point.timestamp += point.timestamp;
                    self.insert(repetition_point);
                }

                if repetition_count > 1 {
                    self.insert(AbsoluteActionPoint {
                        timestamp: point.timestamp + repetition_duration,
                        value: Action::Repetition {
                            sequence,
                            repetition_duration,
                            repetition_times: repetition_count - 1,
                        },
                    });
                }
            }
            Action::Marker => {}
        }

        Ok(())
    }

    fn insert(&mut self, point: AbsoluteActionPoint<Opl2<I, S>, Opl2Error>) {
        if self.points.is_empty() {
            self.points.push_back(point);
            return;
        }

        let mut index = None;

        for (i, p) in self.points.iter().enumerate() {
            if p.timestamp > point.timestamp {
                index = Some(i);
                break;
            }
        }

        match index {
            Some(index) => {
                let mut late_half = self.points.split_off(index);
                self.points.push_back(point);
                self.points.append(&mut late_half);
            }
            None => {
                self.points.push_back(point);
            }
        }
    }
}

pub struct ActionPoint<O, E> {
    delay: u32,
    value: Action<O, E>,
}

impl<O, E> ActionPoint<O, E> {
    pub fn new(delay: u32, value: Action<O, E>) -> Self {
        Self { delay, value }
    }
}

struct AbsoluteActionPoint<O, E> {
    timestamp: u32,
    value: Action<O, E>,
}

impl<O, E> AbsoluteActionPoint<O, E> {
    fn new(timestamp: u32, value: Action<O, E>) -> Self {
        Self { timestamp, value }
    }
}

pub enum Action<O, E> {
    Custom {
        function: fn(&mut O) -> Result<(), E>,
    },
    NoteOn {
        channel: usize,
        value: Note,
    },
    NoteOff {
        channel: usize,
    },
    PlayNote {
        channel: usize,
        value: Note,
        duration: u32,
    },
    Repetition {
        sequence: Sequence<O, E>,
        repetition_duration: u32,
        repetition_times: u32,
    },
    Marker,
}

impl<O, E> Display for Action<O, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Action::Custom { .. } => write!(f, "Action Custom"),
            Action::NoteOn { .. } => write!(f, "Action NoteOn"),
            Action::NoteOff { .. } => write!(f, "Action NoteOff"),
            Action::PlayNote { .. } => write!(f, "Action PlayNote"),
            Action::Repetition { .. } => write!(f, "Action Repetition"),
            Action::Marker { .. } => write!(f, "Action Marker"),
        }
    }
}

impl<O, E> Clone for Sequence<O, E> {
    fn clone(&self) -> Self {
        Self {
            points: self.points.clone(),
        }
    }
}

impl<O, E> Clone for AbsoluteActionPoint<O, E> {
    fn clone(&self) -> Self {
        Self {
            timestamp: self.timestamp.clone(),
            value: self.value.clone(),
        }
    }
}

impl<O, E> Clone for Action<O, E> {
    fn clone(&self) -> Self {
        match self {
            Action::Custom { function } => Action::Custom {
                function: function.clone(),
            },
            Action::NoteOn { channel, value } => Action::NoteOn {
                channel: channel.clone(),
                value: value.clone(),
            },
            Action::NoteOff { channel } => Action::NoteOff {
                channel: channel.clone(),
            },
            Action::PlayNote {
                channel,
                value,
                duration,
            } => Action::PlayNote {
                channel: channel.clone(),
                value: value.clone(),
                duration: duration.clone(),
            },
            Action::Repetition {
                sequence,
                repetition_duration,
                repetition_times: repetition_count,
            } => Action::Repetition {
                sequence: sequence.clone(),
                repetition_duration: repetition_duration.clone(),
                repetition_times: repetition_count.clone(),
            },
            Action::Marker => Action::Marker,
        }
    }
}
