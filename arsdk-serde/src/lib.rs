mod de;
mod error;
mod ser;

pub use de::{from_bytes, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_bytes, Serializer};

#[cfg(test)]
mod test {
    use super::{from_bytes, to_bytes};

    #[test]
    fn test_values() {
        let bytes = vec![5_u8];
        assert_eq!(to_bytes(&5_u8).unwrap(), bytes);
        assert_eq!(from_bytes::<u8>(&bytes).unwrap(), 5_u8);
    }

    mod tuple {
        use crate::{from_bytes, to_bytes, Error};
        use scroll::{Pread, Pwrite};
        use serde::{Deserialize, Serialize, Serializer};
        use std::convert::TryFrom;

        pub type Common = u8;

        pub type Tuple<V> = (V, Vec<u8>);
        pub type TupleResult<V> = Result<Tuple<V>, Error>;

        // #[derive(Debug, Serialize)]
        // #[serde(remote = "std::result::Result")]
        // struct ResultDef<V, E>(std::result::Result<V, E>);

        // impl<V, E> Serialize for ResultDef<V, E>
        // where
        //     V: Serialize,
        // {
        //     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        //     where
        //         S: Serializer,
        //     {
        //         self.0?
        //     }
        // }

        #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
        // #[serde(try_from = "Tuple<u8>", into = "Tuple<u8>")]
        #[serde(into = "Tuple<u8>")]
        pub enum Feature {
            Common(Common),
            ArDrone3(ArDrone3),
        }

        // TODO: Serde doesn't allow for `TryInto`, can we use a Result instead?
        impl Into<Tuple<u8>> for Feature {
            fn into(self) -> Tuple<u8> {
                match self {
                    Feature::Common(common) => {
                        let common = to_bytes(&common).expect("Common");

                        (0, common)
                    }
                    Feature::ArDrone3(ardrone3) => {
                        let ardrone3 = to_bytes(&ardrone3).expect("ArDrone3");

                        (1, ardrone3)
                    }
                }
            }
        }

        impl TryFrom<Tuple<u8>> for Feature {
            type Error = Error;

            fn try_from(tuple: Tuple<u8>) -> Result<Self, Error> {
                let mut offset = 0;

                match tuple.0 {
                    0 => from_bytes::<Common>(&tuple.1).map(Self::Common),
                    1 => from_bytes::<ArDrone3>(&tuple.1).map(Self::ArDrone3),
                    _ => Err(Error::Message("Out of bound for Feature".to_string())),
                }
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
        #[serde(try_from = "Tuple<u8>", into = "Tuple<u8>")]
        pub enum ArDrone3 {
            /// ARCOMMANDS_ID_ARDRONE3_CLASS_PILOTING = 0
            Piloting(Piloting),
            /// ARCOMMANDS_ID_ARDRONE3_CLASS_CAMERA = 1
            Camera,
        }

        // TODO: Serde doesn't allow for `TryInto`, can we use a Result instead?
        impl Into<Tuple<u8>> for ArDrone3 {
            fn into(self) -> Tuple<u8> {
                match self {
                    ArDrone3::Piloting(piloting) => (0, to_bytes(&piloting).expect("Piloting")),
                    ArDrone3::Camera => (1, vec![]),
                }
            }
        }

        impl TryFrom<Tuple<u8>> for ArDrone3 {
            type Error = Error;

            fn try_from(tuple: Tuple<u8>) -> Result<Self, Error> {
                match tuple.0 {
                    0 => Ok(Self::Piloting(from_bytes(&tuple.1)?)),
                    1 => Ok(Self::Camera),
                    _ => Err(Error::Message("Out of bound for ArDrone3".to_string())),
                }
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(try_from = "Tuple<u16>", into = "Tuple<u16>")]
        /// u16
        pub enum Piloting {
            /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_FLATTRIM = 0
            FlatTrim,
            /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_TAKEOFF = 1
            TakeOff,
        }

        // TODO: Serde doesn't allow for `TryInto`, can we use a Result instead?
        impl Into<Tuple<u16>> for Piloting {
            fn into(self) -> Tuple<u16> {
                match self {
                    Piloting::FlatTrim => (0, vec![]),
                    Piloting::TakeOff => (1, vec![]),
                }
            }
        }

        impl TryFrom<Tuple<u16>> for Piloting {
            type Error = Error;

            fn try_from(tuple: Tuple<u16>) -> Result<Self, Error> {
                match tuple.0 {
                    0 => Ok(Self::FlatTrim),
                    1 => Ok(Self::TakeOff),
                    _ => Err(Error::Message("Out of bound for Piloting".to_string())),
                }
            }
        }

        #[test]
        /// Feature - ArDrone3 = 1_u8
        /// ArDrone3 - Piloting = 0_u8
        /// Piloting - TakeOff = 1_u16; bytes: [1_u8, 0]
        fn test_enum_feature_de_serialization() {
            let feature = Feature::ArDrone3(ArDrone3::Piloting(Piloting::TakeOff));

            let actual_serialized = to_bytes(&feature).expect("Should serialize");

            let expected_serialized = [1_u8, 0, 1, 0];

            assert_eq!(actual_serialized, expected_serialized);

            // let actual_deserialized =
            //     from_bytes::<Feature>(&expected_serialized).expect("Should deserialize");
            // assert_eq!(feature, actual_deserialized);

            let actual_deserialized =
                from_bytes::<Tuple<u8>>(&expected_serialized).expect("Should deserialize");
            assert_eq!(
                actual_deserialized,
                (expected_serialized[0], expected_serialized[1..].to_vec())
            );
        }
    }
}
