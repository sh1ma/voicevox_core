use self::status::*;
use super::*;
use onnxruntime::{ndarray, session::NdArray};

const PHONEME_LENGTH_MINIMAL: f32 = 0.01;

pub struct InferenceCore {
    status: Status,
}

impl InferenceCore {
    pub(crate) async fn new_with_initialize(use_gpu: bool, cpu_num_threads: u16) -> Result<Self> {
        if !use_gpu || Self::can_support_gpu_feature()? {
            let status = Status::new(use_gpu, cpu_num_threads);
            Ok(Self { status })
        } else {
            Err(ErrorRepr::GpuSupport.into())
        }
    }

    fn can_support_gpu_feature() -> Result<bool> {
        let supported_devices = SupportedDevices::create()?;

        cfg_if! {
            if #[cfg(feature = "directml")]{
                Ok(*supported_devices.dml())
            } else{
                Ok(*supported_devices.cuda())
            }
        }
    }

    pub async fn load_model(&self, model: &VoiceModel) -> Result<()> {
        self.status.load_model(model).await
    }

    pub fn unload_model(&self, voice_model_id: &VoiceModelId) -> Result<()> {
        self.status.unload_model(voice_model_id)
    }
    pub fn metas(&self) -> VoiceModelMeta {
        self.status.metas()
    }

    pub fn is_loaded_model(&self, model_id: &VoiceModelId) -> bool {
        self.status.is_loaded_model(model_id)
    }

    pub fn is_model_loaded_by_style_id(&self, style_id: StyleId) -> bool {
        self.status.is_loaded_model_by_style_id(style_id)
    }

    pub async fn predict_duration(
        &self,
        phoneme_vector: &[i64],
        style_id: StyleId,
    ) -> Result<Vec<f32>> {
        if !self.status.validate_speaker_id(style_id) {
            return Err(ErrorRepr::StyleNotFound { style_id }.into());
        }

        let (model_id, model_inner_id) = self.status.ids_for(style_id)?;

        let phoneme_vector_array = NdArray::new(ndarray::arr1(phoneme_vector));
        let speaker_id_array = NdArray::new(ndarray::arr1(&[model_inner_id.raw_id().into()]));

        let mut output = self
            .status
            .predict_duration_session_run(&model_id, phoneme_vector_array, speaker_id_array)
            .await?;

        for output_item in output.iter_mut() {
            if *output_item < PHONEME_LENGTH_MINIMAL {
                *output_item = PHONEME_LENGTH_MINIMAL;
            }
        }

        Ok(output)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn predict_intonation(
        &self,
        length: usize,
        vowel_phoneme_vector: &[i64],
        consonant_phoneme_vector: &[i64],
        start_accent_vector: &[i64],
        end_accent_vector: &[i64],
        start_accent_phrase_vector: &[i64],
        end_accent_phrase_vector: &[i64],
        style_id: StyleId,
    ) -> Result<Vec<f32>> {
        if !self.status.validate_speaker_id(style_id) {
            return Err(ErrorRepr::StyleNotFound { style_id }.into());
        }

        let (model_id, model_inner_id) = self.status.ids_for(style_id)?;

        let length_array = NdArray::new(ndarray::arr0(length as i64));
        let vowel_phoneme_vector_array = NdArray::new(ndarray::arr1(vowel_phoneme_vector));
        let consonant_phoneme_vector_array = NdArray::new(ndarray::arr1(consonant_phoneme_vector));
        let start_accent_vector_array = NdArray::new(ndarray::arr1(start_accent_vector));
        let end_accent_vector_array = NdArray::new(ndarray::arr1(end_accent_vector));
        let start_accent_phrase_vector_array =
            NdArray::new(ndarray::arr1(start_accent_phrase_vector));
        let end_accent_phrase_vector_array = NdArray::new(ndarray::arr1(end_accent_phrase_vector));
        let speaker_id_array = NdArray::new(ndarray::arr1(&[model_inner_id.raw_id().into()]));

        self.status
            .predict_intonation_session_run(
                &model_id,
                length_array,
                vowel_phoneme_vector_array,
                consonant_phoneme_vector_array,
                start_accent_vector_array,
                end_accent_vector_array,
                start_accent_phrase_vector_array,
                end_accent_phrase_vector_array,
                speaker_id_array,
            )
            .await
    }

    pub async fn decode(
        &self,
        length: usize,
        phoneme_size: usize,
        f0: &[f32],
        phoneme_vector: &[f32],
        style_id: StyleId,
    ) -> Result<Vec<f32>> {
        if !self.status.validate_speaker_id(style_id) {
            return Err(ErrorRepr::StyleNotFound { style_id }.into());
        }

        let (model_id, model_inner_id) = self.status.ids_for(style_id)?;

        // 音が途切れてしまうのを避けるworkaround処理が入っている
        // TODO: 改善したらここのpadding処理を取り除く
        const PADDING_SIZE: f64 = 0.4;
        const DEFAULT_SAMPLING_RATE: f64 = 24000.0;
        let padding_size = ((PADDING_SIZE * DEFAULT_SAMPLING_RATE) / 256.0).round() as usize;
        let start_and_end_padding_size = 2 * padding_size;
        let length_with_padding = length + start_and_end_padding_size;
        let f0_with_padding = Self::make_f0_with_padding(f0, length_with_padding, padding_size);

        let phoneme_with_padding = Self::make_phoneme_with_padding(
            phoneme_vector,
            phoneme_size,
            length_with_padding,
            padding_size,
        );

        let f0_array = NdArray::new(
            ndarray::arr1(&f0_with_padding)
                .into_shape([length_with_padding, 1])
                .unwrap(),
        );
        let phoneme_array = NdArray::new(
            ndarray::arr1(&phoneme_with_padding)
                .into_shape([length_with_padding, phoneme_size])
                .unwrap(),
        );
        let speaker_id_array = NdArray::new(ndarray::arr1(&[model_inner_id.raw_id().into()]));

        self.status
            .decode_session_run(&model_id, f0_array, phoneme_array, speaker_id_array)
            .await
            .map(|output| Self::trim_padding_from_output(output, padding_size))
    }

    fn make_f0_with_padding(
        f0_slice: &[f32],
        length_with_padding: usize,
        padding_size: usize,
    ) -> Vec<f32> {
        // 音が途切れてしまうのを避けるworkaround処理
        // 改善したらこの関数を削除する
        let mut f0_with_padding = Vec::with_capacity(length_with_padding);
        let padding = vec![0.0; padding_size];
        f0_with_padding.extend_from_slice(&padding);
        f0_with_padding.extend_from_slice(f0_slice);
        f0_with_padding.extend_from_slice(&padding);
        f0_with_padding
    }

    fn make_phoneme_with_padding(
        phoneme_slice: &[f32],
        phoneme_size: usize,
        length_with_padding: usize,
        padding_size: usize,
    ) -> Vec<f32> {
        // 音が途切れてしまうのを避けるworkaround処理
        // 改善したらこの関数を削除する
        let mut padding_phoneme = vec![0.0; phoneme_size];
        padding_phoneme[0] = 1.0;
        let padding_phoneme_len = padding_phoneme.len();
        let padding_phonemes: Vec<f32> = padding_phoneme
            .into_iter()
            .cycle()
            .take(padding_phoneme_len * padding_size)
            .collect();
        let mut phoneme_with_padding = Vec::with_capacity(phoneme_size * length_with_padding);
        phoneme_with_padding.extend_from_slice(&padding_phonemes);
        phoneme_with_padding.extend_from_slice(phoneme_slice);
        phoneme_with_padding.extend_from_slice(&padding_phonemes);

        phoneme_with_padding
    }

    fn trim_padding_from_output(mut output: Vec<f32>, padding_f0_size: usize) -> Vec<f32> {
        // 音が途切れてしまうのを避けるworkaround処理
        // 改善したらこの関数を削除する
        let padding_sampling_size = padding_f0_size * 256;
        output
            .drain(padding_sampling_size..output.len() - padding_sampling_size)
            .collect()
    }
}
