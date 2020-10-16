use chrono::{DateTime, Utc};

/// Parrot Piloting Command
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PCMD {
    /// 1 if the roll and pitch values should be taken in consideration. 0 otherwise
    pub flag: bool,
    pub roll: i8,
    pub pitch: i8,
    pub yaw: i8,
    pub gaz: i8,
    pub timestamp: DateTime<Utc>,
    // TODO: How should we handle the `sequence_id` in order not to show it to the user?
    pub sequence_id: u8,
}

mod scroll_impl {
    use super::*;
    use crate::frame::Error;
    use chrono::TimeZone;
    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for PCMD {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;
            let flag = match src.gread_with::<u8>(&mut offset, ctx)? {
                0 => false,
                1 => true,
                value => {
                    return Err(Self::Error::OutOfBound {
                        value: value.into(),
                        param: "flag".to_string(),
                    })
                }
            };

            let roll = src.gread_with(&mut offset, ctx)?;
            let pitch = src.gread_with(&mut offset, ctx)?;
            let yaw = src.gread_with(&mut offset, ctx)?;
            let gaz = src.gread_with(&mut offset, ctx)?;

            let timestamp_and_seq = src.gread_with::<TimestampAndSeq>(&mut offset, ctx)?;

            Ok((
                PCMD {
                    flag,
                    roll,
                    pitch,
                    yaw,
                    gaz,
                    timestamp: timestamp_and_seq.timestamp,
                    sequence_id: timestamp_and_seq.sequence_id,
                },
                offset,
            ))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for PCMD {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;
            this.gwrite_with::<u8>(self.flag.into(), &mut offset, ctx)?;
            this.gwrite_with(self.roll, &mut offset, ctx)?;
            this.gwrite_with(self.pitch, &mut offset, ctx)?;
            this.gwrite_with(self.yaw, &mut offset, ctx)?;
            this.gwrite_with(self.gaz, &mut offset, ctx)?;
            let timestamp_and_seq = TimestampAndSeq {
                timestamp: self.timestamp,
                sequence_id: self.sequence_id,
            };

            this.gwrite_with(timestamp_and_seq, &mut offset, ctx)?;

            Ok(offset)
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for TimestampAndSeq {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;

            // we always use Little-endian
            let timestamp_and_seq = src.gread_with::<u32>(&mut offset, ctx)?.to_le_bytes();
            // 24 bits
            let timestamp_i64 = i64::from_le_bytes([
                timestamp_and_seq[0],
                timestamp_and_seq[1],
                timestamp_and_seq[2],
                0,
                0,
                0,
                0,
                0,
            ]);
            let timestamp = Utc.timestamp_millis(timestamp_i64);
            // 8 bits
            let sequence_id = timestamp_and_seq[3];

            Ok((
                Self {
                    timestamp,
                    sequence_id,
                },
                offset,
            ))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for TimestampAndSeq {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;

            let milliseconds = self.timestamp.timestamp_millis();
            // from byte 5 to 8 = 3 bytes
            // always use Little-endian!
            let bytes = &milliseconds.to_le_bytes()[5..];

            this.gwrite_with(bytes, &mut offset, ())?;
            this.gwrite_with(self.sequence_id, &mut offset, ctx)?;

            Ok(offset)
        }
    }

    struct TimestampAndSeq {
        timestamp: DateTime<Utc>,
        sequence_id: u8,
    }
}
