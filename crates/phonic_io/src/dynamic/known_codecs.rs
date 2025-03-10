use crate::{
    dynamic::{DynCodecConstructor, DynStream, TaggedSignal},
    CodecTag, StreamSpec, StreamSpecBuilder,
};
use phonic_signal::{PhonicError, PhonicResult};
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum KnownCodec {
    #[cfg(feature = "pcm")]
    PcmLE,

    #[cfg(feature = "pcm")]
    PcmBE,
}

impl CodecTag for KnownCodec {
    fn infer_spec(spec: StreamSpecBuilder<Self>) -> PhonicResult<StreamSpec<Self>> {
        use crate::codecs::*;

        match spec.codec {
            #[cfg(feature = "pcm")]
            Some(Self::PcmLE | Self::PcmBE) => pcm::PcmCodecTag::infer_tagged_spec(spec),

            None => Err(PhonicError::missing_data()),
        }
    }
}

impl DynCodecConstructor for KnownCodec {
    fn encoder(&self, signal: TaggedSignal) -> PhonicResult<Box<dyn DynStream<Tag = Self>>> {
        use crate::codecs::*;

        match self {
            #[cfg(feature = "pcm")]
            Self::PcmLE | Self::PcmBE => pcm::PcmCodecTag::from_dyn_signal(*self, signal),

            #[allow(unreachable_patterns)]
            _ => Err(PhonicError::unsupported()),
        }
    }

    fn decoder(stream: Box<dyn DynStream<Tag = Self>>) -> PhonicResult<TaggedSignal> {
        use crate::codecs::*;

        match stream.stream_spec().codec {
            #[cfg(feature = "pcm")]
            Self::PcmLE | Self::PcmBE => pcm::PcmCodecTag::from_dyn_stream(stream),

            #[allow(unreachable_patterns)]
            _ => Err(PhonicError::unsupported()),
        }
    }
}
